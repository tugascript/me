use super::timeline_item::TimelineItem;
use leptos::prelude::*;

#[component]
pub fn Timeline() -> impl IntoView {
    view! {
        <div class="timeline">
            <TimelineItem
                title="Senior Software Engineer"
                company="MATTR"
                location="Auckland, New Zealand"
                date="FEB, 2024 - PRESENT"
                points={vec![
                    "Support ISO 18013-5 digital credentials custom revocation and Digital Credential API.".to_string(),
                    "Implemented X.509 certificate lifecycle management (TypeScript/Node.js).".to_string(),
                    "Linked native iOS (Swift) and Android (Kotlin) to React Native (TypeScript) SDKs".to_string(),
                ]}
            />
            <TimelineItem
                title="Software Engineer"
                company="MATTR"
                location="Auckland, New Zealand"
                date="JUL, 2023 - FEB, 2024"
                points={vec![
                    "Implemented OAuth 2.0 Client Credentials Grant for claims source authentication".to_string(),
                    "Managed React Native state & data (TypeScript) with Redux and Realm.".to_string(),
                    "Enabled PDF generation & upload with ClamAV TCP virus scanning.".to_string(),
                ]}
            />
            <TimelineItem
                title="Software Developer"
                company="MATTR"
                location="Lisbon, Portugal"
                date="SEP, 2022 - JUN, 2023"
                points={vec![
                    "Implemented W3C Verifiable Credentials and DID management for BBS signatures.".to_string(),
                    "Upgraded Kafka analytics message queuing (TypeScript/Node.js).".to_string(),
                    "Maintained Rust with TypeScript wrapper for elliptic curve crypto libraries.".to_string(),
                ]}
            />
            <TimelineItem
                title="Cloud Engineer Apprentice"
                company="UpSkill, Deloitte"
                location="Lisbon, Portugal"
                date="FEB, 2021 - SEP, 2022"
                points={vec![
                    "AWS focused Cloud Engineering Apprenticeship by ISCTE-IUL and Deloitte".to_string(),
                    "Developed a React & NestJS application for school management in JavaScript".to_string(),
                ]}
            />
            <TimelineItem
                title="Full Stack Developer"
                company="BuffQuest, Self-Employed"
                location="Óbidos, Portugal"
                date="MAR, 2020 - FEB, 2021"
                points={vec![
                    "Flutter APP and NestJS API MVP for a fitness app".to_string(),
                    "Django proof of concept for a web app".to_string(),
                ]}
            />
        </div>
    }
}
