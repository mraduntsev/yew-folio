mod api;
mod components;
mod content;
mod hooks;
mod sections;
mod ui;

use yew::prelude::*;
use components::{Navbar, Footer, ParticleCanvas};
use sections::{Hero, About, Education, Projects};
use hooks::{use_scroll_spy, ScrollSpyProps};

fn main() {
    yew::Renderer::<App>::new().render();
}

#[function_component(App)]
fn app() -> Html {
    let active_section = use_scroll_spy(&ScrollSpyProps {
        section_selectors: vec![
            "#about",
            "#education",
            "#projects",
            "#contact",
        ],
        threshold: 0.3,
    });
    html! {
        <>
            <Navbar active_section={active_section} />
            <Hero />
            <About />
            <Education />
            <Projects />
            // <Contact />
            <Footer />
            <ParticleCanvas />
        </>
    }
}