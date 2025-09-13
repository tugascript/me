// Copyright (c) 2025 Afonso Barracha
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use leptos::prelude::*;
use leptos_router::{components::A, hooks::use_location};

use crate::components::global::state::AuthenticatedState;

#[component]
pub fn NavbarButton(
    #[prop(into)] set_is_menu_open: WriteSignal<bool>,
    #[prop(into)] text: String,
    #[prop(into)] href: String,
) -> impl IntoView {
    let path = use_location().pathname;

    view! {
        <A href={href.clone()} on:click=move |_| set_is_menu_open.set(false)>
            <span class=move || if path.get() == href { "active" } else { "" }>
                {text}
            </span>
        </A>
    }
}

#[component]
fn CommonButtons(#[prop(into)] set_is_menu_open: WriteSignal<bool>) -> impl IntoView {
    let path = use_location().pathname;

    view! {
        <NavbarButton set_is_menu_open text="Home" href="/" />
        <NavbarButton set_is_menu_open text="Portfolio" href="/portfolio" />
        <NavbarButton set_is_menu_open text="CV" href="/cv" />
        <A
            href="/blog"
            on:click=move |_| set_is_menu_open.set(false)
        >
            <span class=move || if path.get().contains("/blog") { "active" } else { "" }>
                "Blog"
            </span>
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
pub fn NavbarButtons(#[prop(into)] set_is_menu_open: WriteSignal<bool>) -> impl IntoView {
    view! {
        <CommonButtons set_is_menu_open />
        <AuthButton set_is_menu_open />
    }
}
