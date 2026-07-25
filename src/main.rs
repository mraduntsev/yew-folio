mod api;
mod components;
mod content;
mod hooks;
mod sections;
mod ui;

use components::{Footer, Navbar, ParticleCanvas};
use hooks::{ScrollSpyProps, use_scroll_spy};
use sections::{About, Contact, Education, Hero, Projects};
use yew::prelude::*;

use crate::hooks::LanguageProvider;

fn main() {
    yew::Renderer::<App>::new().render();
}

#[function_component(App)]
fn app() -> Html {
    let active_section = use_scroll_spy(&ScrollSpyProps {
        section_selectors: vec!["#about", "#education", "#projects", "#contact"],
        threshold: 0.3,
    });
    html! {
        <LanguageProvider>
            <Navbar active_section={active_section} />
            <Hero />
            <About />
            <Education />
            <Projects />
            <Contact />
            <Footer />
            <ParticleCanvas />
        </LanguageProvider>
    }
}
