use leptos::prelude::*;

#[component]
pub fn Contact() -> impl IntoView {
    view! {
        <section id="contact" class="contact">
            <h2 class="section-title">"contact"</h2>
            <div class="contact-content">
                <p class="contact-text">
                    "I'm always open to new opportunities, collaborations, or just a good "
                    "conversation about systems programming."
                </p>
                <div class="contact-links">
                    <a
                        href="https://github.com/YeabsiraShimelis"
                        target="_blank"
                        rel="noopener"
                        class="contact-link"
                    >
                        <span class="contact-icon">"gh"</span>
                        <span>"GitHub"</span>
                    </a>
                    <a href="mailto:shimelisyeabsira123@gmail.com" class="contact-link">
                        <span class="contact-icon">"@"</span>
                        <span>"Email"</span>
                    </a>
                    <a
                        href="https://www.linkedin.com/in/yeabsira-shimelis-4146a0385/"
                        target="_blank"
                        rel="noopener"
                        class="contact-link"
                    >
                        <span class="contact-icon">"in"</span>
                        <span>"LinkedIn"</span>
                    </a>
                </div>
            </div>
        </section>
    }
}
