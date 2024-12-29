use crate::components::navigation::nav::Nav;
use crate::components::{mcq_struct::McqData, mcqs::Mcq};
use crate::components::{
    mcq_struct::McqDataMultipleSelect, mcqs_multiple_select::McqMultipleSelect,
};
use leptos::prelude::*;

#[component]
pub fn BeingAssertive() -> impl IntoView {
    let assertive_defination = McqDataMultipleSelect {
        question: "Assertiveness is an attitude and a way of acting in any situation where you need to".to_string(),
        correct_answers: vec!["Express your feelings".to_string(),"Ask for what you want".to_string(), "Say no to something you don't want".to_string()],
        options: vec![
            "Don't Express your feelings".to_string(),
            "Don't ask what you want".to_string(),
        ],
        context: r#"Becoming assertive involves self-awareness and knowing what you want. Behind this knowledge is the belief that you have the right to ask for what you want. When you are assertive, you are conscious of your basic rights as a human being. You give yourself and your particular needs the same respect and dignity you'd give anyone else's. Acting assertively is a way of developing self-respect and self-worth"#
            .to_string(),
    };

    let assertiveness_balance = McqData {
        question:
            "Assertiveness is best described as a way of acting that strikes a balance between:"
                .to_string(),
        correct_answer: "Aggressiveness and submissiveness".to_string(),
        options: vec![
            "Confidence and arrogance".to_string(),
            "Passivity and timidity".to_string(),
            "Authority and obedience".to_string(),
        ],
        context: r#""#.to_string(),
    };

    let submissive_behavior = McqData {
        question: "What is a characteristic of nonassertive or submissive behavior?".to_string(),
        correct_answer: "Yielding to someone else's preferences while discounting one's own rights and needs".to_string(),
        options: vec![
            "Asserting one's own rights and needs while respecting others".to_string(),
            "Ignoring others' preferences in favor of one's own".to_string(),
            "Focusing on finding compromises to meet everyone's needs".to_string(),
        ],
        context: r#"Nonassertive or submissive behavior involves yielding to someone else's preferences while discounting your own rights and needs. You don't express your feelings or let others know what you want. Phobic and anxiety-prone people are often submissive because, as previously mentioned, they are overly invested in being “nice” or “pleasing” to everybody. Or they may be afraid that the open expression of their needs will alienate a spouse or partner on whom they feel dependent."#
            .to_string(),
    };

    let aggressive_behavior = McqData {
        question: "Which of the following best describes aggressive behavior?".to_string(),
        correct_answer: "Using coercion or intimidation to achieve one's goals, often disregarding others' rights and feelings".to_string(),
        options: vec![
            "Communicating in a respectful and empathetic way to maintain harmony".to_string(),
            "Balancing personal needs with consideration for others' feelings and rights".to_string(),
            "Avoiding conflict by suppressing one’s desires and yielding to others".to_string(),
        ],
        context: r#"Aggressive behavior, on the other hand, may involve communicating in a demanding, abrasive, or even hostile way with others. Aggressive people typically are insensitive to others' rights and feelings and will attempt to obtain what they want through coercion or intimidation. Aggressiveness succeeds by sheer force, creating enemies and conflict along the way. It often puts others on the defensive, leading them to withdraw or fight back rather than cooperate. For example, an aggressive way of telling someone you want a particular assignment at work would be to say, “That assignment has my name written on it. If you so much as look at the boss when she brings it up during the staff meeting, you're going to regret it.”"#
            .to_string(),
    };

    let passive_aggression = McqDataMultipleSelect {
        question: "Passive-aggressive behavior is expressing anger or frustration through passive resistance and indirect actions. Which of these are examples of passive aggression?".to_string(),
        correct_answers: vec!["You're angry at your boss, so you're perpetually late to work".to_string(),"You don't want to comply with your spouse's request, so you procrastinate or “forget” about the request altogether".to_string()],
        options: vec![
            "You openly tell your boss you are unhappy with their decision".to_string(),
            "You directly ask your spouse for help with a household task".to_string(),
        ],
        context: r#"As an alternative to being openly aggressive, many people are passive- aggressive. If this is your style, instead of openly confronting an issue, you express angry, aggressive feelings in a covert fashion through passive resistance. You're angry at your boss, so you're perpetually late to work. You don't want to comply with your spouse's request, so you procrastinate or “forget” about the request altogether. Instead of asking for or doing something about what you really want, you perpetually complain or moan about what is lacking. Passive- aggressive people seldom get what they want because they never get it across. Their behavior tends to leave other people angry, confused, and resentful. A passive-aggressive way of asking for a particular assignment at work might be to point out how inappropriate someone else is for the job or to say to a coworker, “If I got more interesting assignments, I might be able to get somewhere in this organization.”"#
            .to_string(),
    };

    let manipulative_behavior = McqData {
        question: "What is a key characteristic of manipulative behavior?".to_string(),
        correct_answer: "Using guilt or pity to influence others and playing the role of victim or martyr".to_string(),
        options: vec![
            "Taking responsibility for one's own needs and openly expressing them".to_string(),
            "Being direct and honest in communication to achieve desired outcomes".to_string(),
            "Relying on logical persuasion to convince others of a point".to_string(),
        ],
        context: r#"A final nonassertive behavior style is being manipulative. Manipulative people attempt to get what they want by making others feel sorry for or guilty toward them. Instead of taking responsibility for meeting their own needs, they play the role of victim or martyr in an effort to get others to take care of them. When this doesn't work, they may become openly angry or feign indifference. Manipulation only works as long as those at whom it is targeted fail to recognize what is happening. The person being manipulated may feel confused or “crazy” up to this point; afterward they become angry and resentful toward the manipulator. A manipulative way of asking for a particular assignment at work would be to tell your boss, “Gee, if I get that assignment, I think my boyfriend will finally have some respect for me,” or to tell a coworker, “Don't breathe a word about this—but if I don't get that assignment, I'm going to finally use those sleeping pills I've been saving up.”"#
            .to_string(),
    };
    view! {
        <div>
            <Nav/>
            <div class="container mx-auto">
                <McqMultipleSelect data=assertive_defination/>
                <Mcq data=assertiveness_balance/>
                <Mcq data=submissive_behavior/>
                <Mcq data=aggressive_behavior/>
                <McqMultipleSelect data=passive_aggression/>
                <Mcq data=manipulative_behavior/>
                <section class="bg-white dark:bg-slate-800 py-12 rounded px-4">
                    <p class="text-gray-800 mb-4  dark:text-white">
                        "Learning to be assertive involves working on yourself in six distinct areas:"
                    </p>
                    <p class="text-gray-800 mb-4  dark:text-white">
                        "1. Developing nonverbal assertive behaviors"
                    </p>
                    <p class="text-gray-800 mb-4  dark:text-white">
                        "2. Recognizing and being willing to exercise your basic rights as a human being"
                    </p>
                    <p class="text-gray-800 mb-4  dark:text-white">
                        "3. Becoming aware of your own unique feelings, needs, and wants"
                    </p>
                    <p class="text-gray-800 mb-4  dark:text-white">
                        "4. Practicing assertive responses—first through writing and role- playing and then in real life"
                    </p>
                    <p class="text-gray-800 mb-4  dark:text-white">
                        "5. Assertiveness on the spot"
                    </p>
                    <p class="text-gray-800 mb-4  dark:text-white">"6. Learning to say no"</p>
                </section>
            </div>
        </div>
    }
}
