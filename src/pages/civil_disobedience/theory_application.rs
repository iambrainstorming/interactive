use crate::components::{mcq_struct::McqData, mcqs::Mcq, navigation::nav::Nav};
use leptos::prelude::*;

#[component]
pub fn TheoryApplication() -> impl IntoView {
    let what_is_civil_disobedience = McqData {
        question: "What is civil disobedience?".to_string(),
        correct_answer: "Civil disobedience is the active, and professed refusal of a citizen to obey certain laws, demands, orders or commands of a government (or any other authority)".to_string(),
        options: vec!["A form of passive acceptance of government rules and regulations.".to_string(), "A means of seeking revenge against the government".to_string(), "A form of diplomacy to resolve conflicts between government and people.".to_string()],
        context: r#""#.to_string(),
    };

    let why_civil = McqData {
        question: r#"Why is civil disobedience called "Civil"?"#.to_string(),
        correct_answer: r#"Because civil disobedience has to be nonviolent to be called "civil""#
            .to_string(),
        options: vec![
            "Because it is a form of disobedience that is only practiced by civilians".to_string(),
            "Because it involves violent protests and aggressive behavior".to_string(),
            "Because it is a form of disobedience that is only practiced during times of war"
                .to_string(),
        ],
        context: r#""#.to_string(),
    };
    let first_step = McqData {
        question: "What is the first step in a strategic nonviolent struggle?".to_string(),
        correct_answer: "Recognizing the Vision of Tomorrow ".to_string(),
        options: vec![
            "Identifying opponents and forming alliances".to_string(),
            "Planning a nonviolent conflict".to_string(),
            "Defining immediate goals".to_string(),
        ],
        context: r#"Every long journey starts with one small first step. In the case of a strategic nonviolent struggle, this step is the simple and precise answer to the question: <br/><br/>What does your movement want the society to be like when the struggle is over?<br/><br/> The answer to this question, known as the “Vision of Tomorrow”, provides a picture of the future society you are striving towards. Once formulated, the Vision of Tomorrow becomes your movement’s primary objective. It is a permanent guideline for your movement’s supporters. Your strategic nonviolent struggle becomes a journey towards achieving that vision. <br/><br/>That journey, however, is not an easy one. The obstacles on your journey must be foreseen and removed; allies can be approached and convinced to join their efforts with your effort. Also, opponents must be recognized and faced."#.to_string(),
    };
    let vision_of_tomorrow = McqData {
        question: r#"What is the "Vision of Tomorrow" in a strategic nonviolent struggle?"#
            .to_string(),
        correct_answer: "A picture of the future society you are striving towards".to_string(),
        options: vec![
            "A picture of the current state of society".to_string(),
            "A list of immediate goals to be achieved".to_string(),
            "A description of the obstacles to be overcome".to_string(),
        ],
        context: r#""#.to_string(),
    };
    let role_vision = McqData {
        question:
            r#"What is the role of the "Vision of Tomorrow" in a strategic nonviolent struggle?"#
                .to_string(),
        correct_answer: "It becomes the movement's primary objective and guideline"
            .to_string(),
        options: vec![
            "It provides a temporary guideline for the movement's supporters".to_string(),
            "It serves as a means to identify opponents and form alliances".to_string(),
            "It helps to plan a nonviolent conflict".to_string(),
        ],
        context: r#"If you want changes in your society not only to occur, but to progressively continue, then you must see beyond the immediate goals of the movement when planning a nonviolent conflict. <br/>Looking beyond your immediate goals forces you to look not just at what your movement is struggling against, but also what your movement is struggling for. For example, the US Civil Rights Movement did not just struggle against discrimination, it struggled for a more equal and just society. Defining what your movement is struggling for is an important part of formulating your vision."#.to_string(),
    };
    let widely_shared = McqData {
        question: "Why are widely shared objectives important in a movement?".to_string(),
        correct_answer: "They create the potential for more widely distributed risks and reduce the likelihood of a single part of the movement becoming a target".to_string(),
        options: vec!["They increase the likelihood of a single part of the movement becoming a target".to_string(), " They create conflict within the movement".to_string(), "They reduce the potential for widely distributed risks".to_string()],
        context: r#"Most people will struggle and sacrifice only for goals that are concrete and realistic enough to be reasonably at- tainable. Widely shared objectives create the potential for more widely distributed risks and reduce the likelihood that any single part of your movement will become a deci- sive target for your opponent."#.to_string(),
    };
    let realist_goals = McqData {
        question: "Why is it beneficial for a movement to have concrete and realistic goals?"
            .to_string(),
        correct_answer: "It motivates people to struggle and sacrifice for the cause".to_string(),
        options: vec![
            "It makes the movement more vulnerable to opposition".to_string(),
            "It creates unrealistic expectations among supporters".to_string(),
            "It leads to a decrease in morale and motivation among supporters".to_string(),
        ],
        context: r#""#.to_string(),
    };

    let understanding_different_groups = McqData {
        question: "What is the best approach for a nonviolent movement to understand the needs and desires of different groups in society?".to_string(),
        correct_answer: "To listen to people from different groups in society and analyze their values and vision".to_string(),
        options: vec!["To ignore the discrepancies or contradictions among different visions or desires within the population".to_string(), "To focus solely on the movement's own goals and objectives".to_string(), "To develop a top-down approach to decision-making and planning ".to_string()],
        context: r#"The strategist understands the importance of listening to different groups in society and letting the members of each articulate their values and vision. The best place to start is to train your movement’s members to listen to the people in their communities. A movement is not launched by telling people what YOU think is best for them. <br/> Therefore, planners and strategists of nonviolent move- ments should start by listening to people from different groups in society, analyzing what they have said, and findng the commonalities between the visions of the movement and the people. Movement planners and strategists also need to understand the roots of discrepancies or contradictions among different visions or desires within the population. <br/> This process of listening is very important because before people accept and share your movement’s vision, people must be able to articulate and interpret their vision. They must see at least part of their vision in your vision. "#.to_string(),
    };
    let monolithic_model = McqData {
        question:
            "What is the main idea of the monolithic model of power, which is actually NOT TRUE?"
                .to_string(),
        correct_answer: "The person or people at the top can be changed - through a revolution, a war, or a coup - but the model remains the same: whoever gets to the top of the mountain ends up controlling all power in society".to_string(),
        options: vec![
            "That the person or people at the top of the power structure are irrelevant to the functioning of society".to_string(),
            "That the power structure is flexible and can be changed from the bottom up through nonviolent struggle and social movements".to_string(),
            "That the person or people at the top of the power structure have a divine right to rule and cannot be challenged".to_string(),
        ],
        context: r#"The monolithic model describes the system authoritarians want you to perceive as being solid and unmovable like a mountain. This model promotes the idea of a fixed power structure, as if nothing could be changed except the per- son or people at the top. Whoever the person on the top of the power structure is, he/she has power over society. The decisions he/she makes today become a reality for the entire society tomorrow. The person or people at the top can be changed - through a revolution, a war, or a coup - but the model remains the same: whoever gets to the top of the mountain ends up controlling all power in society. You can break small pieces away by attacking this monolith with your actions or campaigns... But the mountain of power is still there. <br/>You can create positive change in society, if you have the right person at the top! - says the regime… Still, there’s only one problem with this model.<br/><br/> THIS MODEL OF POLITICAL POWER IS NOT TRUE! <br/><br/>Power does not function this way. No matter how many times an authoritarian or others tell you that it does, reality teaches us something else."#.to_string(),
    };
    let pluralistic_model = McqData {
        question: "What is the main idea of the pluralistic model of power, which is a true description of the world?".to_string(),
        correct_answer: "Power in society ultimately comes from the people. And those people - each of whom is a small, individual source of political power - can change their minds. Rulers only have that power which people provide to them.".to_string(),
        options: vec!["That power is held by a single person or group at the top of a rigid hierarchy ".to_string(), "That power is fixed and unchanging, and can only be altered through violent revolution".to_string(), "That power is only held by those who are educated or have a high level of expertise".to_string()],
        context: r#"However monolithic or fixed it may appear in the previous model, the nature of power is actually diametrically differ- ent. In a society, power can change very swiftly. It is fragile and dispersed. Wherever people are, the ultimate reality of power is the same. Power in society ultimately comes from the people. And those people - each of whom is a small, individual source of political power - can change their minds. Rulers only have that power which people provide to them. <br/> Power can be given to the ruler willingly, like in democratic societies, or people can be coerced to give it, against their own will, or they can simply be apathetic, and relinquish that power because they don’t care and they don’t think their actions can lead to any change. <br/>This is why nonviolent campaigns are so important: They make people aware that their actions CAN and DO make change. This is especially true when people are unified and act together in nonviolent and strategically coordinated ways."#.to_string(),
    };
    let sources_of_power = McqData {
        question: "What are the sources of power, whose availability to the ruler depends on the continued obedience of the people?".to_string(),
        correct_answer: "All of the Above".to_string(),
        options: vec!["AUTHORITY (or legitimacy) – defined as the position to give orders, combined with people’s belief that those orders are legitimate.".to_string(), "HUMAN RESOURCES – persons and groups that obey, cooperate with, or assist the ruler. Human resourc- es are commonly seen as the people who work in differ- ent institutions that cooperate with the ruler voluntarily or under pressure, spreading and implementing his/her policies over society.".to_string(), "SKILLS AND KNOWLEDGE – needed by the ruler and supplied by cooperating groups and citizens. No ruler can function without a steady supply of skills and knowledge that only experts such as engineers or technicians have, and just as a ruler needs the cooperation of experts to stay in power, an opposition group that attracts them can gain power.".to_string(),"MATERIAL RESOURCES – control of or access to property, natural resources, financial resources, and means of communication and transportation. A ruler’s power is related to the amount of material resources that are under his/her control. For example, rulers know that those who are materially dependent on their regime are less likely to act against them.".to_string(), "INTANGIBLE FACTORS – the group of habits, atti- tudes, traditional values, and psychological, cultural, reli- gious and sometimes ideological factors that may induce people to obey and assist the rulers. These factors usually owe their existence to some combination of religion and culture, or tradition and conventions, such as the tradi- tion of obeying people in uniforms or representatives of religious establishments.".to_string(), "SANCTIONS – the enforcement of obedience through punishments, either threatened or applied, to ensure the submission and cooperation that are needed for a ruler to carry out his/her policies and to maintain control. The fear that sanctions create in others is often more pow- erful than the use of the actual sanctions themselves. Therefore, this source of power is expressed not neces- sarily by the application of the sanctions, but rather by the possibility or threat of sanctions, such as being fired, arrested or physically abused for disobedience.".to_string()],
        context: r#""#.to_string(),
    };
    let heart_of_political_power = McqData {
        question: r#"Why is obedience regarded as the "heart of political power"?"#.to_string(),
        correct_answer: "Because if the people do not obey, the ruler cannot rule".to_string(),
        options: vec![
            "Because it is a fundamental principle of laws of physics".to_string(),
            "Because it is a fundamental principle of international diplomacy ".to_string(),
            "Because it is a necessary condition for maintaining social harmony".to_string(),
        ],
        context: r#""#.to_string(),
    };
    let obey = McqData {
        question: "Why do people tend to obey authority figures and rules?<br/> A) INDIFFERENCE – Many people obey simply because the cost of not doing so seems at that moment to be more trouble than it is worth <br/> B) TRYING TO IMPRESS:  Because they are trying to impress their peers or gain social status <br/> C) HABIT – Since childhood most people have been conditioned to obey authority. This process begins with our parents and grandparents, and then in school our teachers are figures of authority that we obey, as are officers during military training and our superiors at our workplace.<br/> D) ROBOTS: Because they are secretly robots programmed to obey <br/> E) FEAR OF SANCTIONS – A strong reason why people obey is fear of punishment. Violation of a law or rule can result in a variety of sanctions, from paying fines, to harassment, to losing one’s job or position, to losing property, to imprisonment or even execution.<br/> F) MAJORITY DOING IT – Some people behave in a certain way because they see that the majority of people are also behaving that way. Peer and social pressure can be a powerful influence on behavior. If the majority of the people start to behave differently, these people may start to behave differently as well. <br/> G) MAGIC: Because they are under a magical spell that forces them to obey <br/>".to_string(),
        correct_answer: "A, C, E, F".to_string(),
        options: vec!["A, B, C, D".to_string(), "A, C, F, G".to_string(), "B, C, D, E".to_string()],
        context: r#""#.to_string(),
    };
    let question_data = McqData {
        question: "".to_string(),
        correct_answer: "".to_string(),
        options: vec!["".to_string(), "".to_string(), "".to_string()],
        context: r#""#.to_string(),
    };

    view! {
        <div>
            <Nav/>
            <div class="container mx-auto">
            <Mcq data=what_is_civil_disobedience/>
             <Mcq data=why_civil/>
              <Mcq data=first_step/>
             <Mcq data=vision_of_tomorrow/>
             <Mcq data=role_vision/>
             <Mcq data=widely_shared/>
             <Mcq data=realist_goals />
             <Mcq data=understanding_different_groups />
             <Mcq data=monolithic_model />
             <Mcq data=pluralistic_model />
             <Mcq data=sources_of_power />
             <br/><br/>
             <h2 class="text-xl dark:text-white">"Obedience"</h2>
             <br/>
             <Mcq data=heart_of_political_power />
             <Mcq data=obey />
            </div>

        </div>
    }
}
