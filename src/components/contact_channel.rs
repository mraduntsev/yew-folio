use yew::prelude::*;

use crate::ui::Icon;

#[derive(Properties, PartialEq)]
pub struct ContactChannelProps {
    pub label: String,
    pub value: String,
    pub icon_name: String,
    pub url: Option<String>,
    pub description: String,
}

#[function_component(ContactChannel)]
pub fn contact_channel(props: &ContactChannelProps) -> Html {
    html! {
        <a href={props.url.clone()} target="_blank" rel="noopener"
           class="card-lift group flex flex-col items-start gap-3 p-5 rounded-2xl border border-ink-200 dark:border-ink-800 bg-white dark:bg-ink-900/50 hover:border-rust-500">
            <div class="w-11 h-11 rounded-xl bg-rust-500/10 text-rust-500 flex items-center justify-center group-hover:bg-rust-500 group-hover:text-white transition">
                <Icon name={props.icon_name.clone()} size="20" color="currentColor" stroke_width="2" />
            </div>
            <div>
                <div class="text-xs text-ink-500 font-mono mb-0.5">{ &props.label }</div>
                <div class="font-semibold text-sm">{ &props.value }</div>
                <div class="text-xs text-ink-500 mt-1">{ &props.description }</div>
            </div>
        </a>
    }
}
