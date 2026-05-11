use leptos::prelude::*;

struct SkillCategory {
    name: &'static str,
    icon: &'static str,
    items: Vec<&'static str>,
}

#[component]
pub fn Skills() -> impl IntoView {
    let categories = vec![
        SkillCategory {
            name: "Languages",
            icon: ">>",
            items: vec!["Rust", "TypeScript", "Go", "JavaScript"],
        },
        SkillCategory {
            name: "Backend",
            icon: "[]",
            items: vec!["Node.js", "Express", "Actix-web", "Socket.IO"],
        },
        SkillCategory {
            name: "Frontend",
            icon: "<>",
            items: vec!["React", "Next.js", "Leptos", "WASM"],
        },
        SkillCategory {
            name: "Databases",
            icon: "{}",
            items: vec!["PostgreSQL", "MongoDB", "MySQL", "Supabase"],
        },
        SkillCategory {
            name: "DevOps & Cloud",
            icon: "~/",
            items: vec!["Docker", "GitHub Actions", "Cloudflare", "Vercel"],
        },
        SkillCategory {
            name: "Systems",
            icon: "0x",
            items: vec!["Multithreading", "Networking", "HTTP Internals", "Emulation"],
        },
    ];

    view! {
        <section id="skills" class="skills">
            <h2 class="section-title">"Skills"</h2>
            <div class="skills-grid">
                {categories
                    .into_iter()
                    .map(|cat| {
                        view! {
                            <div class="skill-card">
                                <span class="skill-icon">{cat.icon}</span>
                                <h3>{cat.name}</h3>
                                <ul>
                                    {cat
                                        .items
                                        .into_iter()
                                        .map(|item| view! { <li>{item}</li> })
                                        .collect_view()}
                                </ul>
                            </div>
                        }
                    })
                    .collect_view()}
            </div>
        </section>
    }
}
