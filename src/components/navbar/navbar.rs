use leptos::prelude::*;
use leptos_router::components::A;

use super::buttons::NavbarButtons;

#[component]
pub fn Navbar() -> impl IntoView {
    let (is_menu_open, set_is_menu_open) = signal(false);

    provide_context(is_menu_open);
    provide_context(set_is_menu_open);

    view! {
        <nav class="navbar">
            <div class="container">
                <span class="navbar-brand">
                    <A href="/">"TugaScript"</A>
                </span>
                <button
                    class="burger-menu"
                    on:click=move |_| set_is_menu_open.update(|value| *value = !*value)
                >
                    <span class=move || if is_menu_open.get() { "open" } else { "" }></span>
                    <span class=move || if is_menu_open.get() { "open" } else { "" }></span>
                    <span class=move || if is_menu_open.get() { "open" } else { "" }></span>
                </button>
                <div class=move || format!("navbar-buttons {}", if is_menu_open.get() { "open" } else { "" })>
                    <NavbarButtons is_menu_open set_is_menu_open />
                </div>
            </div>
        </nav>
    }
}
