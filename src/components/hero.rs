use leptos::prelude::*;

#[component]
pub fn Hero() -> impl IntoView {
    let (typed_text, set_typed_text) = signal(String::new());
    let full_text = "Systems Developer & Full-Stack Engineer";

    // Typewriter effect using set_interval
    let chars: Vec<char> = full_text.chars().collect();
    let total = chars.len();
    let idx = StoredValue::new(0usize);

    set_interval(
        move || {
            idx.update_value(|i| {
                if *i < total {
                    set_typed_text.update(|t| t.push(chars[*i]));
                    *i += 1;
                }
            });
        },
        std::time::Duration::from_millis(60),
    );

    view! {
        <section id="home" class="hero">
            <div class="hero-content">
                <p class="hero-greeting">"Hello, I'm"</p>
                <h1 class="hero-name">"Yeabsira Shimelis"</h1>
                <p class="hero-title">
                    <span class="typed">{typed_text}</span>
                    <span class="cursor">"|"</span>
                </p>
                <p class="hero-tagline">
                    "I build high-performance systems in "
                    <span class="highlight">"Rust"</span>
                    " and ship products with "
                    <span class="highlight">"TypeScript"</span>
                    "."
                </p>
                <div class="hero-cta">
                    <a href="#projects" class="btn btn-primary">"View My Work"</a>
                    <a href="#contact" class="btn btn-outline">"Get In Touch"</a>
                </div>
            </div>
            <div class="hero-decoration">
                <pre class="code-block">
{r#"fn main() {
    let developer = Developer {
        name: "Yeabsira",
        focus: vec![
            "Systems Programming",
            "Backend Architecture",
            "Full-Stack Web",
        ],
        languages: vec![
            "Rust", "TypeScript", "Go",
        ],
    };
    developer.build_amazing_things();
}"#}
                </pre>
            </div>
        </section>
    }
}
