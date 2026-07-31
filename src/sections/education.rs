use yew::prelude::*;
use crate::ui::SectionHeader;
use crate::components::{EducationCard, TimelineItem};
use crate::content::{COURSES, EDUCATION_ENTRIES, EDUCATION_SECTION};

#[function_component(Education)]
pub fn education() -> Html {

    html! {
        <section id="education" class="py-24 relative bg-ink-100/50 dark:bg-ink-900/30">
            <div class="max-w-6xl mx-auto px-6">
                <SectionHeader
                    number={EDUCATION_SECTION.number}
                    title={EDUCATION_SECTION.title}
                    description={EDUCATION_SECTION.description}
                    centered={EDUCATION_SECTION.centered}
                />

                <div class="grid lg:grid-cols-5 gap-8">
                    <div class="lg:col-span-2 reveal">
                        <EducationCard entries={EDUCATION_ENTRIES} />
                    </div>

                    <div class="lg:col-span-3 reveal">
                        <div class="flex items-center justify-between mb-6">
                            <h3 class="text-xl font-bold">{"Courses & certifications"}</h3>
                            <span class="text-xs font-mono text-ink-500">
                                { format!("{} completed", COURSES.len()) }
                            </span>
                        </div>

                        <div class="timeline relative space-y-6">
                            {
                                COURSES.iter().map(|c| html! {
                                    <TimelineItem
                                        key={format!("{}-{}", c.year, c.title)}
                                        year={c.year}
                                        hours={c.hours}
                                        title={c.title}
                                        provider={c.provider}
                                        description={c.description}
                                        tags={c.tags}
                                    />
                                }).collect::<Html>()
                            }
                        </div>
                    </div>
                </div>
            </div>
        </section>
    }
}