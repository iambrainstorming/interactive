use crate::components::navigation::nav::Nav;
use crate::components::{mcq_struct::McqData, mcqs::Mcq};
use crate::components::{
    mcq_struct::McqDataMultipleSelect, mcqs_multiple_select::McqMultipleSelect,
};
use leptos::prelude::*;

#[component]
pub fn SelfTalk() -> impl IntoView {
    let self_talk_anxiety = McqData {
            question: "In a scenario where two individuals are stuck in traffic, the one who is more likely to feel anxiety, anger, and frustration:".to_string(),
            correct_answer: r#"Individual with self talk "This is a nightmare", "I can't stand this" or "I'm wasting my life stuck here.""#.to_string(),
            options: vec![
                r#"The individual telling himself as he listens to music, "I can handle this" or "Let's see how long I can last in this traffic," or "Let me relax and do some deep breathing.""#.to_string(),
            ],
            context: r#"Despite the identical circumstances, the two individuals experience vastly different emotional responses due to the distinct self-talk playing out in their minds."#
                .to_string(),
        };

    let external_situation = McqData {
        question: "What determines our mood and feelings in response to a situation?".to_string(),
        correct_answer: "Our internal dialogue and self-talk".to_string(),
        options: vec![
            "The external situation itself".to_string(),
            "The people around us".to_string(),
            "Our physical environment".to_string(),
        ],
        context: r#"The key to our emotional state lies in the inner conversations we have in response to life's situations. These conversations often unfold so quickly and instinctively that we're unaware of them, leading us to believe that external circumstances dictate our emotions."#.to_string(),
    };

    let we_responsible = McqData {
            question: "Are we largely responsible for how you feel?".to_string(),
            correct_answer: "The words we choose to say to ourselves in response to a situation have a profound impact on our emotional state.".to_string(),
            options: vec![
                "External circumstances are the primary cause of how we feel".to_string(),
                "We have no control over our reactions and emotions".to_string(),
                "We are completely at the mercy of our genes and upbringing".to_string(),
            ],
            context: r#"You have the power to shape your emotional state, and it's a vital truth that deserves recognition. However, this reality can be challenging to accept, especially when it's easier to blame external factors for your emotions. By acknowledging and taking responsibility for your reactions, you'll begin to assert control over your life and unlock a more fulfilling existence."#
                .to_string(),
        };

    let worrier = McqDataMultipleSelect {
        question:
            "What is the primary characteristic of The Worrier self-talk? <br/> Select all that apply. "
                .to_string(),
        correct_answers: vec![
            "It creates anxiety by imagining the worst-case scenario".to_string(),
            "Scares you with fantasies of disaster or catastrophe".to_string(),
            "Exaggerating the likelihood of a negative or embarrassing outcome".to_string(),
            "Favorite expression is “what if ...?”".to_string(),
        ],
        options: vec![
            "It promotes feelings of calmness and relaxation".to_string(),
            "It encourages people to take risks and face their fears".to_string(),
        ],
        context: r#""#.to_string(),
    };

    let critic = McqDataMultipleSelect {
        question: "What is the primary characteristic of The Critic self-talk? <br/> Select all that apply. "
            .to_string(),
        correct_answers: vec![
            "It tends to point out your flaws and limitations whenever possible".to_string(),
            "It jumps on any mistake you make to remind you that you’r a failure.".to_string(),
            "Its a part of you that is constantly judging and evaluating your behavior".to_string(),
            "Generates anxiety by putting you down for not being able to handle your panic symptoms, for not being able to go places you used to go, for being unable to perform at your best".to_string(),
        ],
        options: vec![
            "It generates anxiety by encouraging you to take risks and face your fears".to_string(),
            "It is a supportive and encouraging subpersonality".to_string(),
        ],
        context: r#""#.to_string(),
    };

    let victim = McqDataMultipleSelect {
        question: "What is the primary characteristic of The Victim self-talk? <br/> Select all that apply. "
            .to_string(),
        correct_answers: vec!["It generates anxiety by telling you that you’re not making any progress, that your condition is incurable".to_string(), "It says the road is too long and steep for you to have a real chance at recovering".to_string(), "There is something inherently wrong with you: you are in some way deprived, defective, or unworthy".to_string(), "Favorite expressions: I can’t.” “I’ll never be able to.".to_string(), "They say I’ll never be able to do that, so what’s the point in even trying?".to_string()],
        options: vec![" It generates hope and optimism by telling you that you're making good progress and have a strong chance of recover".to_string(), "It emphasizes your strengths and abilities, and encourages you to take action towards recovery".to_string()],
        context: r#""#.to_string(),
    };

    let perfectionist = McqDataMultipleSelect {
            question: "What is the primary characteristic of The Perfectionist self-talk? <br/> Select all that apply. ".to_string(),
            correct_answers: vec![" It  generates anxiety by constantly telling you that your efforts aren’t good enough, that you should be working harder".to_string(),"It tells you should always have everything under control, should always be competent, should always be pleasing, should always be ___________ ".to_string(), "Favorite Expressions: “I should.” “I have to.” “I must.”".to_string(), "wants to be best and is intolerant of mistakes or setbacks".to_string()],
            options: vec![
                "It tells you that it's okay to make mistakes, that they're an important part of the learning process".to_string(),
                "It is a helpful and supportive part of the self, that encourages you to do your best".to_string(),
            ],
            context: r#""#
                .to_string(),
        };
    view! {
        <div>
            <Nav/>
            <div class="container mx-auto">
                <Mcq data=self_talk_anxiety/>
                <Mcq data=external_situation/>
                <Mcq data=we_responsible/>
                <McqMultipleSelect data=worrier/>
                <McqMultipleSelect data=critic/>
                <McqMultipleSelect data=victim/>
                <McqMultipleSelect data=perfectionist/>
                <section class="bg-white dark:bg-slate-800 py-12 rounded px-4">
                    <p class="text-gray-800 mb-4  dark:text-white">
                        "During the next two weeks, monitor the
                        times when you feel anxious or panicky. Each time you do, use the following
                        five steps to work with negative self-talk:"
                    </p>
                    <p class="text-gray-800 mb-4  dark:text-white">
                        "Step 1: If you’re feeling anxious or upset, do something to
                        relax, such as abdominal breathing, progressive muscle
                        relaxation, or meditation. It’s easier to notice your internal
                        dialogue when you take time to slow down and relax."
                    </p>
                    <p class="text-gray-800 mb-4  dark:text-white">
                        "Step 2: After you get somewhat relaxed, ask yourself, “What
                        was I telling myself that made me anxious?” or “What was
                        going through my mind?” Remember to separate thoughts from
                        feelings. For example, “I felt terrified” describes a feeling,
                        while “This panic will never end” is an over​estimating thought
                        that might have led you to feel terrified."
                    </p>
                    <p class="text-gray-800 mb-4  dark:text-white">
                        "Step 3: Identify the three basic types of distortions among your
                        anxious self-talk. Sort out overestimating thoughts,
                        catastrophic thoughts, and thoughts that underestimate your
                        ability to cope."
                    </p>
                    <p class="text-gray-800 mb-4  dark:text-white">
                        "Step 4: When you’ve identified your anxious, distorted
                        thoughts, challenge them with appropriate questions.
                        For overestimating thoughts: “What are the
                        realistic odds that this feared outcome would
                        actually happen?”"
                    </p>
                    <p class="text-gray-800 mb-4  dark:text-white">
                        "For catastrophic thoughts: “If the feared
                        outcome actually did occur, how terrible
                        would it be? Is it really true that I would go to
                        pieces and lose my ability to cope?”
                        For thoughts underestimating your ability to
                        cope: “If the feared outcome did occur, what
                        could I actually do to cope?”"
                    </p>
                    <p class="text-gray-800 mb-4  dark:text-white">
                        "Step 5: Write counterstatements to each of your anxious self-
                        statements. These counterstatements should contain language
                        and logic that reflect more balanced, realistic thinking."
                    </p>
                </section>

            </div>
        </div>
    }
}
