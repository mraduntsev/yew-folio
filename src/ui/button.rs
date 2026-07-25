use yew::prelude::*;

use crate::ui::Icon;

#[derive(Properties, PartialEq)]
pub struct ButtonProps {
    #[prop_or_default]
    pub href: String,
    pub variant: String, // "primary", "outline", "ghost"
    #[prop_or("sm".into())]
    pub size: String,
    #[prop_or_default]
    pub left_icon: Option<&'static str>,
    #[prop_or_default]
    pub right_icon: Option<&'static str>,
    #[prop_or_default]
    pub onclick: Option<Callback<MouseEvent>>,
    #[prop_or_default]
    pub class: &'static str,
    pub children: Html,
}

#[function_component(Button)]
pub fn button(props: &ButtonProps) -> Html {
    let base_class = classes!(
        "inline-flex",
        "items-center",
        "gap-2",
        "font-semibold",
        "transition",
        match props.size.as_str() {
            "md" => "px-5 py-3 rounded-xl text-base",
            _ => "px-4 py-2 rounded-lg text-sm",
        },
        match props.variant.as_str() {
            "primary" => "bg-rust-500 hover:bg-rust-600 text-white shadow-lg shadow-rust-500/20",
            "outline" =>
                "border border-rust-500/40 text-rust-500 hover:bg-rust-500 hover:text-white",
            _ => "",
        },
        props.class,
    );

    let inner = html! {
        <>
            if let Some(icon) = props.left_icon {
                <Icon name={icon} size="14" stroke_width="2.5" />
            }
            { props.children.clone() }
            if let Some(icon) = props.right_icon {
                <Icon name={icon} size="14" stroke_width="2.5" />
            }
        </>
    };

    if !props.href.is_empty() {
        let download_attr = if props.href.contains(".pdf") {
            Some("resume.pdf")
        } else {
            None
        };
        html! {
            <a href={props.href.clone()} class={base_class} download={download_attr}>
                { inner }
            </a>
        }
    } else {
        let onclick = props.onclick.clone();
        html! {
            <button class={base_class} onclick={move |e: MouseEvent| {
                if let Some(ref cb) = onclick {
                    cb.emit(e);
                }
            }}>
                { inner }
            </button>
        }
    }
}
