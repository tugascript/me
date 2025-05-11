use super::links::Links;
use super::made::Made;
use leptos::prelude::*;

#[component]
pub fn Footer() -> impl IntoView {
    view! {
        <footer class="footer">
            <Links />
            <Made />
        </footer>
    }
}
