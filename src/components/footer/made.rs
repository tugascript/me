use chrono::{Datelike, Local};
use leptos::prelude::*;

#[component]
pub fn Made() -> impl IntoView {
    view! {
        <div class="footer-made">
            "Made with "<span>{"❤️"}</span>
            " and " <a href="https://leptos.dev/" target="_blank">{"Leptos"}</a>
            " by Afonso Barracha "
            <span title="copyright">{"©"}</span>" "
            {move || {
                let year = Local::now().year();
                year.to_string()
            }}
        </div>
    }
}
