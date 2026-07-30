use yew::prelude::*;
use crate::ui::{Icon, SectionHeader};
use crate::components::{EducationCard, TimelineItem};
use crate::content::{get_education_entries, get_courses};

#[function_component(Education)]
pub fn education() -> Html {
    let formal = get_education_entries();
    let courses = get_courses();

    html! {
        <section id="education" class="py-24 relative bg-ink-100/50 dark:bg-ink-900/30">
            <div class="max-w-6xl mx-auto px-6">
                <SectionHeader
                    number="02 — education"
                    title="Learning path"
                    description={Some("Formal CS background plus a steady diet of deep-dive courses and certifications.".into())}
                    centered={false}
                />

                <div class="grid lg:grid-cols-5 gap-8">
                    <div class="lg:col-span-2 reveal">
                        <EducationCard
                            degree={formal.degree}
                            university={formal.university}
                            years={formal.years}
                            description={formal.description}
                            badge={formal.badge}
                            highlights={formal.highlights.iter().map(|s| s).collect()}
                            additional_degree={formal.additional_degree.map(|s| s)}
                            additional_university={formal.additional_university.map(|s| s)}
                            additional_years={formal.additional_years.map(|s| s)}
                        />
                    </div>

                    // Список курсов
                    <div class="lg:col-span-3 reveal">
                        <div class="flex items-center justify-between mb-6">
                            <h3 class="text-xl font-bold">{"Courses & certifications"}</h3>
                            <span class="text-xs font-mono text-ink-500">
                                { format!("{} completed", courses.len()) }
                            </span>
                        </div>

                        <div class="timeline relative space-y-6">
                            { for courses.into_iter().map(|c| {
                                html! {
                                    <TimelineItem
                                        key={format!("{}-{}", c.year, c.title)}
                                        year={c.year}
                                        hours={c.hours}
                                        title={c.title}
                                        provider={c.provider}
                                        description={c.description}
                                        tags={c.tags.iter().map(|s| s).collect()}
                                    />
                                }
                            }) }
                        </div>

                        <div class="mt-10 flex flex-wrap gap-3">
                            <a href="assets/resume.pdf" class="inline-flex items-center gap-2 px-4 py-2 rounded-lg border border-ink-200 dark:border-ink-800 hover:border-rust-500 hover:text-rust-500 text-sm font-semibold transition">
                                <Icon name="file-text" size=14 color="currentColor" stroke_width="2" />
                                {"Download CV (PDF)"}
                            </a>
                            <a href="#" class="inline-flex items-center gap-2 px-4 py-2 rounded-lg text-sm font-semibold text-ink-500 hover:text-rust-500 transition">
                                {"view all certificates"}
                                <Icon name="arrow-right" size=14 color="currentColor" stroke_width="2" />
                            </a>
                        </div>
                    </div>
                </div>
            </div>
        </section>
    }
}