use yew::prelude::*;
use crate::ui::{PrincipleCard, SectionHeader, SkillBadge};
use crate::content::{ABOUT_SECTION, PRINCIPLES, SKILLS};

#[function_component(About)]
pub fn about() -> Html {
    html! {
        <section id="about" class="py-24 relative">
            <div class="max-w-6xl mx-auto px-6">
                <SectionHeader
                    number={ABOUT_SECTION.number}
                    title={ABOUT_SECTION.title}
                    description={ABOUT_SECTION.description}
                    centered={ABOUT_SECTION.centered}
                />

                <div class="grid grid-cols-1 md:grid-cols-3 gap-6">
                    {
                        PRINCIPLES.iter().map(|p| html! {
                            <PrincipleCard
                                key={p.title}
                                icon_name={p.icon}
                                title={p.title}
                                description={p.description}
                            />
                        }).collect::<Html>()
                    }
                </div>

                <div class="reveal mt-16">
                    <h3 class="font-mono text-sm text-ink-500 mb-6">{"// tech stack"}</h3>
                    <div class="flex flex-wrap gap-3">
                        {
                            SKILLS.iter().map(|skill| {
                                html! {
                                    <SkillBadge
                                        key={skill.title}
                                        gradient={skill.gradient}
                                        letter={skill.letter}
                                        title={skill.title}
                                        technologies={skill.technologies}
                                    />
                                }
                            }).collect::<Html>()
                        }
                    </div>
                </div>
            </div>
        </section>
    }
}