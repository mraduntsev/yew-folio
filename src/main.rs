mod api;
mod components;
mod content;
mod hooks;
mod sections;
mod ui;

use yew::prelude::*;
// use sections::{Hero, About, Projects, Contact};
// use components::{Navbar, Footer};
use sections::{Hero, };
use components::{Navbar, };

fn main() {
    yew::Renderer::<App>::new().render();
}

#[function_component(App)]
fn app() -> Html {
    html! {
        <>
            <Navbar />
            <Hero />
            // <About />
            // <Projects />
            // <Contact />
            // <Footer />
        </>
    }
}