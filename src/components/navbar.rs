use yew::prelude::*;
use crate::content::NAV_LINKS;
use crate::ui::{NavLinkItem, ThemeToggle, Button};

#[function_component(Navbar)]
pub fn navbar() -> Html {
    html! {
        <header id="navbar" class="fixed top-0 inset-x-0 z-50 backdrop-blur-md bg-ink-50/70 dark:bg-ink-950/70 border-b border-ink-200/60 dark:border-ink-800/60">
            <nav class="max-w-6xl mx-auto px-6 h-16 flex items-center justify-between">
                <a href="#hero" class="flex items-center gap-2 group">
                    <div class="w-9 h-9 rounded-lg bg-gradient-to-br from-rust-500 to-rust-700 flex items-center justify-center text-white font-mono font-bold shadow-lg shadow-rust-500/20">
                        { "</>" }
                    </div>
                    <span class="font-mono font-semibold tracking-tight">
                        { "raduntsev" }<span class="text-rust-500">{ ".dev" }</span>
                    </span>
                </a>

                <ul class="hidden md:flex items-center gap-8 text-sm font-medium">
                    { for NAV_LINKS.iter().map(|(label, href)| html! {
                        <NavLinkItem href={*href} label={*label} />
                    }) }
                </ul>

                <div class="flex items-center gap-3">
                    <ThemeToggle />
                    <Button variant="outline" href="/resume.pdf" left_icon="download">{ "Resume" }</Button>
                    <Button variant="primary" href="#contact" right_icon="arrow-right">{ "Hire me" }</Button>
                </div>
            </nav>
        </header>
    }
}