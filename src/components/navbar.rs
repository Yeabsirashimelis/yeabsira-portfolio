use leptos::prelude::*;

#[component]
pub fn Navbar(
    active_section: ReadSignal<String>,
    set_active_section: WriteSignal<String>,
) -> impl IntoView {
    let (menu_open, set_menu_open) = signal(false);

    let sections = vec!["home", "about", "skills", "projects", "contact"];

    view! {
        <nav class="navbar">
            <a class="nav-logo" href="#home" on:click=move |_| set_active_section.set("home".into())>
                <span class="logo-bracket">"{"</span>
                " YS "
                <span class="logo-bracket">"}"</span>
            </a>

            <button
                class="nav-toggle"
                on:click=move |_| set_menu_open.update(|v| *v = !*v)
            >
                <span class="hamburger" class:open=menu_open></span>
            </button>

            <ul class="nav-links" class:open=menu_open>
                {sections
                    .into_iter()
                    .map(|section| {
                        let section_owned = section.to_string();
                        let section_click = section.to_string();
                        view! {
                            <li>
                                <a
                                    href=format!("#{}", section)
                                    class:active=move || active_section.get() == section_owned
                                    on:click=move |_| {
                                        set_active_section.set(section_click.clone());
                                        set_menu_open.set(false);
                                    }
                                >
                                    {section.to_uppercase()}
                                </a>
                            </li>
                        }
                    })
                    .collect_view()}
            </ul>
        </nav>
    }
}
