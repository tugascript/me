use leptos::prelude::*;
use leptos_router::components::A;

use crate::components::global::state::AuthenticatedState;

#[component]
pub fn SignInButton() -> impl IntoView {
    view! {
        <A href="/sign-in">
            "Sign In"
        </A>
    }
}

#[component]
pub fn UserButton() -> impl IntoView {
    view! {
        <A href="/me">
            "User"
        </A>
    }
}

#[component]
pub fn HomeScreenButton(
    #[prop(into)] is_menu_open: ReadSignal<bool>,
    #[prop(into)] set_is_menu_open: WriteSignal<bool>,
) -> impl IntoView {
    let on_click = move |_| {
        set_is_menu_open.set(false);
    };

    view! {
        <A href="/" on:click=on_click>
            {move || {
                if is_menu_open.get() {
                    "TugaScript"
                } else {
                    ""
                }
            }}
        </A>
    }
}

#[component]
pub fn CommonButtons(
    #[prop(into)] is_menu_open: ReadSignal<bool>,
    #[prop(into)] set_is_menu_open: WriteSignal<bool>,
) -> impl IntoView {
    let on_click = move |_| {
        set_is_menu_open.set(false);
    };

    view! {
        <HomeScreenButton is_menu_open set_is_menu_open />
        <A href="/portfolio" on:click=on_click>
            "Portfolio"
        </A>
        <A href="/blog" on:click=on_click>
            "Blog"
        </A>
        <A href="/contact" on:click=on_click>
            "Contact Me"
        </A>
    }
}

#[component]
pub fn AuthButton(#[prop(into)] set_is_menu_open: WriteSignal<bool>) -> impl IntoView {
    let auth_state = use_context::<ReadSignal<AuthenticatedState>>();
    let on_click = move |_| {
        set_is_menu_open.set(false);
    };

    let is_authenticated = move || {
        auth_state
            .map(|state| state.get().is_authenticated)
            .unwrap_or(false)
    };

    view! {
        {move || {
            if is_authenticated() {
                view! {
                    <A href="/me" on:click=on_click>
                        "User"
                    </A>
                }
            } else {
                view! {
                    <A href="/sign-in" on:click=on_click>
                        "Sign In"
                    </A>
                }
            }
        }}
    }
}

#[component]
pub fn NavbarButtons(
    #[prop(into)] is_menu_open: ReadSignal<bool>,
    #[prop(into)] set_is_menu_open: WriteSignal<bool>,
) -> impl IntoView {
    view! {
        <CommonButtons is_menu_open set_is_menu_open />
        <AuthButton set_is_menu_open />
    }
}
