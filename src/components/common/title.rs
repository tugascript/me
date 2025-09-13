use super::socials::Socials;

use leptos::prelude::*;

#[component]
pub fn Title(
    #[prop(into)] image_asset: &'static str,
    #[prop(into)] title: &'static str,
) -> impl IntoView {
    view! {
        <div class="title-container">
            <div class="title">
                <img src={"/assets/".to_string() + image_asset + ".jpg"} alt="Afonso Barracha" />
                <h1>"Afonso Barracha"</h1>
                <h2>{title}</h2>
                <Socials />
            </div>
        </div>
    }
}
