use gloo_storage::{LocalStorage, Storage};
use wasm_bindgen::JsCast;
use web_sys::{HtmlElement, window};
use yew::prelude::*;

const STORAGE_KEY: &str = "theme";

#[derive(Clone, PartialEq)]
pub struct ThemeHandle {
    pub is_dark: bool,
    toggle_cb: Callback<()>,
}

impl ThemeHandle {
    pub fn toggle(&self) {
        self.toggle_cb.emit(());
    }
}

fn apply_theme_class(is_dark: bool) {
    if let Some(html_el) = window()
        .and_then(|w| w.document())
        .and_then(|d| d.document_element())
        .and_then(|el| el.dyn_into::<HtmlElement>().ok())
    {
        if is_dark {
            let _ = html_el.class_list().add_1("dark");
        } else {
            let _ = html_el.class_list().remove_1("dark");
        }
    }
}

fn initial_theme() -> bool {
    if let Ok(saved) = LocalStorage::get::<String>(STORAGE_KEY) {
        return saved == "dark";
    }
    window()
        .and_then(|w| w.match_media("(prefers-color-scheme: dark)").ok().flatten())
        .map(|mq| mq.matches())
        .unwrap_or(false)
}

#[hook]
pub fn use_theme() -> ThemeHandle {
    let is_dark = use_state(initial_theme);

    {
        let is_dark = *is_dark;
        use_effect_with(is_dark, move |is_dark| {
            apply_theme_class(*is_dark);
            let _ = LocalStorage::set(STORAGE_KEY, if *is_dark { "dark" } else { "light" });
            || ()
        });
    }

    let toggle_cb = {
        let is_dark = is_dark.clone();
        Callback::from(move |_: ()| is_dark.set(!*is_dark))
    };

    ThemeHandle {
        is_dark: *is_dark,
        toggle_cb,
    }
}
