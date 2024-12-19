use crate::components::{mcq_struct::McqData, mcqs::Mcq, navigation::nav::Nav};
use leptos::prelude::*;

#[component]
pub fn CausesOfAnxietyDisorders() -> impl IntoView {
    let strategies_overcoming = McqData {
            question: "What are the various strategies for overcoming anxiety disorders?".to_string(),
            correct_answer: "All of the Above".to_string(),
            options: vec![
                "relaxation".to_string(),
                "exercise".to_string(),
                "changing self-​talk and mistaken beliefs".to_string(),
            ],
            context: r#"The various strategies for overcoming anxiety disorders presented in this workbook—such as relaxation, exercise, desensitization, changing self-​talk and mistaken beliefs, or dealing with feelings —do not depend on a knowledge of underlying causes to be effective. Whether you are dealing with panic attacks, social phobia, generalized anxiety, or obsessive-compulsive disorder, recognize that there is no one cause which, if removed, would eliminate the problem."#
                .to_string(),
        };

    let anxiety_problems = McqData {
            question: "Anxiety problems are brought about by ".to_string(),
            correct_answer: "All of the above".to_string(),
            options: vec![
                "heredity and biology".to_string(),
                "family background and upbringing".to_string(),
                "conditioning".to_string(),
                "recent stressors".to_string(),
                "your self-talk and personal belief".to_string(),
                "your ability to express feelings".to_string()

            ],
            context: r#"Anxiety problems are brought about by a variety of causes operating on numerous different levels: heredity, biology, family ​background and upbringing, conditioning, recent stressors, your self-talk and personal belief - system, your ability to express feelings, and so on"#
                .to_string(),
        };

    let single_cause_theory = McqData {
        question: "What is the primary criticism of \"single-cause\" theories of anxiety disorders?".to_string(),
        correct_answer: "They tend to oversimplify anxiety disorders and are susceptible to mistaken lines of reasoning ".to_string(),
        options: vec!["They are too complex and difficult to understand".to_string(), "They are not supported by empirical evidence".to_string(), "They are only applicable to specific types of anxiety disorders".to_string()],
        context: r#"Some experts in the field of anxiety disorders propose single-cause theories. Such theories tend to greatly oversimplify anxiety disorders and are susceptible to one of two mistaken lines of reasoning: the biological fallacy and the psychological fallacy. The biological fallacy assumes that a particular type of - anxiety disorder is caused solely by some biological or physiological imbalance in the brain or body. For example, there has recently been a tendency to reduce the causation of panic disorder, as well as obsessive-compulsive disorder, to a strictly biological level. Panic disorder is viewed as arising from a dysfunction in parts of the brain, such as the amygdala and the locus coeruleus. Obsessive--compulsive disorder is thought to be caused by a deficiency in a particular neurotransmitter substance in the brain called serotonin—or a dysregulation in the serotonin system of neurons in the brain. (A neurotransmitter is a chemical substance that allows nerve impulses to be transmitted from one nerve cell to another.)"#.to_string(),
    };

    let physiological_disturbances = McqData {
            question: "What is the proposed explanation for the origin of physiological disturbances in anxiety disorders, such as the malfunction of the amygdala and locus coeruleus in panic disorder or the disturbance in brain serotonin levels in obsessive-compulsive disorder?".to_string(),
            correct_answer: "Stress or other psychological factors, such as chronic stress due to psychological conflict or chronically suppressed anger, which may have originated from a person's upbringing".to_string(),
            options: vec![
                "Physiological imbalances that occur randomly and without any underlying cause".to_string(),
                "Physiological disturbances have no connection with social stressors and conflict, and are solely caused by genetic or neurological factors".to_string(),
                "Your upbringing has no role in physiological disturbances".to_string(),
            ],
            context: r#"What caused the physiological disturbance itself? Perhaps chronic stress due to psychological conflict causes the amygdala and locus coeruleus to malfunction in panic disorder. Or perhaps chronically suppressed anger sets up a disturbance in brain serotonin levels that is a contributing cause of obsessive-compulsive disorder. Psychological conflicts and repressed anger may, in turn, have been caused by a person’s upbringing. Because any particular physiological disturbance may have originally been set up by stress or other psychological factors, it is a fallacy to assume that anxiety disorders are solely (or even primarily) caused by physiological imbalances."#
                .to_string(),
        };

    let psychological_fallacy = McqData {
            question: "What is the psychological fallacy? A fallacy is the use of invalid or otherwise faulty reasoning in the construction of an argument.".to_string(),
            correct_answer: "The assumption that anxiety disorders are solely caused by psychological factors, such as a person's upbringing".to_string(),
            options: vec![
                "The assumption that anxiety disorders are caused by a combination of biological and psychological factors".to_string(),
                "The assumption that anxiety disorders are not caused by any specific factors".to_string(),
                "The assumption that anxiety disorders are solely caused by biological factors".to_string(),
            ],
            context: r#"The psychological fallacy makes the same kind of mistake in the opposite direction. It assumes that, say, social phobia or generalized anxiety disorder is caused by having grown up with parents who neglected, abandoned, or abused you, resulting in a deep-seated sense of insecurity or shame that causes your current phobic avoidance and anxiety as an adult. While it may be true that your family background contributed in an important way to your current problems, is it reasonable to assume that this is the only cause? Again, not really. To do so overlooks the possible contributions of hereditary and biological factors. After all, not all children who grow up in dysfunctional families develop anxiety disorders. It is more plausible to assume that your problem is a result of both 1) a hereditary predisposition toward anxiety (and possibly phobia) and 2) early childhood conditions that fostered a sense of shame and/or insecurity."#
                .to_string(),
        };

    let genetic_anxiety = McqData {
            question: "The long term predisposing causes are heredity and childhood circumstances. How do you know heredity plays a role in anxiety disorders?".to_string(),
            correct_answer: "Through studies of identical twins.Identical twins (also called monozygotic twins) result from the fertilization of a single egg by a single sperm, with the fertilized egg then splitting into two.".to_string(),
            options: vec![
                "Through studies of fraternal twins. Fraternal twins (also called dizygotic twins) result from the fertilization of two separate eggs with two different sperm during the same pregnancy".to_string(),
                "Through studies of adoptive families ".to_string(),
                "Through studies of brain chemistry".to_string(),
            ],
            context: r#"Are anxiety disorders inherited? The limited evidence that exists to date would argue that they are—at least in part. For example, it is estimated that 15 to 25 percent of children growing up with at least one agoraphobic parent become agoraphobic themselves, while the rate of agoraphobia in the general population is only 5 percent. This fact in itself doesn’t prove that agoraphobia is inherited,- however, because it could be argued that children learn from their parents to be agoraphobic. More compelling evidence comes from studies of identical twins, who, of course, have exactly the same genetic makeup. If one identical twin has an anxiety disorder, the probability of the other identical twin having an anxiety disorder ranges from 31 to 88 percent, depending on the study you’re looking at."#
                .to_string(),
        };

    let childhood_circumstances = McqData {
            question: "What childhood experiences or familly environments might predispose you to develop a particular anxiety disorder? ".to_string(),
            correct_answer: "All of the Above".to_string(),
            options: vec![
                "Your parents communicate an overly cautious view of the world. When you learn that the outside world is threatening, you automatically restrict your exploration and risk taking. You grow up with a tendency to worry excessively and be overly concerned with safety.".to_string(),
                "Your parents are overly critical and set excessively high standards. Children growing up with critical, perfectionist parents are never quite sure of their own acceptability. There is always some doubt about whether you are “good enough,” or sufficiently worthy".to_string(),
                "Experiences of neglect, rejection, abandonment through divorce or death, domestic voilence and physical or sexual abuse can also produce the kind of basic insecurity (as well as emotional dependency) that forms a background for anxiety disorders.".to_string(),
                "Parents not only may foster dependency but may suppress your innate capacity to express your ​feelings and assert yourself. For example, as a child you may have been continually reprimanded or ​punished for speaking out, acting impulsively, or getting angry.".to_string(),
            ],
            context: r#""#
                .to_string(),
        };

    view! {
        <div>
            <Nav/>
            <div class="container mx-auto">
                <Mcq data=strategies_overcoming/>
                <Mcq data=anxiety_problems/>
                <Mcq data=single_cause_theory/>
                <Mcq data=physiological_disturbances/>
                <Mcq data=psychological_fallacy/>
                <Mcq data=genetic_anxiety/>
                <Mcq data=childhood_circumstances/>
            </div>
        </div>
    }
}
