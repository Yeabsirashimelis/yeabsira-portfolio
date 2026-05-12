mod components;

use components::{
    about::About, contact::Contact, hero::Hero, navbar::Navbar, projects::Projects,
    skills::Skills,
};
use leptos::prelude::*;

fn main() {
    mount_to_body(App);
}

#[component]
fn App() -> impl IntoView {
    let (active_section, set_active_section) = signal(String::from("home"));

    view! {
        <Navbar active_section=active_section set_active_section=set_active_section />
        <main>
            <Hero />
            <hr class="section-divider" />
            <About />
            <hr class="section-divider" />
            <Skills />
            <hr class="section-divider" />
            <Projects />
            <hr class="section-divider" />
            <Contact />
        </main>
        <footer class="footer">
            <p>"Built with Rust + Leptos + WebAssembly"</p>
            <p class="footer-sub">"Compiled to WASM. No JavaScript frameworks were harmed."</p>
        </footer>
    }
}
