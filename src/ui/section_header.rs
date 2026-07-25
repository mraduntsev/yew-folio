use yew::prelude::*;

use crate::{content::SectionDescription, ui::Icon};

#[derive(Clone, Properties, PartialEq)]
pub struct SectionHeaderProps {
    pub number: String,
    pub title: String,
    #[prop_or_default]
    pub description: Option<SectionDescription>,
    #[prop_or(false)]
    pub centered: bool,
}

fn render_description(desc: &SectionDescription, centered: bool) -> Html {
    match desc {
        SectionDescription::Text(text) => {
            let class = if centered {
                "text-ink-600 dark:text-ink-300 max-w-xl mx-auto"
            } else {
                "max-w-md text-ink-600 dark:text-ink-300"
            };
            html! {
                <p class={class}>{ text }</p>
            }
        }
        SectionDescription::Link { label, href } => html! {
            <div class="reveal flex justify-end mb-8 -mt-8">
                <a
                    href={href.clone()}
                    target="_blank"
                    class="font-mono text-sm text-ink-500 hover:text-rust-500 transition flex items-center gap-1"
                >
                    { label }
                    <Icon name="external-link" size="14" color="currentColor" stroke_width="2" />
                </a>
            </div>
        },
    }
}

#[function_component(SectionHeader)]
pub fn section_header(props: &SectionHeaderProps) -> Html {
    let description = props
        .description
        .as_ref()
        .map(|desc| render_description(desc, props.centered));

    if props.centered {
        html! {
            <div class="reveal text-center mb-12">
                <div class="font-mono text-sm text-rust-500 mb-2">{ &props.number }</div>
                <h2 class="text-4xl md:text-5xl font-bold tracking-tight mb-4">{ &props.title }</h2>
                { for description }
            </div>
        }
    } else {
        html! {
            <div class="reveal flex items-end justify-between mb-12 flex-wrap gap-4">
                <div>
                    <div class="font-mono text-sm text-rust-500 mb-2">{ &props.number }</div>
                    <h2 class="text-4xl md:text-5xl font-bold tracking-tight">{ &props.title }</h2>
                </div>
                { for description }
            </div>
        }
    }
}
