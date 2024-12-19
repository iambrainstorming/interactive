use crate::components::{mcq_struct::McqData, mcqs::Mcq, navigation::nav::Nav};
use leptos::prelude::*;

#[component]
pub fn AnxietyDisorders() -> impl IntoView {
    let what_is_anxiety = McqData {
        question: "What is anxiety?".to_string(),
        correct_answer: "A response to a vague, distant, or unrecognized internal danger."
            .to_string(),
        options: vec![
            "A fear of a specific, concrete external object or situation.".to_string(),
            "A feeling of excitement and euphoria in response to a challenging situation."
                .to_string(),
            "A state of complete calmness and relaxation.".to_string(),
        ],
        context: r#"You can better understand the nature of anxiety by looking at both what it is and what it is not. For example, anxiety can be distinguished from fear in several ways. When you are afraid, your fear is usually directed toward some concrete external object or situation. The event that you fear usually is within the bounds of possibility. You might fear not meeting a deadline, failing an exam, being unable to pay your bills, or being rejected by someone you want to please. When you experience anxiety, on the other hand, you often can’t specify what it is you’re anxious about. The focus of anxiety is more internal than external. It seems to be a response to a vague, distant, or even unrecognized danger. You might be anxious about “losing control” of yourself or some situation. Or you might feel a vague anxiety about “something bad happening.”"#.to_string(),
    };

    let which_on_anxiety = McqData {
            question: "Which of these is an example of anxiety?".to_string(),
            correct_answer: "Fearing something bad will happen and you will lose control of yourself".to_string(),
            options: vec![
                "Fearing not meeting a deadline".to_string(),
                "Fearing failing an exam".to_string(),
                "Fearing being rejected by someone you want to please".to_string(),
            ],
            context: r#"Anxiety is characterized by a vague, distant, or unrecognized danger, and a fear of "losing control" of oneself or some situation. Options A, B, and C are examples of fear, which is directed towards a concrete external object or situation, whereas option D is an example of anxiety, which is a more internal and vague fear."#
                .to_string(),
        };
    let affect_of_anxiety = McqData {
            question: "How does anxiety affect your body and mind?".to_string(),
            correct_answer: "All of the above: physically, behaviorally, and psychologically".to_string(),
            options: vec![
                "It is physiological, including rapid heartbeat, muscle tension, queasiness, dry mouth, or sweating.".to_string(),
                "It is behavioral, affecting your ability to act or express yourself.".to_string(),
                "It's psychological, characterized by fearful or uneasy anticipation of the future, or uneasiness. In extreme forms, it can cause you to feel detached from yourself and even fearful of dying or going crazy.".to_string(),
            ],
            context: r#"Anxiety can appear in different forms and at different levels of intensity. It can range in severity from a mere twinge of uneasiness to a full-blown panic attack marked by heart palpitations, disorientation, and terror.<br/>Shortness of breath<br/>
            Heart palpitations (rapid or irregular heartbeat)<br/>
            Trembling or shaking<br/>
            Sweating<br/>
            Choking<br/>
            Nausea or abdominal distress<br/>
            Numbness<br/>
            Dizziness or unsteadiness<br/>
            Feeling of detachment or being out of touch with yourself<br/>
            Hot flashes or chills<br/>
            Fear of dying<br/>
            Fear of going crazy or out of control"#
                .to_string(),
        };
    let anxiety_disorders = McqData {
            question: "Anxiety disorders are distinguished from everyday, normal anxiety in that they involve anxiety that".to_string(),
            correct_answer: "All of the above".to_string(),
            options: vec![
                "is more intense (for example, panic attacks)".to_string(),
                "lasts longer (anxiety that may persist for months instead of going away after a stressful situation has passed)".to_string(),
                "leads to phobias that interfere with your life".to_string(),
            ],
            context: r#""#
                .to_string(),
        };

    view! {
        <div>
            <Nav/>
            <div class="container mx-auto">
                <Mcq data=what_is_anxiety/>
                <Mcq data=which_on_anxiety/>
                <Mcq data=affect_of_anxiety/>
                <Mcq data=anxiety_disorders/>
            </div>
        </div>
    }
}
