use leptos::prelude::*;
use leptos_router::components::A;

#[component]
fn Link(#[prop(into)] href: String, #[prop(into)] text: String) -> impl IntoView {
    view! {
        <li class="footer-link">
            <A href={href}>
                {text}
            </A>
        </li>
    }
}

#[component]
fn ExternalLink(#[prop(into)] href: String, #[prop(into)] text: String) -> impl IntoView {
    view! {
        <li class="footer-link">
            <a href={href} target="_blank">
                {text}
            </a>
        </li>
    }
}

#[component]
pub fn Links() -> impl IntoView {
    view! {
        <div class="footer-links">
            <ExternalLink href="https://github.com/tugascript/me" text="Source Code" />
            <Link href="/privacy-policy" text="Privacy Policy" />
            <Link href="/terms-and-conditions" text="Terms and Conditions" />
        </div>
    }
}
