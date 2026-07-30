use yew::prelude::*;
use crate::ui::Icon;
use crate::content::EducationEntry;

#[derive(Properties, PartialEq)]
pub struct EducationCardProps {
    pub entries: &'static [EducationEntry],
}

#[function_component(EducationCard)]
pub fn education_card(props: &EducationCardProps) -> Html {
    let (latest, previous) = props.entries.split_first().expect("EducationCard requires at least one entry");

    html! {
        <div class="card-lift relative rounded-2xl border border-ink-200 dark:border-ink-800 bg-white dark:bg-ink-900/60 overflow-hidden h-full">
            <div class="h-32 bg-gradient-to-br from-rust-500 via-rust-600 to-rust-800 relative overflow-hidden">
                <div class="absolute inset-0 grid-bg opacity-30"></div>
                <div class="absolute top-4 left-4 w-14 h-14 rounded-xl bg-white/15 backdrop-blur border border-white/20 flex items-center justify-center">
                    <Icon name="book-open" size=28 color="white" stroke_width="2" />
                </div>
                <div class="absolute bottom-3 right-4 font-mono text-white/70 text-xs">{ &latest.years }</div>
            </div>

            <div class="p-6">
                <div class="inline-flex items-center gap-1.5 px-2 py-0.5 rounded-md bg-rust-500/10 text-rust-500 text-xs font-mono mb-3">
                    { &latest.badge }
                </div>
                <h3 class="text-xl font-bold mb-1">{ &latest.degree }</h3>
                <p class="text-sm text-ink-500 dark:text-ink-400 mb-4">{ &latest.university }</p>

                <p class="text-sm text-ink-600 dark:text-ink-300 leading-relaxed mb-5">
                    { &latest.description }
                </p>

                <div class="space-y-2.5 text-sm">
                    { for latest.highlights.iter().map(|h| {
                        html! {
                            <div class="flex items-start gap-2.5">
                                <div class="text-rust-500 mt-0.5 flex-shrink-0">
                                    <Icon name="check" size=14 color="currentColor" stroke_width="2.5" />
                                </div>
                                <span class="text-ink-600 dark:text-ink-300">{ h }</span>
                            </div>
                        }
                    }) }
                </div>

                { if !previous.is_empty() {
                    html! {
                            <div class="mt-6 pt-5 border-t border-ink-200 dark:border-ink-800">
                                <div class="text-xs font-mono text-ink-500 mb-4">{"also"}</div>
                                <div class="space-y-5">
                                    { for previous.iter().map(|prev| {
                                        html! {
                                            <div class="pl-4 border-l-2 border-rust-500/30">
                                                <div class="flex flex-wrap items-center gap-2 mb-1">
                                                    <span class="px-2 py-0.5 rounded-md bg-ink-100 dark:bg-ink-800 text-ink-600 dark:text-ink-300 text-xs font-mono">
                                                        { &prev.badge }
                                                    </span>
                                                    <span class="text-xs text-ink-400">{ &prev.years }</span>
                                                </div>
                                                <h4 class="font-semibold text-sm">{ &prev.degree }</h4>
                                                <p class="text-xs text-ink-500 mt-0.5 mb-2">{ &prev.university }</p>
                                                if !prev.description.is_empty() {
                                                    <p class="text-xs text-ink-600 dark:text-ink-300 leading-relaxed mb-2">
                                                        { &prev.description }
                                                    </p>
                                                }
                                                if !prev.highlights.is_empty() {
                                                    <div class="flex flex-wrap gap-1.5 mt-2">
                                                        { for prev.highlights.iter().map(|h| {
                                                            html! {
                                                                <span class="px-2 py-0.5 rounded-md bg-rust-500/10 text-rust-500 text-xs font-mono">
                                                                    { h }
                                                                </span>
                                                            }
                                                        }) }
                                                    </div>
                                                }
                                            </div>
                                        }
                                    }) }
                                </div>
                            </div>
                        }
                } else {
                    html! {}
                }}
            </div>
        </div>
    }
}