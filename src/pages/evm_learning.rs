use crate::components::{mcq_struct::McqData, mcqs::Mcq};
use leptos::prelude::*;

#[component]
pub fn EvmLearning() -> impl IntoView {
    let question_data = McqData {
        question: "Which statement best describes the impact of secrecy on software security?".to_string(),
        correct_answer: "Secrecy is a liability, as it hinders the identification and patching of security flaws by a wider community of experts.".to_string(),
        options: vec![
            "Secrecy enhances software security by preventing malicious actors from exploiting vulnerabilities.".to_string(),
            "Secrecy fosters an environment of trust, as users perceive hidden mechanisms as more secure.".to_string(),
            "Open-source software exposes code to a wider range of attackers, increasing the risk of exploitation.".to_string(),
        ],
        context: r#"Secrecy is a liability in software security. By keeping the software development process closed and secretive, it becomes more challenging to identify and address security vulnerabilities. A more effective approach is to embrace open-source development, where the source code is publicly available for review and contribution by a community of developers and security experts."#
            .to_string(),
    };
    view! { <Mcq data=question_data/> }
}
