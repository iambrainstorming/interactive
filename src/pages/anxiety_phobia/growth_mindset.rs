use crate::components::navigation::nav::Nav;
use crate::components::{mcq_struct::McqData, mcqs::Mcq};
use leptos::prelude::*;

#[component]
pub fn GrowthMindset() -> impl IntoView {
    let growth_mindset_def = McqData {
        question: "What a growth mindset is?".to_string(),
        correct_answer: "It a belief that qualities can change and abilities and intelligence can be developed through dedication, hard work, and persistence. ".to_string(),
        options: vec!["It is about working hard.".to_string(), "It's about maintaining high expectations".to_string(), r#"It's about being resilient. Its about person's ability to bounce back from adversity, trauma, or stress, and to maintain their stability and composure in the face of challenges."#.to_string(), "It is the belief that intelligence and abilities cannot be developed.".to_string()],
        context: r#""#.to_string(),
    };

    let foster_growth_mindset = McqData {
        question: "How to foster a growth mindset?".to_string(),
        correct_answer: "It’s not just about praising effort. You also need to learn skills that let you use your brain in a smarter way. . .   to get better at something.".to_string(),
        options: vec!["telling children that they’re smart and implying that their success depends on it".to_string(), "we should simply praise children for working hard.".to_string(), "exhorting students to work hard to directly change behaviors without changing the underlying belief about the nature of abilities".to_string()],
        context: r#""#.to_string(),
    };

    let grit = McqData {
        question: "What is considered the most important factor for success?".to_string(),
        correct_answer: "Grit: courage and determination despite difficulty".to_string(),
        options: vec![
            "Intelligence".to_string(),
            "Both equally".to_string(),
            "Neither".to_string(),
        ],
        context: r#"Grit: The Power of Passion and Perseverance | Angela Lee Duckworth <br/> <br/> <iframe width="560" height="315" src="https://www.youtube.com/embed/H14bBuluwB8?si=Ozt6iMNsI0QkF9th" title="YouTube video player" frameborder="0" allow="accelerometer; autoplay; clipboard-write; encrypted-media; gyroscope; picture-in-picture; web-share" referrerpolicy="strict-origin-when-cross-origin" allowfullscreen></iframe>"#.to_string(),
    };

    let grit_intelligence = McqData {
        question: "What relationship was found between grit and intelligence according to some research?".to_string(),
        correct_answer: "Grit was either orthogonal/unrelated to or slightly inversely correlated with intelligence.".to_string(),
        options: vec!["Grit was strongly positively correlated with intelligence.".to_string(), "Grit was slightly positively correlated with intelligence.".to_string(), "Grit and intelligence are unrelated.".to_string()],
        context: r#""#.to_string(),
    };

    let neuroplasticity = McqData {
        question: "What is neuroplasticity?".to_string(),
        correct_answer:
            "It is a process that involves adaptive structural and functional changes to the brain"
                .to_string(),
        options: vec![
            "A chemical in the brain responsible for memory formation.".to_string(),
            "A type of genetic mutation affecting neural pathways.".to_string(),
            "A fixed structure in the brain that prevents change.".to_string(),
        ],
        context: r#""#.to_string(),
    };

    let earlier_belief_neuroplasticity = McqData {
        question: "What was the earlier belief about neuroplasticity?".to_string(),
        correct_answer: "It manifests only during childhood.".to_string(),
        options: vec![
            "It occurs only in adulthood.".to_string(),
            "It does not exist in humans.".to_string(),
            " It is constant throughout life.".to_string(),
        ],
        context: r#""#.to_string(),
    };

    let recent_neuroplasticity = McqData {
        question:
            "What did research in the latter half of the 20th century reveal about neuroplasticity?"
                .to_string(),
        correct_answer: "Many aspects of the brain can change even in adulthood.".to_string(),
        options: vec![
            "Neuroplasticity is fixed and unchangeable.".to_string(),
            r#"The brain is not "plastic" after childhood."#.to_string(),
            "Neuroplasticity is only a theoretical concept.".to_string(),
        ],
        context: r#""#.to_string(),
    };

    let brain_training = McqData {
            question: "What is the result of training on the brain areas and neural pathways involved in performing a given task?".to_string(),
            correct_answer: "Performance on the task improves, and it eventually becomes automatized and effortless.".to_string(),
            options: vec![
                "The brain areas become more rigid, limiting adaptability.".to_string(),
                "The task becomes more challenging over time due to neural fatigue.".to_string(),
                "Training has no significant impact on neural pathways.".to_string(),
            ],
            context: r#"A series of studies performed on London taxi drivers over the past 15 years provides yet more evidence that mental training can indeed induce anatomical changes to the brain. To qualify as a licensed London taxi driver, trainees undergo years of comprehensive memory training to learn the labyrinthine layout of some 26,000 streets within a six-mile radius of Charing Cross station, the location of thousands of landmarks, and also the quickest way to navigate between any two points in the city. <br/>Prospective taxi drivers typically spend three to four years studying maps and driving around the city, in order to acquire “the Knowledge” of London’s streets. During this time, they also take a set of stringent examinations designed to test their spatial learning of each city district, and are allowed a limited number of attempts at each before progressing on to the next. Only after successfully completing all of these examinations can they qualify and obtain a license to operate one of London’s famous black taxis, and approximately half of those who begin the training fail their examinations or drop out at some point.<br/>In 2000, researchers at University College London published a study <b>showing that gray matter density in the posterior hippocampus is significantly larger</b> in qualified London taxi drivers than in controls. This brain structure is known to be involved in spatial navigation, and the study also showed that its size was closely correlated with the amount of time spent as a taxi driver—the more experienced the driver, the larger was their posterior hippocampus<br/> <cite> - Neuroplasticity (MIT Press Essential Knowledge Series)</cite>"#
                .to_string(),
        };

    let crispr_baby = McqData {
            question: r#"Why is labeling children as "gifted" considered problematic in the context of human genetics? What did we learn from the CRISPR baby scandal?"#.to_string(),
            correct_answer: "Human genetics is far more complex, and it operates on the principle of trade-offs. While an individual may excel in one area, this often comes at the expense of other important traits.".to_string(),
            options: vec![
                "It ensures better educational outcomes for all".to_string(),
                "It highlights the unique abilities of every individual".to_string(),
                "It eliminates biases in education and society".to_string(),
            ],
            context: r#"Gifted Children and CRISPR-baby scandal<br/>Using terms like "gifted children" and separating them from others is a form of pseudoscience that bears an uncomfortable resemblance to eugenics and <a href="https://www.nature.com/articles/d41586-019-00673-1">"CRISPR-baby scandal"</a>. This approach can be seen as a form of racism, as it implies that some individuals are inherently superior to others based on their genetic makeup. However, human genetics is far more complex, and it operates on the principle of trade-offs. While an individual may excel in one area, this often comes at the expense of other important traits. In other words, being exceptionally good at one thing may be offset by weaknesses in other areas. By labeling children as "gifted," we are oversimplifying the complexities of human genetics and neglecting the fact that every individual has a unique set of strengths and weaknesses.<br/><br/> He claims to have disabled a gene called CCR5, which encodes a protein that allows HIV to enter cells. He was aiming to mimic a mutation that exists in about 10% of Europeans, and helps to protect them from HIV infection. But He might have inadvertently caused mutations in other parts of the genome, which could have unpredictable health consequences. (He claims to have found no such mutations.) Also, CCR5 is thought to help people fight off the effects of various other infections, such as West Nile virus. If the gene is disabled, the girls could be vulnerable. If they do suffer in a way that is linked to He’s procedure, and He is found to have been practising medicine illegally, he could be sentenced to between three and ten years in prison, says Zhang Peng, a criminal-law scholar at Beijing Wuzi University. But identifying those health effects could take years."#
                .to_string(),
        };
    let autism_spectrum = McqData {
        question: "Autism is a neurological and developmental disorder that affects how people interact with others, communicate, learn, and behave. How does autism manifest, and what does it imply about the severity and support needs of individuals?".to_string(),
        correct_answer: "Autism manifests in diverse ways, with its severity and support needs varying widely across the spectrum.".to_string(),
        options: vec!["Autism manifests as a uniform condition where all individuals share the same severity level and support needs.".to_string(), "Autism is primarily defined by severe symptoms, with minimal variation in how it affects individuals.".to_string(), "Autism implies a fixed level of support needs, determined by universal criteria applied to all autistic individuals.".to_string()],
        context: r#"Autism is a spectrum condition and affects people in different ways. Like all people, autistic people have their own strengths and weaknesses."#.to_string(),
    };
    let dyslexia = McqData {
        question: "Dyslexia, is a learning disability that affects either reading or writing. Can people with dyslexia be intelligent or smart?".to_string(),
        correct_answer: "Dyslexia occurs at all levels of intelligence—average, above average and highly intelligence.".to_string(),
        options: vec!["No, dyslexia is associated with low intelligence".to_string(), "All people with dyslexia have average intelligence.".to_string(), "Dyslexia is associated with very high intelligence.".to_string()],
        context: r#""#.to_string(),
    };
    view! {
        <div>
            <Nav/>
            <div class="container mx-auto">
                <Mcq data=growth_mindset_def/>

                <Mcq data=foster_growth_mindset/>
                <Mcq data=grit/>

                <Mcq data=grit_intelligence/>
                <Mcq data=neuroplasticity/>

                <Mcq data=earlier_belief_neuroplasticity/>
                <Mcq data=recent_neuroplasticity/>
                <Mcq data=brain_training/>

                <Mcq data=crispr_baby/>

                <Mcq data=autism_spectrum/>

                <Mcq data=dyslexia/>

            </div>
        </div>
    }
}
