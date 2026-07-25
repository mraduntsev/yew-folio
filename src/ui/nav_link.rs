use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct NavLinkProps {
    pub href: String,
    pub label: String,
}

#[function_component(NavLinkItem)]
pub fn nav_link(props: &NavLinkProps) -> Html {
    html! {
        <li>
            <a
                href={props.href.clone()}
                class="nav-link text-ink-600 dark:text-ink-300 hover:text-rust-500 transition"
                data-section={props.label.clone()}
            >
                { props.label.clone() }
            </a>
        </li>
    }
}
