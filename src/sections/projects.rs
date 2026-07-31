use yew::prelude::*;
use crate::ui::SectionHeader;
use crate::components::ProjectCard;
use crate::content::{PROJECTS, PROJECT_SECTION};

#[function_component(Projects)]
pub fn projects() -> Html {
    let projects = PROJECTS;
    // 
    html! {
        <section id="projects" class="py-24 relative">
            <div class="max-w-6xl mx-auto px-6">
                <SectionHeader
                    number={PROJECT_SECTION.number}
                    title={PROJECT_SECTION.title}
                    description={PROJECT_SECTION.description}
                    centered={PROJECT_SECTION.centered}
                />
                <div class="grid md:grid-cols-2 gap-6">
                    { for projects.into_iter().map(|p| {
                        html! {
                            <ProjectCard
                                key={p.title}
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