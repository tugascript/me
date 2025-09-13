use leptos::prelude::*;

use crate::components::common::Title;

#[component]
pub fn HomePage() -> impl IntoView {
    view! {
        <Title title="Rust is love, Go is life" image_asset="nature" />
    }
}
