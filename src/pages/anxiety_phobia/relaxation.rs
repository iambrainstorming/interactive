use crate::components::{mcq_struct::McqData, mcqs::Mcq, navigation::nav::Nav};
use leptos::prelude::*;

#[component]
pub fn Relaxation() -> impl IntoView {
    let foundation_program = McqData {
            question: "What is the foundation program that needs to be undertaken to overcome anxiety, phobias, or panic attacks?".to_string(),
            correct_answer: "Regular, daily practice of deep relaxation technique".to_string(),
            options: vec![
                "Watching TV and distracting yourself".to_string(),
                "Spending time in a bathtub".to_string(),
                "Watching mobile and social media".to_string(),
            ],
            context: r#"Regular, daily practice of deep relaxation techniques, such as meditation, yoga, or progressive muscle relaxation, can help individuals manage anxiety, phobias, and panic attacks by reducing stress and promoting relaxation. The other options are not effective foundation programs for overcoming anxiety, phobias, or panic attacks. Many of the other skills such as desensitization, visualization, and changing negative self-talk, build on the capacity to achieve deep relaxation."#
                .to_string(),
        };
    let how_often_deep_relaxation_technique = McqData {
            question: "How often does one have to practice deep relaxation technique?".to_string(),
            correct_answer: "Regularly, daily, 20-30 minutes".to_string(),
            options: vec![
                "Weekly 1 hours".to_string(),
                "Monthly 2 hours".to_string(),
            ],
            context: r#"To overcome anxiety, phobias, or panic attacks, it is recommended to practice deep relaxation techniques regularly, ideally on a daily basis. This consistent practice helps to reduce stress and anxiety, and promotes overall well-being."#
                .to_string(),
        };
    let deep_relaxation_refer_to = McqData {
            question: "What does deep relaxation refer to?".to_string(),
            correct_answer: "A distinct physiological state that is the exact opposite of the way your body reacts under stress or during a panic attack".to_string(),
            options: vec![
                "A state of high alertness and tension".to_string(),
                "A state of emotional numbness".to_string(),
                "A state of moderate physical activity".to_string(),
            ],
            context: r#""#
                .to_string(),
        };
    let physiological_changes_decreases = McqData {
            question: "What physiological changes does the deep relaxation technique bring about? It increases or decreases the heart rate, respiration rate, blood pressure, skeletal muscle tension, metabolic rate, and oxygen consumption, and affects analytical thinking?".to_string(),
            correct_answer: "Decreases".to_string(),
            options: vec![
                "Increases".to_string(),
            ],
            context: r#"Deep relaxation techniques, such as progressive muscle relaxation, meditation, or yoga, can bring about several physiological changes, including:<br/>Decrease in heart rate <br/>Decrease in respiration rate <br/> Decrease in blood pressure <br/> Decrease in skeletal muscle tension <br/>Decrease in metabolic rate <br/>Decrease in oxygen consumption <br/>Reduction in analytical thinking, allowing for a more relaxed and calm state of mind."#
                .to_string(),
        };
    let physiological_changes_increase = McqData {
            question: "Does the deep relaxation technique decrease or increase alpha wave activity in the brain?".to_string(),
            correct_answer: "Increase".to_string(),
            options: vec![
                "Decrease".to_string(),
            ],
            context: r#"The deep relaxation technique increases alpha wave activity in the brain. Alpha waves are a type of brain wave that is typically associated with relaxation, calmness, and reduced cortical activity."#
                .to_string(),
        };

    let benefits_deep_relaxtion = McqData {
        question: r#"What of these is NOT the benefits of the deep relaxation technique?"#.to_string(),
        correct_answer: "Increase your height and change your skin color".to_string(),
        options: vec![
            "Reduction of generalized anxiety.".to_string(),
            "Increased energy level and productivity.".to_string(),
            "Improved concentration and memory. Regular practice of deep relaxation tends to increase your ability to focus and keeps your mind from “racing.”".to_string(),
            "Reduction of insomnia and fatigue".to_string(),
            "Prevention and/or reduction of psychosomatic disorders, such as hypertension, migraines, headaches, asthma, and ulcers.".to_string(),
            "Increased self-confidence and reduced self-blame. For many people, stress and excessive self-criticism or feelings of           inadequacy go hand in hand. You can perform better, as well as feel better, when you are relaxed.".to_string(),
        ],
        context: r#""#.to_string(),
    };

    view! {
        <div>
            <Nav/>
            <div class="container mx-auto">
                <Mcq data=foundation_program/>
                <Mcq data=how_often_deep_relaxation_technique/>
                <Mcq data=deep_relaxation_refer_to/>
                <Mcq data=physiological_changes_decreases/>
                <Mcq data=physiological_changes_increase/>
                <Mcq data=benefits_deep_relaxtion/>
                <br/>
                <h2 class="text-xl dark:text-white">"Breathing Exercises"</h2>
                <iframe
                    width="560"
                    height="315"
                    src="https://www.youtube.com/embed/OXjlR4mXxSk?si=nhnSq0c4loTVwcQ5"
                    title="YouTube video player"
                    frameborder="0"
                    allow="accelerometer; autoplay; clipboard-write; encrypted-media; gyroscope; picture-in-picture; web-share"
                    referrerpolicy="strict-origin-when-cross-origin"
                    allowfullscreen
                ></iframe>
                <br/>
                <h2 class="text-xl dark:text-white">"Progressive Muscle Relaxation"</h2>
                <iframe
                    width="560"
                    height="315"
                    src="https://www.youtube.com/embed/aF4H0oAiDSQ?si=JY_HFevKvBjHhHHk"
                    title="YouTube video player"
                    frameborder="0"
                    allow="accelerometer; autoplay; clipboard-write; encrypted-media; gyroscope; picture-in-picture; web-share"
                    referrerpolicy="strict-origin-when-cross-origin"
                    allowfullscreen
                ></iframe>
                <br/>
                <h2 class="text-xl dark:text-white">"5-Minute Meditation You Can Do Anywhere"</h2>
                <iframe
                    width="560"
                    height="315"
                    src="https://www.youtube.com/embed/inpok4MKVLM?si=gTSVEV8rtLETH27e"
                    title="YouTube video player"
                    frameborder="0"
                    allow="accelerometer; autoplay; clipboard-write; encrypted-media; gyroscope; picture-in-picture; web-share"
                    referrerpolicy="strict-origin-when-cross-origin"
                    allowfullscreen
                ></iframe>
            </div>
        </div>
    }
}
