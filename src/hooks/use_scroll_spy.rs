use wasm_bindgen::{JsCast, JsValue, closure::Closure};
use web_sys::{
    Element, IntersectionObserver, IntersectionObserverEntry, IntersectionObserverInit, window,
};
use yew::prelude::*;

#[derive(Clone, PartialEq, Properties)]
pub struct ScrollSpyProps {
    /// List of CSS section selectors
    pub section_selectors: Vec<&'static str>,
    /// Visibility threshold (0.0–1.0), at which the section is considered active
    pub threshold: f64,
}

/// The hook runs:
/// 1. Reveal animation for all elements with the `.reveal` class
/// 2. Track the active section and return its `id`
#[hook]
pub fn use_scroll_spy(props: &ScrollSpyProps) -> Option<String> {
    let active_section = use_state(|| None::<String>);
    let active_section_setter = active_section.setter();

    // Copy the data from the props so that the closure owns it
    let selectors = props.section_selectors.clone();

    use_effect_with((), move |_| {
        let window = window().expect("no global window");
        let document = window.document().expect("no document");

        // ---------- Observer for reveal animation ----------
        let reveal_observer = {
            let thresholds = js_sys::Array::new();
            thresholds.push(&JsValue::from_f64(0.1));

            let init = IntersectionObserverInit::new();
            init.set_threshold(&thresholds);

            let callback = Closure::wrap(Box::new(
                move |entries: Vec<IntersectionObserverEntry>, _observer: IntersectionObserver| {
                    for entry in entries {
                        if entry.is_intersecting() {
                            let _ = entry.target().class_list().add_1("visible");
                            // For a one-time animation, uncomment:
                            // let _ = _observer.unobserve(&entry.target());
                        }
                    }
                },
            )
                as Box<dyn Fn(Vec<IntersectionObserverEntry>, IntersectionObserver)>);

            let observer =
                IntersectionObserver::new_with_options(callback.as_ref().unchecked_ref(), &init)
                    .expect("failed to create reveal IntersectionObserver");

            callback.forget();
            observer
        };

        // Observe all .reveal elements
        if let Ok(elements) = document.query_selector_all(".reveal") {
            for i in 0..elements.length() {
                if let Some(node) = elements.get(i) {
                    // Bring Node -> Element (dyn_into may be required)
                    if let Ok(el) = node.dyn_into::<Element>() {
                        let _ = reveal_observer.observe(&el);
                    }
                }
            }
        }

        // ---------- Observer for the active section ----------
        let section_setter = active_section_setter.clone();
        let section_observer = {
            let thresholds = js_sys::Array::new();
            thresholds.push(&JsValue::from_f64(0.1));

            let init = IntersectionObserverInit::new();
            init.set_threshold(&thresholds);

            let callback = Closure::wrap(Box::new(
                move |entries: Vec<IntersectionObserverEntry>, _observer: IntersectionObserver| {
                    for entry in entries {
                        if entry.is_intersecting() {
                            let target = entry.target();
                            let id = target.id();
                            if !id.is_empty() {
                                section_setter.set(Some(id));
                            }
                        }
                    }
                },
            )
                as Box<dyn Fn(Vec<IntersectionObserverEntry>, IntersectionObserver)>);

            let observer =
                IntersectionObserver::new_with_options(callback.as_ref().unchecked_ref(), &init)
                    .expect("failed to create section IntersectionObserver");

            callback.forget();
            observer
        };

        // Observe the sections
        for selector in &selectors {
            if let Ok(Some(element)) = document.query_selector(selector) {
                let _ = section_observer.observe(&element);
            }
        }

        // Cleanup when unmounting a component
        move || {
            reveal_observer.disconnect();
            section_observer.disconnect();
        }
    });

    (*active_section).clone()
}
