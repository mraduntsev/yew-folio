use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct TimelineItemProps {
    pub year: &'static str,
    pub hours: &'static str,
    pub title: &'static str,
    pub provider: &'static str,
    pub description: &'static str,
    pub tags: Vec<&'static str>,
}

#[function_component(TimelineItem)]
pub fn timeline_item(props: &TimelineItemProps) -> Html {
    html! {
        <div class="relative pl-12 md:pl-0 md:grid md:grid-cols-2 md:gap-10">
            <div class="timeline-dot"></div>
            <div class="md:text-right md:pr-10">
                <div class="inline-flex items-center gap-2 mb-1">
                    <span class="text-xs font-mono text-rust-500">{ &props.year }</span>
                    <span class="text-ink-300">{"·"}</span>
                    <span class="text-xs font-mono text-ink-500">{ &props.hours }</span>
                </div>
                <h4 class="font-semibold">{ &props.title }</h4>
                <p class="text-xs text-ink-500 font-mono mb-1">{ &props.provider }</p>
            </div>
            <div class="md:pl-10">
                <p class="text-sm text-ink-600 dark:text-ink-300 leading-relaxed">{ &props.description }</p>
                <div class="flex flex-wrap gap-1.5 mt-2 md:justify-start">
                    { for props.tags.iter().map(|tag| {
                        html! { <span class="px-2 py-0.5 rounded-md bg-rust-500/10 text-rust-500 text-xs font-mono">{ tag }</span> }
                    }) }
                </div>
            </div>
        </div>
    }
}