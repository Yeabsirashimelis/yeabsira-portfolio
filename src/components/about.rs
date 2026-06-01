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
                </div>
            </div>
        </section>
    }
}
