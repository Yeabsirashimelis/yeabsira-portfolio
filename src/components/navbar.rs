use leptos::prelude::*;
use wasm_bindgen::prelude::*;

#[component]
pub fn Navbar(
    active_section: ReadSignal<String>,
    set_active_section: WriteSignal<String>,
) -> impl IntoView {
    let (menu_open, set_menu_open) = signal(false);

    let sections = vec!["home", "about", "skills", "projects", "contact"];

    // Scroll spy: update active section based on scroll position
    let closure = Closure::<dyn Fn()>::new(move || {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();
        let scroll_y = window.scroll_y().unwrap_or(0.0);
        let offset = 150.0;

        let section_ids = ["contact", "projects", "skills", "about", "home"];
        for id in section_ids {
            if let Some(el) = document.get_element_by_id(id) {
                let top = el.get_bounding_client_rect().top() + scroll_y - offset;
                if scroll_y >= top {
                    set_active_section.set(id.to_string());
                    break;
                }
            }
        }
    });

    let window = web_sys::window().unwrap();
    window
        .add_event_listener_with_callback("scroll", closure.as_ref().unchecked_ref())
        .unwrap();
    closure.forget();

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
