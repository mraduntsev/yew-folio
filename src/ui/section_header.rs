use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct SectionHeaderProps {
    pub number: &'static str,
    pub title: &'static str,
    #[prop_or_default]
    pub description: Option<&'static str>,
    #[prop_or(false)]
    pub centered: bool,
}

#[function_component(SectionHeader)]
pub fn section_header(props: &SectionHeaderProps) -> Html {
    if props.centered {
        html! {
            <div class="reveal text-center mb-12">
                <div class="font-mono text-sm text-rust-500 mb-2">{ &props.number }</div>
                <h2 class="text-4xl md:text-5xl font-bold tracking-tight mb-4">{ &props.title }</h2>
                if let Some(desc) = &props.description {
                    <p class="text-ink-600 dark:text-ink-300 max-w-xl mx-auto">{ desc }</p>
                }
            </div>
        }
    } else {
        html! {
            <div class="reveal flex items-end justify-between mb-12 flex-wrap gap-4">
                <div>
                    <div class="font-mono text-sm text-rust-500 mb-2">{ &props.number }</div>
                    <h2 class="text-4xl md:text-5xl font-bold tracking-tight">{ &props.title }</h2>
                </div>
                if let Some(desc) = &props.description {
                    <p class="max-w-md text-ink-600 dark:text-ink-300">{ desc }</p>
                }
            </div>
        }
    }
}