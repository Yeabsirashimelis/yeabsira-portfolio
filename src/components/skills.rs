use leptos::prelude::*;

struct SkillCategory {
    name: &'static str,
    items: Vec<&'static str>,
}

#[component]
pub fn Skills() -> impl IntoView {
    let categories = vec![
        SkillCategory {
            name: "languages",
            items: vec!["Rust", "TypeScript", "Go", "JavaScript"],
        },
        SkillCategory {
            name: "backend",
            items: vec!["Node.js", "Express", "Actix-web", "Socket.IO"],
        },
        SkillCategory {
            name: "frontend",
            items: vec!["React", "Next.js", "Leptos", "WASM"],
        },
        SkillCategory {
            name: "databases",
            items: vec!["PostgreSQL", "MongoDB", "MySQL", "Supabase"],
        },
        SkillCategory {
            name: "devops",
            items: vec!["Docker", "GitHub Actions", "Cloudflare", "Vercel"],
        },
        SkillCategory {
            name: "systems",
            items: vec!["Multithreading", "Networking", "HTTP Internals", "Emulation"],
        },
    ];

    view! {
        <section id="skills" class="skills">
            <h2 class="section-title">"skills"</h2>
            <div class="skills-list">
                {categories
                    .into_iter()
                    .map(|cat| {
                        view! {
                            <div class="skill-row">
                                <span class="skill-label">{cat.name}</span>
                                <span class="skill-items">
                                    {cat
                                        .items
                                        .join(" / ")}
                                </span>
                            </div>
                        }
                    })
                    .collect_view()}
            </div>
        </section>
    }
}
