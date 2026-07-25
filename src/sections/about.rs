use yew::prelude::*;

use crate::{
    content::load_locale,
    hooks::use_language,
    ui::{PrincipleCard, SectionHeader, SkillBadge},
};

#[function_component(About)]
pub fn about() -> Html {
    let lang = use_language().current;
    let about = load_locale(lang).about;
    html! {
        <section id="about" class="py-24 relative">
            <div class="max-w-6xl mx-auto px-6">
                <SectionHeader
                    number={about.number}
                    title={about.title}
                    description={about.description}
                    centered={about.centered}
                />

                <div class="grid grid-cols-1 md:grid-cols-3 gap-6">
                    {
                        about.principles.iter().map(|p| html! {
                            <PrincipleCard
                                key={p.title.clone()}
                                icon_name={p.icon.clone()}
                                title={p.title.clone()}
                                description={p.description.clone()}
                            />
                        }).collect::<Html>()
                    }
                </div>

                <div class="reveal mt-16">
                    <h3 class="font-mono text-sm text-ink-500 mb-6">{"// tech stack"}</h3>
                    <div class="flex flex-wrap gap-3">
                        {
                            about.skills.iter().map(|skill| {
                                html! {
                                    <SkillBadge
                                        key={format!("{}-{}", skill.letter.clone(), skill.title.clone())}
                                        gradient={skill.gradient.clone()}
                                        letter={skill.letter.clone()}
                                        title={skill.title.clone()}
                                        technologies={skill.technologies.clone()}
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
