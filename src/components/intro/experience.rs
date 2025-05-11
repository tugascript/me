use super::timeline::Timeline;
use leptos::prelude::*;

#[component]
pub fn Experience() -> impl IntoView {
    view! {
        <div class="experience-container">
            <section class="experience">
                <h2>"Experience"</h2>
                <Timeline />
            </section>
        </div>
    }
}
