use yew::prelude::*;

use crate::{
    components::{EducationCard, TimelineItem},
    content::load_locale,
    hooks::use_language,
    ui::SectionHeader,
};

#[function_component(Education)]
pub fn education() -> Html {
    let lang = use_language().current;
    let education = load_locale(lang).education;

    html! {
        <section id="education" class="py-24 relative bg-ink-100/50 dark:bg-ink-900/30">
            <div class="max-w-6xl mx-auto px-6">
                <SectionHeader
                    number={education.number}
                    title={education.title}
                    description={education.description}
                    centered={education.centered}
                />

                <div class="grid lg:grid-cols-5 gap-8">
                    <div class="lg:col-span-2 reveal">
                        <EducationCard entries={education.entries} />
                    </div>

                    <div class="lg:col-span-3 reveal">
                        <div class="flex items-center justify-between mb-6">
                            <h3 class="text-xl font-bold">{"Courses & certifications"}</h3>
                            <span class="text-xs font-mono text-ink-500">
                                { format!("{} completed", education.courses.len()) }
                            </span>
                        </div>

                        <div class="timeline relative space-y-6">
                            {
                                education.courses.iter().map(|c| html! {
                                    <TimelineItem
                                        key={format!("{}-{}", c.year, c.title)}
                                        year={c.year.clone()}
                                        hours={c.hours.clone()}
                                        title={c.title.clone()}
                                        provider={c.provider.clone()}
                                        description={c.description.clone()}
                                        tags={c.tags.clone()}
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
