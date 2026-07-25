use std::{cell::RefCell, rc::Rc};

use gloo_timers::callback::Timeout;
use yew::prelude::*;

struct TypeState {
    pub word_index: usize,
    pub char_index: usize,
    pub is_deleting: bool,
}
#[derive(Properties, PartialEq)]
pub struct TypewriterProps {
    pub words: Vec<String>,
    #[prop_or(90)]
    pub type_speed: u32,
    #[prop_or(40)]
    pub delete_speed: u32,
    #[prop_or(1500)]
    pub pause_duration: u32,
    #[prop_or_default]
    pub class: &'static str,
}

#[function_component(Typewriter)]
pub fn typewriter(props: &TypewriterProps) -> Html {
    let displayed_text = use_state(String::new);

    {
        let words = props.words.clone();
        let displayed_text = displayed_text.clone();
        let type_speed = props.type_speed;
        let delete_speed = props.delete_speed;
        let pause_duration = props.pause_duration;

        use_effect_with(props.words.clone(), move |_| {
            let state = Rc::new(RefCell::new(TypeState {
                word_index: 0,
                char_index: 0,
                is_deleting: false,
            }));

            let timeout_handle: Rc<RefCell<Option<Timeout>>> = Rc::new(RefCell::new(None));

            let tick: Rc<RefCell<Option<Rc<dyn Fn()>>>> = Rc::new(RefCell::new(None));

            {
                let tick_ref = tick.clone();
                let timeout_handle = timeout_handle.clone();
                let state = state.clone();

                let tick_fn: Rc<dyn Fn()> = Rc::new(move || {
                    if words.is_empty() {
                        return;
                    }

                    let mut s = state.borrow_mut();
                    let current_word = &words[s.word_index];
                    let mut next_delay = type_speed;

                    if !s.is_deleting {
                        s.char_index += 1;
                        let total_chars = current_word.chars().count();
                        if s.char_index >= total_chars {
                            s.char_index = total_chars;
                            s.is_deleting = true;
                            next_delay = pause_duration;
                        }
                    } else {
                        if s.char_index > 0 {
                            s.char_index -= 1;
                        }
                        if s.char_index == 0 {
                            s.is_deleting = false;
                            s.word_index = (s.word_index + 1) % words.len();
                        }
                        next_delay = delete_speed;
                    }

                    let display_word = &words[s.word_index];
                    let display_text_value: String =
                        display_word.chars().take(s.char_index).collect();

                    displayed_text.set(display_text_value);
                    drop(s);

                    let tick_next = tick_ref.clone();
                    let timeout = Timeout::new(next_delay, move || {
                        if let Some(f) = tick_next.borrow().as_ref() {
                            f();
                        }
                    });
                    *timeout_handle.borrow_mut() = Some(timeout);
                });

                *tick.borrow_mut() = Some(tick_fn);
            }

            if let Some(f) = tick.borrow().as_ref() {
                f();
            }

            move || {
                *timeout_handle.borrow_mut() = None;
            }
        });
    }

    html! {
        <span class={classes!("font-mono", props.class)}>
            { (*displayed_text).clone() }
            <span class="cursor"></span>
        </span>
    }
}
