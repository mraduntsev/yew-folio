use yew::prelude::*;

use crate::{
    components::ProjectCard, content::load_locale, hooks::use_language, ui::SectionHeader,
};

#[function_component(Projects)]
pub fn projects() -> Html {
    let lang = use_language().current;
    let projects = load_locale(lang).projects;
    //
    html! {
        <section id="projects" class="py-24 relative">
            <div class="max-w-6xl mx-auto px-6">
                <SectionHeader
                    number={projects.number}
                    title={projects.title}
                    description={projects.description}
                    centered={projects.centered}
                />
                <div class="grid md:grid-cols-2 gap-6">
                    { for projects.projects.into_iter().map(|p| {
                        let key = format!("{}-{}", p.version, p.title);
                        html! {
                            <ProjectCard
                                key={key}
                                title={p.title}
                                description={p.description}
                                tags={p.tags}
                                gradient={p.gradient}
                                version={p.version}
                                stars={p.stars}
                                forks={p.forks}
                                featured={p.featured}
                                repo_url={p.repo_url}
                            />
                        }
                    }) }
                </div>
            </div>
        </section>
    }
}
