use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct ProjectCardProps {
    pub title: &'static str,
    pub description: &'static str,
    pub tags: &'static [&'static str],
    pub stars: u32,
    pub forks: u32,
    pub gradient: &'static str,
    pub version: &'static str,
    pub featured: bool,
    pub repo_url: Option<&'static str>,
}

#[function_component(ProjectCard)]
pub fn project_card(props: &ProjectCardProps) -> Html {
    let gradient_classes = format!("h-40 bg-gradient-to-br {} relative overflow-hidden", props.gradient);
    let stars_str = if props.stars >= 1000 {
        format!("{:.1}k", props.stars as f64 / 1000.0)
    } else {
        props.stars.to_string()
    };
    let forks_str = props.forks.to_string();

    html! {
        <article class="reveal card-lift group relative rounded-2xl border border-ink-200 dark:border-ink-800 bg-white dark:bg-ink-900/60 overflow-hidden">
            <div class={gradient_classes}>
                <div class="absolute inset-0 grid-bg opacity-30"></div>
                <div class="absolute bottom-3 left-4 font-mono text-white/80 text-xs">{ &props.version }</div>
                if props.featured {
                    <div class="absolute top-3 right-3 px-2 py-1 rounded-md bg-black/30 backdrop-blur text-white text-xs font-mono">
                        { "featured" }
                    </div>
                }
            </div>
            <div class="p-6">
                <div class="flex items-start justify-between gap-4 mb-2">
                    <h3 class="text-xl font-bold">{ &props.title }</h3>
                    if let Some(url) = &props.repo_url {
                        <a href={url.clone()} target="_blank" class="text-ink-400 group-hover:text-rust-500 group-hover:-translate-y-0.5 group-hover:translate-x-0.5 transition">
                            <svg class="w-4.5 h-4.5" xmlns="http://www.w3.org/2000/svg" width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M7 17L17 7M7 7h10v10"/></svg>
                        </a>
                    } else {
                        <span class="text-ink-400">
                            <svg xmlns="http://www.w3.org/2000/svg" width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M7 17L17 7M7 7h10v10"/></svg>
                        </span>
                    }
                </div>
                <p class="text-sm text-ink-600 dark:text-ink-300 mb-4 leading-relaxed">{ &props.description }</p>
                <div class="flex flex-wrap gap-2 mb-4">
                    { for props.tags.iter().map(|tag| html! {
                        <span class="px-2 py-1 rounded-md bg-ink-100 dark:bg-ink-800 text-xs font-mono">{ tag }</span>
                    }) }
                </div>
                <div class="flex items-center gap-4 text-xs text-ink-500 font-mono">
                    <span class="flex items-center gap-1">
                        <svg xmlns="http://www.w3.org/2000/svg" width="12" height="12" viewBox="0 0 24 24" fill="currentColor"><path d="M12 .5C5.7.5.5 5.7.5 12c0 5.1 3.3 9.4 7.8 10.9.6.1.8-.2.8-.6v-2c-3.2.7-3.9-1.5-3.9-1.5-.5-1.3-1.3-1.7-1.3-1.7-1.1-.7.1-.7.1-.7 1.2.1 1.8 1.2 1.8 1.2 1.1 1.8 2.8 1.3 3.5 1 .1-.8.4-1.3.8-1.6-2.6-.3-5.3-1.3-5.3-5.7 0-1.3.5-2.3 1.2-3.1-.1-.3-.5-1.5.1-3.1 0 0 1-.3 3.3 1.2 1-.3 2-.4 3-.4s2 .1 3 .4c2.3-1.5 3.3-1.2 3.3-1.2.7 1.6.2 2.8.1 3.1.8.8 1.2 1.9 1.2 3.1 0 4.4-2.7 5.4-5.3 5.7.4.4.8 1.1.8 2.2v3.3c0 .3.2.7.8.6 4.5-1.5 7.8-5.8 7.8-10.9C23.5 5.7 18.3.5 12 .5z"/></svg>
                        { stars_str }
                    </span>
                    <span class="flex items-center gap-1">
                        <svg xmlns="http://www.w3.org/2000/svg" width="12" height="12" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M6 3v12M18 9v12M6 15c0 1 1 2 6 2s6-1 6-2M6 3c0 1 1 2 6 2s6-1 6-2M18 3c0 1-1 2-6 2s-6-1-6-2"/></svg>
                        { forks_str }
                    </span>
                </div>
            </div>
        </article>
    }
}