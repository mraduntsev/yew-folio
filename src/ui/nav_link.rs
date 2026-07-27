use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct NavLinkProps {
    pub href: &'static str,
    pub label: &'static str,
}

#[function_component(NavLinkItem)]
pub fn nav_link(props: &NavLinkProps) -> Html {
    html! {
        <li>
            <a
                href={props.href}
                class="nav-link text-ink-600 dark:text-ink-300 hover:text-rust-500 transition"
                data-section={props.label}
            >
                { props.label }
            </a>
        </li>
    }
}