use leptos::prelude::*;

#[derive(Clone)]
struct TimelineData {
    title: &'static str,
    company: &'static str,
    location: &'static str,
    date: &'static str,
    points: Vec<&'static str>,
}

#[component]
fn TimelineItem(
    #[prop(into)] title: &'static str,
    #[prop(into)] company: &'static str,
    #[prop(into)] location: &'static str,
    #[prop(into)] date: &'static str,
    #[prop(into)] points: Vec<&'static str>,
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

#[component]
pub fn Timeline() -> impl IntoView {
    let timeline_data: [TimelineData; 5] = [
        TimelineData {
            title: "Senior Software Engineer",
            company: "MATTR",
            location: "Auckland, New Zealand",
            date: "FEB, 2024 - PRESENT",
            points: vec![
                "Revocation & Digital Credentials API support for ISO 18013-5 mobile driver license credentials.",
                "X.509 PKI Certificate lifecycle management using TypeScript and Node.JS.",
                "Native iOS (Swift) and Android (Kotlin) SDK linking with React Native (TypeScript) SDKs.",
            ],
        },
        TimelineData {
            title: "Software Engineer",
            company: "MATTR",
            location: "Auckland, New Zealand",
            date: "JUL, 2023 - FEB, 2024",
            points: vec![
                "OAuth 2.0 Client Credentials Grant authentication support to a claims source service.",
                "React Native (TypeScript) data migrations with Redux and Realm.",
            ],
        },
        TimelineData {
            title: "Software Developer",
            company: "MATTR",
            location: "Lisbon, Portugal",
            date: "SEP, 2022 - JUN, 2023",
            points: vec![
                "W3C Verifiable Credentials issuance and DID management for the BBS signature scheme.",
                "Kafka message queuing service for analytics in TypeScript and Node.JS",
                "Rust with TypeScript wrapper mantainance for elliptic curve crypto libraries.",
            ],
        },
        TimelineData {
            title: "Cloud Engineer Apprentice",
            company: "UpSkill, Deloitte",
            location: "Lisbon, Portugal",
            date: "FEB, 2021 - SEP, 2022",
            points: vec!["AWS focused Cloud Engineering Apprenticeship by ISCTE-IUL and Deloitte."],
        },
        TimelineData {
            title: "Full Stack Developer",
            company: "BuffQuest, Self-Employed",
            location: "Óbidos, Portugal",
            date: "MAR, 2020 - FEB, 2021",
            points: vec![
                "MVP app with Flutter & Dart with a TypeScript NestJS Node.JS GraphQL API development.",
                "Basic Python Django proof of concept development.",
            ],
        },
    ];

    view! {
        <div class="timeline">
            {timeline_data.iter().map(|item| {
                view! {
                    <TimelineItem
                        title=item.title
                        company=item.company
                        location=item.location
                        date=item.date
                        points=item.points.clone()
                    />
                }
            }).collect::<Vec<_>>()}
        </div>
    }
}
