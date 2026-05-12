use leptos::prelude::*;

#[component]
pub fn About() -> impl IntoView {
    view! {
        <section id="about" class="about">
            <h2 class="section-title">"about"</h2>
            <div class="about-content">
                <div class="about-text">
                    <p>
                        "Systems-minded developer at "
                        <strong>"Eaglepoint AI"</strong>
                        ". I build backend systems and full-stack apps that "
                        "prioritize reliability and speed. From HTTP servers from scratch "
                        "to CPU emulators — I understand things at the lowest level."
                    </p>
                    <p>
                        "This portfolio: "
                        <strong>"Rust + Leptos"</strong>
                        " compiled to "
                        <strong>"WebAssembly"</strong>
                        "."
                    </p>
                </div>
                <div class="about-stats">
                    <div class="stat">
                        <span class="stat-number">"57+"</span>
                        <span class="stat-label">"repos"</span>
                    </div>
                    <div class="stat">
                        <span class="stat-number">"4"</span>
                        <span class="stat-label">"languages"</span>
                    </div>
                    <div class="stat">
                        <span class="stat-number">"~"</span>
                        <span class="stat-label">"curiosity"</span>
                    </div>
                </div>
            </div>
        </section>
    }
}
