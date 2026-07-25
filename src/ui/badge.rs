use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct BadgeProps {
    #[prop_or_default]
    pub variant: &'static str,
    #[prop_or_default]
    pub color: &'static str,
    #[prop_or_default]
    pub class: &'static str,
    pub children: Html,
}

#[function_component(Badge)]
pub fn badge(props: &BadgeProps) -> Html {
    if props.variant == "dot" {
        html! {
            <div class={classes!(
                "inline-flex", "items-center", "gap-2", "px-3", "py-1.5",
                "rounded-full", "border", "border-ink-200", "dark:border-ink-800",
                "bg-white/60", "dark:bg-ink-900/60", "backdrop-blur", "text-xs", "font-mono", "bg-emerald-400",
                props.class
            )}>
                <span class="relative flex h-2 w-2">
                    <span class={classes!(
                        "absolute", "inline-flex", "h-full", "w-full",
                        "rounded-full", format!("bg-{}-400", props.color), "opacity-75"
                    )}></span>
                    <span class={classes!(
                        "relative", "inline-flex", "rounded-full", "h-2", "w-2",
                        format!("bg-{}-500", props.color)
                    )}></span>
                </span>
                <span class="text-ink-600 dark:text-ink-300">
                    { props.children.clone() }
                </span>
            </div>
        }
    } else {
        html! {
            <span class={classes!(
                "px-2", "py-1", "rounded-md",
                "bg-ink-100", "dark:bg-ink-800",
                "text-xs", "font-mono",
                props.class
            )}>
                { props.children.clone() }
            </span>
        }
    }
}
