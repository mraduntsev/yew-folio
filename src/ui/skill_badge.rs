use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct SkillBadgeProps {
    pub gradient: String,
    pub letter: String,
    pub title: String,
    pub technologies: String,
}

#[function_component(SkillBadge)]
pub fn skill_badge(props: &SkillBadgeProps) -> Html {
    let icon_class = format!(
        "w-8 h-8 rounded-md bg-gradient-to-br {} flex items-center justify-center text-white font-mono font-bold text-xs",
        props.gradient.clone()
    );

    html! {
        <div class="group flex items-center gap-3 px-4 py-3 rounded-xl border border-ink-200 dark:border-ink-800 bg-white dark:bg-ink-900/50 hover:border-rust-500 transition">
            <div class={icon_class}>{ &props.letter }</div>
            <div>
                <div class="font-semibold text-sm">{ &props.title }</div>
                <div class="text-xs text-ink-500">{ &props.technologies }</div>
            </div>
        </div>
    }
}
