use yew::prelude::*;

use crate::{
    content::load_locale,
    hooks::use_language,
    ui::{Button, LanguageSwitcher, NavLinkItem, ThemeToggle},
};

#[derive(Properties, PartialEq)]
pub struct NavbarProps {
    pub active_section: Option<String>,
}

#[function_component(Navbar)]
pub fn navbar(props: &NavbarProps) -> Html {
    let lang = use_language().current;
    let nav_links = load_locale(lang).navbar;
    let _active = &props.active_section;

    html! {
        <header id="navbar" class="fixed top-0 inset-x-0 z-50 backdrop-blur-md bg-ink-50/70 dark:bg-ink-950/70 border-b border-ink-200/60 dark:border-ink-800/60">
            <nav class="max-w-6xl mx-auto px-6 h-16 flex items-center justify-between">
                <a href="#hero" class="flex items-center gap-2 group">
                    <div class="w-9 h-9 rounded-lg bg-gradient-to-br from-rust-500 to-rust-700 flex items-center justify-center text-white font-mono font-bold shadow-lg shadow-rust-500/20">
                        { "</>" }
                    </div>
                    <span class="font-mono font-semibold tracking-tight">
                        { "maxim" }<span class="text-rust-500">{ ".dev" }</span>
                    </span>
                </a>

                <ul class="hidden md:flex items-center gap-8 text-sm font-medium">
                    { for nav_links.nav.iter().map(|link| html! {
                        <NavLinkItem
                            key={link.href.clone()}
                            href={link.href.clone()}
                            label={link.label.clone()} />
                    }) }
                </ul>

                <div class="flex items-center gap-3">
                    <LanguageSwitcher/>
                    <ThemeToggle />
                    <Button variant="outline" size_hidden="hidden lg:flex" href="assets/resume.pdf" left_icon="download">{ &nav_links.buttons.outline }</Button>
                    <Button variant="primary" size_hidden="hidden lg:flex" href="#contact" right_icon="arrow-right">{ &nav_links.buttons.primary }</Button>
                </div>
            </nav>
        </header>
    }
}
