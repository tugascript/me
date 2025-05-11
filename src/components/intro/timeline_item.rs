use leptos::prelude::*;

#[component]
pub fn TimelineItem(
    #[prop(into)] title: String,
    #[prop(into)] company: String,
    #[prop(into)] location: String,
    #[prop(into)] date: String,
    #[prop(into)] points: Vec<String>,
) -> impl IntoView {
    view! {
        <div class="timeline-item">
            <div class="timeline-content">
                <h3> {title}</h3>
                <h4><span>{"💻 "}</span>{company}</h4>
                <h5><span>{"📍 "}</span>{location}</h5>
                <p class="date">{date}</p>
                <ul>
                {move || {
                    points.clone().into_iter().map(|point| {
                        view! {
                            <li>{point}</li>
                        }
                    }).collect::<Vec<_>>()
                }}
                </ul>
            </div>
        </div>
    }
}
