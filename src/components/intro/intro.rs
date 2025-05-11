use super::description::Description;
use super::experience::Experience;
use super::title::Title;

use leptos::prelude::*;

#[component]
pub fn Intro() -> impl IntoView {
    view! {
        <Title />
        <Description />
        <Experience />
    }
}
