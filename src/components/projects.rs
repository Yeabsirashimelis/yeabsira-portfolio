use leptos::prelude::*;

struct Project {
    name: &'static str,
    description: &'static str,
    tech: Vec<&'static str>,
    url: &'static str,
    stars: u32,
}

fn render_project_card(project: Project) -> impl IntoView {
    view! {
        <a class="project-card" href=project.url target="_blank" rel="noopener">
            <div class="project-header">
                <h3>{project.name}</h3>
                <span class="project-stars">
                    {format!("* {}", project.stars)}
                </span>
            </div>
            <p class="project-desc">{project.description}</p>
            <div class="project-tech">
                {project
                    .tech
                    .into_iter()
                    .map(|t| view! { <span class="tech-tag">{t}</span> })
                    .collect_view()}
            </div>
        </a>
    }
}

#[component]
pub fn Projects() -> impl IntoView {
    let (active_tab, set_active_tab) = signal("rust");

    let rust_projects = vec![
        Project {
            name: "oxide-http1.1",
            description: "A fully functional multithreaded HTTP/1.1 server built from scratch in Rust. Features routing, gzip compression, and thread pooling.",
            tech: vec!["Rust", "Networking", "Multithreading"],
            url: "https://github.com/YeabsiraShimelis/oxide-http1.1",
            stars: 9,
        },
        Project {
            name: "oxide-shell",
            description: "A modern, POSIX-compliant command-line shell written in Rust. Interactive REPL with history, tab completion, I/O redirection, builtins, and cross-platform support.",
            tech: vec!["Rust", "Systems", "CLI"],
            url: "https://github.com/Yeabsirashimelis/oxide-shell",
            stars: 1,
        },
        Project {
            name: "CPU Emulator",
            description: "A CHIP-8 emulator written in Rust with manual instruction loading. Dive deep into how CPUs actually execute instructions.",
            tech: vec!["Rust", "Emulation", "Low-level"],
            url: "https://github.com/YeabsiraShimelis/CPU-emulator",
            stars: 5,
        },
        Project {
            name: "Multithreaded Web Server",
            description: "A concurrent web server implementation in Rust, handling multiple connections with a custom thread pool.",
            tech: vec!["Rust", "Concurrency", "TCP"],
            url: "https://github.com/YeabsiraShimelis/Multithreaded-web-server-with-RUST",
            stars: 4,
        },
        Project {
            name: "oxide-paas",
            description: "A Platform as a Service built in Rust. Includes agent management, database migrations, and deployment tooling — under active development.",
            tech: vec!["Rust", "Cloud", "Infrastructure"],
            url: "https://github.com/Yeabsirashimelis/oxide-paas",
            stars: 0,
        },
    ];

    let ts_projects = vec![
        Project {
            name: "oxide.js",
            description: "A TypeScript backend framework built directly on Node's HTTP module. Express-like DX with a cleaner ctx-based API, built-in middleware, WebSocket support, and rate limiting.",
            tech: vec!["TypeScript", "Node.js", "WebSocket"],
            url: "https://github.com/Yeabsirashimelis/oxide.js",
            stars: 0,
        },
        Project {
            name: "mock-oxide",
            description: "A developer-first platform to design, deploy, and share mock API endpoints. Schema-based JSON responses, request validation, and OpenAPI 3.0 support.",
            tech: vec!["TypeScript", "Next.js", "Prisma", "PostgreSQL"],
            url: "https://github.com/Yeabsirashimelis/mock-oxide",
            stars: 0,
        },
        Project {
            name: "Top Tutor",
            description: "Full-stack tutoring platform with a backend API and instructor dashboard for managing courses and students.",
            tech: vec!["TypeScript", "Node.js", "React"],
            url: "https://github.com/YeabsiraShimelis/top-tutor-backend-and-instructor-dashboard",
            stars: 3,
        },
        Project {
            name: "Landlords House Rental",
            description: "A full-stack house rental platform connecting landlords and tenants with real-time features.",
            tech: vec!["JavaScript", "Node.js", "MongoDB"],
            url: "https://github.com/YeabsiraShimelis/Landlords-house-rental",
            stars: 5,
        },
        Project {
            name: "World Wise",
            description: "Single page application with an interactive world map that tracks your travel footsteps across the globe.",
            tech: vec!["JavaScript", "React", "Leaflet"],
            url: "https://github.com/YeabsiraShimelis/world-wise",
            stars: 3,
        },
    ];

    let rust_cards = rust_projects
        .into_iter()
        .map(render_project_card)
        .collect_view();

    let ts_cards = ts_projects
        .into_iter()
        .map(render_project_card)
        .collect_view();

    view! {
        <section id="projects" class="projects">
            <h2 class="section-title">"projects"</h2>

            <div class="project-tabs">
                <button
                    class="tab-btn"
                    class:active=move || active_tab.get() == "rust"
                    on:click=move |_| set_active_tab.set("rust")
                >
                    "rust"
                </button>
                <button
                    class="tab-btn"
                    class:active=move || active_tab.get() == "typescript"
                    on:click=move |_| set_active_tab.set("typescript")
                >
                    "typescript"
                </button>
            </div>

            <div class="projects-grid" style:display=move || {
                if active_tab.get() == "rust" { "flex" } else { "none" }
            }>
                {rust_cards}
            </div>

            <div class="projects-grid" style:display=move || {
                if active_tab.get() == "typescript" { "flex" } else { "none" }
            }>
                {ts_cards}
            </div>

            <div class="projects-more">
                <a
                    href="https://github.com/YeabsiraShimelis"
                    target="_blank"
                    rel="noopener"
                    class="btn btn-outline"
                >
                    "see_all_repos() // 57+"
                </a>
            </div>
        </section>
    }
}
