use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct TextHighlightProps {
    #[prop_or_default]
    pub color: &'static str,
    #[prop_or_default]
    pub class: &'static str,
    pub children: Html,
}

#[function_component(TextHighlight)]
pub fn text_highlight(props: &TextHighlightProps) -> Html {
    let classes = classes!(
        "font-semibold",
        props.color,
        props.class
    );

    html! {
        <span class={classes}>
            { props.children.clone() }
        </span>
    }
}

impl TextHighlight {
    pub fn tech(label: &str) -> Html {
        html! {
            <TextHighlight color="text-rust-500">{ label }</TextHighlight>
        }
    }
}