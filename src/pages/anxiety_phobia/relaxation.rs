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

    let immediate_benefits = McqData {
            question: "When do participants typically experience initial benefits from mediation or deep relaxation sessions?".to_string(),
            correct_answer: "Immediately after the session".to_string(),
            options: vec![
                "After a week".to_string(),
                "After a month".to_string(),
                "After six months".to_string(),
            ],
            context: r#"With mediation session participants may experience initial benefits during or immediately after the session. These benefits can include:<br/> Clarity on issues and concerns <br/>Improved communication and understanding between parties<br/>Identification of common goals and interests<br/>Reduced tension and conflict"#
                .to_string(),
        };

    let downtime_time_management = McqData {
            question: "Match the words with their definitions <br/><br/>1) Downtime 2)Rest time 3)Recreation time 4)Relationship time<br/><br/> A) Is time when you put aside your private goals and responsibilities in order to enjoy being with another person—or, in some cases, with several people. The focus of relationship time is to honor your relationship with your partner, children, extended family members, friends, pets, and so on, and forget about your individual pursuits for a while. If you have a family,- relationship time needs to be allocated equitably between time alone with your spouse, time alone with your children, and time when the entire family gets together. If you’re single with a partner, time needs to be judiciously allocated between time with your partner and time with friends.<br/> <br/>B) Is time when you set aside all activities and just allow yourself to be. You stop action and let yourself fully rest. Rest time might involve lying on the couch and doing nothing, quietly meditating, sitting in your recliner and listening to peaceful music, soaking in a Jacuzzi, or taking a catnap in the middle of the workday. The key to rest time is that it is fundamentally passive—you allow yourself to stop doing and accomplishing and just be. Contemporary society encourages each of us to be productive and always accomplish more and more every moment of the waking day. Rest time is a needed counterpoint. When you’re under stress, one hour of rest time per day, separate from the time you sleep, is optimal.<br/><br/>C) Involves engaging in activities that help to “re-create” you— that is, serve to replenish your energy. Recreation time brightens and uplifts your spirits. In essence, it is doing anything that you experience as fun or play. Examples of such activities might include puttering in the garden, reading a novel, seeing a special movie, going on a hike, playing soccer, taking a short trip, baking a loaf of bread, or fishing. Recreation time can be done during the workweek and is most important to have on your days off from work.<br/><br/> D)Time out from work or other responsibilities to give yourself an opportunity to rest and replenish your energy. Without periods of downtime, any stress you experience while dealing with work or other responsibilities tends to become cumulative. It keeps building without any remission. You may tend to keep pushing yourself until finally you drop from exhaustion or experience an aggravation of your anxiety or phobias. Sleep at night doesn’t really count as downtime. If you go to bed feeling stressed, you may sleep for eight hours and still wake up feeling tense, tired, and stressed. Downtime needs to be scheduled during the day, apart from sleep. Its primary purpose is simply to allow a break in the stress cycle—to ​prevent stress you’re experiencing from becoming cumulative.
".to_string(),
            correct_answer: "1)-D, 2)-B, 3)-C, 4)-A".to_string(),
            options: vec![
                "1)-A, 2)-B, 3)-C, 4)-D".to_string(),
                "1)-D, 2)-C, 3)-B, 4)-A".to_string(),
                "1)-A, 2)-D, 3)-B, 4)-C".to_string(),
            ],
            context: r#""#
                .to_string(),
        };

    let time_management = McqData {
        question: "Match the words of time management with their definitions <br/><br/>1) PRIORITIZATION 2)DELEGATION 3)ALLOWING EXTRA TIME 4)LETTING GO OF PERFECTIONISM 5) OVERCOMING PROCRASTINATION 6) SAYING NO<br/><br/> A) Skill in delegation means being willing to let someone else take care of a task or activity that has lower priority for you or is an important task that you don’t have to do personally. By delegating, you free up more time for those tasks that are essential and require your personal attention. Often delegation means paying someone else to do what you might do yourself if you had unlimited time: housecleaning, car washing, cooking, child care, basic repairs, and so on. At other times, delegation simply means distributing tasks equitably among family members: your spouse and the kids do their fair share of household chores. A key to delegation is a willingness to trust and rely on others’ capabilities. Give up the idea that only you can do an adequate job, and be willing to entrust responsibility for a task to someone else.<br/><br/>B) A common problem in time management is underestimating the amount of time required to complete a task. The result is that you end up rushing to try to get something done, or else run into overtime and encroach on time that was needed for the next activity in your schedule. As a general rule, it helps to allow a little more time than you would expect for each activity during the day. It’s better to err in favor of overestimating the time required for a task, leaving yourself plenty of time to proceed in a leisurely manner to the next activity. An important prerequisite for allowing extra time is to be willing to do fewer things—not to cram as many tasks or activities into a given time frame<br/><br/>C) means learning to discriminate between tasks or activities that are essential and those that are nonessential. You attend to what’s most important and put everything else on hold (or delegate tasks to other people—see below). You may find it useful to divide your daily tasks and responsibilities into three categories: essential, important, and less important or trivial. Essential tasks or activities include those that require immediate attention: they are absolutely necessary—such as getting the kids off to school. Alternatively, they can be activities that are very important to you—such as physical exercise, if you’re working on reducing your anxiety. Important tasks and activities are those that have ​sig​nificant value but can be delayed for a limited time, such as spending quality one-on-one time with your spouse or partner. Important tasks cannot be delayed for a long time, however. Less important or trivial tasks can be postponed a long time without serious risk or can be delegated to others (tasks such as taking the stack of newspapers in the garage to the recycling center or ​- deleting photos you don’t want to keep on your computer)<br/><br/>D) It essentially means setting your standards and expectations too high: there is no allowance for the inevitable mistakes, frustration, delays, and limitations that come up in the ​process of working toward any goal. It can keep you on a treadmill of overwork or overdedication, to the point that you don’t allow time out for your own needs. Letting go of it requires a fundamental attitude shift. It becomes all right simply to do your best, to make some mistakes along the way, and to accept the results you get, even if your best efforts fall short. It also involves learning to laugh on occasion rather than despair at the limitations inherent to human existence.<br/><br/>E) There are many reasons why people have difficulty saying no. You may always want to be pleasing and responsive to family and friends, no matter what they ask of you, so you have ​difficulty setting limits, even when their demands or needs become more than you can handle. Or you may be so bound up with your work that it’s your primary source of identity and meaning. No matter how demanding and time-consuming work responsibilities become, you keep taking them on, because not to do so would leave you feeling empty. In short, difficulty saying no is usually tied up with your self-image. If your image of yourself requires you to be nice all the time and always available to everyone, then there is probably no limit to what others will ask of you. If your work is who you are, then it will be hard for you to say no to work demands in order to make time for your personal needs.<br/><br/>F) Is is always self-defeating when you leave yourself too little time. Whether preparing for an exam or preparing to go to work, putting off the inevitable leaves you harried and stressed in the end. One reason for procrastinating can be that you really don’t want to do whatever it is that needs doing in the first place. If this is your reason for stalling, the solution lies either in delegating or in prioritizing. If you can delegate an undesirable task to someone else, then by all means do so. If you can’t, then get the undesirable task done first—in other words, prioritize it over the other things you need to do. Promising yourself to do something fun or interesting afterward as a reward for getting the undesirable task done often works well. In overcoming procrastination, the carrot usually works much better than the stick. Another reason for procrastinating is perfectionism. If you feel that something has to be done perfectly, you may keep postponing getting started because you fear that you can’t do it “just right.” The solution here is to jump in and get started, whether or not you feel you’re ready to do it right. An important principle to remember is that motivation often follows behavior. Just getting started on a task will often generate the motivation to follow through and complete it. Then you may have enough time left over to go back and rework or refine what you did during the first round. If you keep stalling, however, you can use up all the time needed to do the kind of job you’d like to do. The worst outcome is when you don’t attempt the task at all because of your impossibly high standards."
            .to_string(),
        correct_answer: "1)-C, 2)-A, 3)-B, 4)-D, 5)-F, 6)-E".to_string(),
        options: vec!["1)-E, 2)-A, 3)-B, 4)-D, 5)-F, 6)-C".to_string(), "1)-E, 2)-D, 3)-B, 4)-A, 5)-F, 6)-C".to_string()],
        context: r#""#.to_string(),
    };

    let procrastination = McqData {
            question: "What is one of the reasons for procrastination? Procrastination means putting off or delaying or defering an action to a later time <br/>A) Lack of motivation, which can be caused by a variety of factors, including a lack of interest in the task, inadequate rewards or recognition, or a general feeling of burnout and exhaustion, leading to a decrease in productivity and a tendency to put off tasks until the last minute.<br/> B) Perfectionism, which involves setting unrealistically high standards for oneself and fearing that one's work will not meet those standards, leading to a delay or avoidance of the task altogether.<br/>C) Poor time management, which can result from a lack of planning, prioritization, and organization, causing individuals to misallocate their time and energy, leading to a backlog of tasks and a sense of overwhelm, ultimately resulting in procrastination.<br/>D) Insufficient resources, such as a lack of access to necessary tools, equipment, or information, which can hinder an individual's ability to complete a task, leading to frustration and a tendency to put off the task until the necessary resources become available.".to_string(),
            correct_answer: "B) and C)".to_string(),
            options: vec![
                "B) and D)".to_string(),
                "A) and C)".to_string(),
            ],
            context: r#""#
                .to_string(),
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
                 <Mcq data=immediate_benefits/>
                <br/>
                <br/>
                <h2 class="text-xl dark:text-white">"Breathing Exercises"</h2>
                <iframe width="560" height="315" src="https://www.youtube.com/embed/cvflhGzINJ4?si=y-xNJQn9xFw3QQoA" title="YouTube video player" frameborder="0" allow="accelerometer; autoplay; clipboard-write; encrypted-media; gyroscope; picture-in-picture; web-share" referrerpolicy="strict-origin-when-cross-origin" allowfullscreen></iframe>
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
                <h2 class="text-xl dark:text-white">"Guided Imagery Exercise"</h2>
                <iframe width="560" height="315" src="https://www.youtube.com/embed/g4iHKY2p5bY?si=14hGJKYnZCcn_J4x" title="YouTube video player" frameborder="0" allow="accelerometer; autoplay; clipboard-write; encrypted-media; gyroscope; picture-in-picture; web-share" referrerpolicy="strict-origin-when-cross-origin" allowfullscreen></iframe>
                <br/>
                <h2 class="text-xl dark:text-white">"Grounding Exercise Outdoors"</h2>
                 <iframe width="560" height="315" src="https://www.youtube.com/embed/3HZ7JkMJceA?si=fheCUc9z8sJzyxV0" title="YouTube video player" frameborder="0" allow="accelerometer; autoplay; clipboard-write; encrypted-media; gyroscope; picture-in-picture; web-share" referrerpolicy="strict-origin-when-cross-origin" allowfullscreen></iframe>
                <br/>
               <h2 class="text-xl dark:text-white">"Grounding Exercise Indoors"</h2>
               <iframe width="560" height="315" src="https://www.youtube.com/embed/KwUBCKs35bU?si=gzrGb6Nj_1Ju3ecF" title="YouTube video player" frameborder="0" allow="accelerometer; autoplay; clipboard-write; encrypted-media; gyroscope; picture-in-picture; web-share" referrerpolicy="strict-origin-when-cross-origin" allowfullscreen></iframe>
                 <br/>
                 <br/>
                <Mcq data=downtime_time_management/>
                 <Mcq data=time_management/>
                 <Mcq data=procrastination/>
                <br/>

            </div>
        </div>
    }
}
