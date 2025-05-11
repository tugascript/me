use super::socials::Socials;

use leptos::prelude::*;

#[component]
pub fn Title() -> impl IntoView {
    view! {
        <div class="title-container">
            <div class="title">
                <img src="/assets/me.jpg" alt="Afonso Barracha" />
                <h1>"Afonso Barracha"</h1>
                <h2>"Senior Software Engineer"</h2>
                <Socials />
            </div>
        </div>
    }
}
