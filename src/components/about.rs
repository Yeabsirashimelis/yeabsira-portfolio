use leptos::prelude::*;

#[component]
pub fn About() -> impl IntoView {
    view! {
        <section id="about" class="about">
            <h2 class="section-title">"About Me"</h2>
            <div class="about-content">
                <div class="about-text">
                    <p>
                        "I'm a systems-minded developer who thrives at the intersection of "
                        "performance and developer experience. Currently working at "
                        <strong>"Eaglepoint AI"</strong>
                        ", I build backend systems and full-stack applications "
                        "that prioritize reliability and speed."
                    </p>
                    <p>
                        "From building an HTTP/1.1 server from scratch in Rust to crafting "
                        "a CHIP-8 CPU emulator, I love understanding how things work at the "
                        "lowest level — then using that knowledge to build better software "
                        "at every layer of the stack."
                    </p>
                    <p>
                        "This portfolio itself is built with "
                        <strong>"Rust + Leptos"</strong>
                        ", compiled to "
                        <strong>"WebAssembly"</strong>
                        " — because even my website should be fast."
                    </p>
                </div>
                <div class="about-stats">
                    <div class="stat">
                        <span class="stat-number">"57+"</span>
                        <span class="stat-label">"Repositories"</span>
                    </div>
                    <div class="stat">
                        <span class="stat-number">"4"</span>
                        <span class="stat-label">"Languages"</span>
                    </div>
                    <div class="stat">
                        <span class="stat-number">"∞"</span>
                        <span class="stat-label">"Curiosity"</span>
                    </div>
                </div>
            </div>
        </section>
    }
}
