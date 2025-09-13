use super::description::Description;
use super::experience::Experience;
use crate::components::common::Title;

use leptos::prelude::*;

#[component]
pub fn Intro() -> impl IntoView {
    view! {
        <Title title="Senior Software Engineer" image_asset="me" />
        <Description />
        <Experience />
    }
}
