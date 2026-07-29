use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct IconProps {
    pub name: &'static str,  // "sun", "moon", "download", "arrow-right", etc.
    #[prop_or(18)]
    pub size: u32,
    #[prop_or("currentColor")]
    pub color: &'static str,
    #[prop_or("2")]
    pub stroke_width: &'static str,
    #[prop_or_default]
    pub class: &'static str,
}

#[function_component(Icon)]
pub fn icon(props: &IconProps) -> Html {
    let size = props.size.to_string();
    match props.name {
        "sun" => html! {
            <svg class={format!("sun-icon hidden dark:block {}", props.class)} xmlns="http://www.w3.org/2000/svg" width={size.clone()} height={size.clone()} viewBox="0 0 24 24" fill="none" stroke={props.color} stroke-width={props.stroke_width} stroke-linecap="round" stroke-linejoin="round">
                <circle cx="12" cy="12" r="4"/>
                <path d="M12 2v2M12 20v2M4.93 4.93l1.41 1.41M17.66 17.66l1.41 1.41M2 12h2M20 12h2M4.93 19.07l1.41-1.41M17.66 6.34l1.41-1.41"/>
            </svg>
        },
        "moon" => html! {
            <svg class={format!("moon-icon block dark:hidden {}", props.class)} xmlns="http://www.w3.org/2000/svg" width={size.clone()} height={size.clone()} viewBox="0 0 24 24" fill="none" stroke={props.color} stroke-width={props.stroke_width} stroke-linecap="round" stroke-linejoin="round">
                <path d="M21 12.79A9 9 0 1 1 11.21 3 7 7 0 0 0 21 12.79z"/>
            </svg>
        },
        "download" => html! {
            <svg class={props.class} xmlns="http://www.w3.org/2000/svg" width={size.clone()} height={size.clone()} viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width={props.stroke_width} stroke-linecap="round" stroke-linejoin="round">
                <path d="M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4M7 10l5 5 5-5M12 15V3"/>
            </svg>
        },
        "arrow-right" => html! {
            <svg class={props.class} xmlns="http://www.w3.org/2000/svg" width={size.clone()} height={size.clone()} viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width={props.stroke_width} stroke-linecap="round" stroke-linejoin="round">
                <path d="M5 12h14M13 5l7 7-7 7"/>
            </svg>
        },
        "arrow-up" => html! {
            <svg class={props.class} xmlns="http://www.w3.org/2000/svg" width={size.clone()} height={size.clone()} viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width={props.stroke_width} stroke-linecap="round" stroke-linejoin="round">
                <path d="M12 19V5M5 12l7-7 7 7"/>
            </svg>
        },
        "bolt" => html! {
            <svg xmlns="http://www.w3.org/2000/svg" width={size.clone()} height={size} viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width={props.stroke_width} stroke-linecap="round" stroke-linejoin="round">
                <path d="M13 2L3 14h9l-1 8 10-12h-9l1-8z"/>
            </svg>
        },
        "shield" => html! {
            <svg xmlns="http://www.w3.org/2000/svg" width={size.clone()} height={size} viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width={props.stroke_width} stroke-linecap="round" stroke-linejoin="round">
                <path d="M12 22s8-4 8-10V5l-8-3-8 3v7c0 6 8 10 8 10z"/>
            </svg>
        },
        "terminal" => html! {
            <svg xmlns="http://www.w3.org/2000/svg" width={size.clone()} height={size} viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width={props.stroke_width} stroke-linecap="round" stroke-linejoin="round">
                <path d="M4 17l6-6-6-6M12 19h8"/>
            </svg>
        },
        _ => html! { <></> },
    }
}