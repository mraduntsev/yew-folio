use yew::prelude::*;
use crate::ui::Icon;

#[derive(Properties, PartialEq)]
pub struct EducationCardProps {
    pub degree: &'static str,
    pub university: &'static str,
    pub years: &'static str,
    pub description: &'static str,
    pub highlights: Vec<&'static str>,
    pub badge: &'static str,
    pub additional_degree: Option<&'static str>,
    pub additional_university: Option<&'static str>,
    pub additional_years: Option<&'static str>,
}

#[function_component(EducationCard)]
pub fn education_card(props: &EducationCardProps) -> Html {
    html! {
        <div class="card-lift relative rounded-2xl border border-ink-200 dark:border-ink-800 bg-white dark:bg-ink-900/60 overflow-hidden h-full">
            <div class="h-32 bg-gradient-to-br from-rust-500 via-rust-600 to-rust-800 relative overflow-hidden">
                <div class="absolute inset-0 grid-bg opacity-30"></div>
                <div class="absolute top-4 left-4 w-14 h-14 rounded-xl bg-white/15 backdrop-blur border border-white/20 flex items-center justify-center">
                    <Icon name="book-open" size=28 color="white" stroke_width="2" />
                </div>
                <div class="absolute bottom-3 right-4 font-mono text-white/70 text-xs">{ &props.years }</div>
            </div>
            <div class="p-6">
                <div class="inline-flex items-center gap-1.5 px-2 py-0.5 rounded-md bg-rust-500/10 text-rust-500 text-xs font-mono mb-3">
                    { &props.badge }
                </div>
                <h3 class="text-xl font-bold mb-1">{ &props.degree }</h3>
                <p class="text-sm text-ink-500 dark:text-ink-400 mb-4">{ &props.university }</p>

                <p class="text-sm text-ink-600 dark:text-ink-300 leading-relaxed mb-5">
                    { &props.description }
                </p>

                <div class="space-y-2.5 text-sm">
                    { for props.highlights.iter().map(|h| {
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

                { if let Some(deg) = &props.additional_degree {
                    html! {
                        <div class="mt-6 pt-5 border-t border-ink-200 dark:border-ink-800">
                            <div class="text-xs font-mono text-ink-500 mb-2">{"also"}</div>
                            <div class="text-sm font-semibold">{ deg }</div>
                            <div class="text-xs text-ink-500">
                                { format!("{} · {}", props.additional_university.as_deref().unwrap_or(""), props.additional_years.as_deref().unwrap_or("")) }
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