use yew::prelude::*;
use crate::hooks::use_theme;
use crate::ui::Icon;

#[function_component(ThemeToggle)]
pub fn theme_toggle() -> Html {
    let theme = use_theme();
    let toggle = {
        let theme = theme.clone();
        Callback::from(move |_: MouseEvent| theme.toggle())
    };

    html! {
        <button
            onclick={toggle}
            aria-label="Toggle theme"
            class="w-10 h-10 rounded-lg border border-ink-200 dark:border-ink-800 hover:border-rust-500 hover:text-rust-500 transition flex items-center justify-center"
        >
            <Icon name="sun" size="18" />
            <Icon name="moon" size="18" />
        </button>
    }
}