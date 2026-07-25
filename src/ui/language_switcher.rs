use yew::prelude::*;

use crate::hooks::{Language, use_language};

#[function_component(LanguageSwitcher)]
pub fn language_switcher() -> Html {
    let lang = use_language();
    let current = lang.current;
    let set_lang = lang.setter.clone();

    let on_en = {
        let set_lang = set_lang.clone();
        Callback::from(move |_| set_lang.set(Language::En))
    };

    let on_ru = {
        let set_lang = set_lang.clone();
        Callback::from(move |_| set_lang.set(Language::Ru))
    };

    let (en_class, ru_class, pill_style) = match current {
        Language::En => (
            "text-rust-500",
            "text-ink-400 dark:text-ink-500 hover:text-ink-700 dark:hover:text-ink-200",
            "left: 4px;",
        ),
        Language::Ru => (
            "text-ink-400 dark:text-ink-500 hover:text-ink-700 dark:hover:text-ink-200",
            "text-rust-500",
            "left: calc(50% - 4px);",
        ),
    };

    html! {
        <div class="relative flex items-center h-10 rounded-lg border border-ink-200 dark:border-ink-800 bg-white/60 dark:bg-ink-900/60 backdrop-blur overflow-hidden">
            <div
                class="absolute top-1 bottom-1 w-[calc(50%-4px)] rounded-md bg-rust-500/10 border border-rust-500/20 transition-all duration-300 ease-out pointer-events-none"
                style={pill_style}
            />
            <button
                type="button"
                class={classes!("relative", "z-10", "px-3", "h-full", "text-xs", "font-mono", "font-semibold", "tracking-wide", "transition-colors", en_class)}
                onclick={on_en}
            >
                {"EN"}
            </button>
            <button
                type="button"
                class={classes!("relative", "z-10", "px-3", "h-full", "text-xs", "font-mono", "font-semibold", "tracking-wide", "transition-colors", ru_class)}
                onclick={on_ru}
            >
                {"RU"}
            </button>
        </div>
    }
}
