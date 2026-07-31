use yew::prelude::*;
use crate::ui::Icon;

#[derive(Properties, PartialEq)]
pub struct PrincipleCardProps {
    pub icon_name: &'static str,
    pub title: &'static str,
    pub description: &'static str,
}

#[function_component(PrincipleCard)]
pub fn principle_card(props: &PrincipleCardProps) -> Html {
    html! {
        <div class="card-lift p-6 rounded-2xl border border-ink-200 dark:border-ink-800 bg-white dark:bg-ink-900/50 hover:border-rust-500/50">
            <div class="w-12 h-12 rounded-xl bg-rust-500/10 text-rust-500 flex items-center justify-center mb-4">
                <Icon name={props.icon_name} size="22" color="currentColor" stroke_width="2" />
            </div>
            <h3 class="font-semibold text-lg mb-2">{ &props.title }</h3>
            <p class="text-sm text-ink-600 dark:text-ink-300 leading-relaxed">{ &props.description }</p>
        </div>
    }
}