use leptos::prelude::*;

#[component]
pub fn Description() -> impl IntoView {
    view! {
        <section class="description">
            <p>
                "I am a back-end and data software engineer passionate about building scalable and data-driven microservices to solve complex challenges. "
                "Currently, I work as a Senior Software Engineer at "
                <a href="https://www.linkedin.com/company/mattrglobal" target="_blank">MATTR</a>
                ", where I contribute to their innovative digital identity platform."
            </p>
            <br />
            <p>
                "I transitioned from an academic background in econometrics with C, Python and R, to general software engineering with Rust, TypeScript, Go and Python. "
                "I work with diverse technology stacks, with a preference for Rust-based systems. "
                "I also flirt with the simplicity of go for backend development and pay my bills with an extensive knowledge of TypeScript (Node.js) and Python."
            </p>
            <br />
            <p>
                "My areas of expertise are in security, mainly in authentication, authorization and digital identity, as well as in financial services and econometrics."
            </p>
        </section>
    }
}
