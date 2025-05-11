use crate::components::{footer::Footer, intro::Intro, navbar::Navbar};
use crate::pages::{PrivacyPolicy, TermsAndConditions};
use leptos::prelude::*;
use leptos_meta::{Link, Stylesheet, Title, provide_meta_context};
use leptos_router::{
    StaticSegment, WildcardSegment,
    components::{Route, Router, Routes},
};

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {
        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/tugascript.css"/>
        <Link rel="preconnect" href="https://fonts.googleapis.com" />
        <Link rel="preconnect" href="https://fonts.gstatic.com" crossorigin="anonymous" />
        <Link href="https://fonts.googleapis.com/css2?family=Roboto:ital,wght@0,100..900;1,100..900&display=swap" rel="stylesheet" />


        // sets the document title
        <Title text="TugaScript"/>

        // content for this welcome page
        <Router>
            <main>
                <Navbar/>
                <Routes fallback=move || "Not found.">
                    <Route path=StaticSegment("") view=HomePage/>
                    <Route path=StaticSegment("portfolio") view=PortfolioPage/>
                    <Route path=StaticSegment("blog") view=BlogPage/>
                    <Route path=StaticSegment("contact") view=ContactPage/>
                    <Route path=StaticSegment("privacy-policy") view=PrivacyPolicy/>
                    <Route path=StaticSegment("terms-and-conditions") view=TermsAndConditions/>
                    <Route path=WildcardSegment("any") view=NotFound/>
                </Routes>
                <Footer/>
            </main>
        </Router>
    }
}

/// Renders the home page of your application.
#[component]
fn HomePage() -> impl IntoView {
    view! {
        <Intro/>
    }
}

/// 404 - Not Found
#[component]
fn NotFound() -> impl IntoView {
    // set an HTTP status code 404
    // this is feature gated because it can only be done during
    // initial server-side rendering
    // if you navigate to the 404 page subsequently, the status
    // code will not be set because there is not a new HTTP request
    // to the server
    #[cfg(feature = "ssr")]
    {
        // this can be done inline because it's synchronous
        // if it were async, we'd use a server function
        let resp = expect_context::<leptos_actix::ResponseOptions>();
        resp.set_status(actix_web::http::StatusCode::NOT_FOUND);
    }

    view! {
        <h1>"Not Found"</h1>
    }
}

#[component]
fn PortfolioPage() -> impl IntoView {
    view! {
        <h1>"Portfolio"</h1>
        <p>"Welcome to my portfolio page!"</p>
    }
}

#[component]
fn BlogPage() -> impl IntoView {
    view! {
        <h1>"Blog"</h1>
        <p>"Welcome to my blog!"</p>
    }
}

#[component]
fn ContactPage() -> impl IntoView {
    view! {
        <h1>"Contact Me"</h1>
        <p>"Get in touch with me!"</p>
    }
}
