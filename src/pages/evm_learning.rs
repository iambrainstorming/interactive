use crate::components::{mcq_struct::McqData, mcqs::Mcq, navigation::nav::Nav};
use leptos::prelude::*;

#[component]
pub fn EvmLearning() -> impl IntoView {
    let secrecy_software = McqData {
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

    let ballot_stuffing = McqData {
            question: "Which of the following best describes ballot stuffing?".to_string(),
            correct_answer: "The practice of submitting multiple ballots by a single individual in an election where one ballot per person is allowed.".to_string(),
            options: vec![
                "Ballot harvesting, where individuals collect and submit ballots on behalf of others without their consent.".to_string(),
                "Voter impersonation, where an individual votes using the identity of another eligible voter without their knowledge.".to_string(),
                "Ballot box tampering, where sealed ballot boxes are illegally accessed and ballots are altered or replaced. ".to_string(),
            ],
            context: r#""#
                .to_string(),
        };

    let ballot_stuffing_vvpat = McqData {
        question: "How can ballot stuffing occur in an election that a compromised VVPAT system?".to_string(),
        correct_answer: "VVPAT could print ballots while no voter is observing the paper trail.".to_string(),
        options: vec!["Voters themselves could engage in ballot stuffing by voting multiple times, through fake identity proof.".to_string(), "Voters could exploit a vulnerability in the VVPAT system where it fails to record a vote on EVM but still prints a paper trail, allowing them to cast multiple votes without detection.".to_string(), "Election officials could replace legitimate VVPAT paper trails with fraudulent ones, inflating the vote count for a specific candidate.".to_string()],
        context: r#"VVPAT could print while no voter is observing the paper trail, a form of [ballot stuffing](https://en.wikipedia.org/wiki/Ballot_stuffing). Even if additional votes were discovered through matching to the voters list, it would be impossible to identify legitimate ballots from fraudulent ballots."#.to_string(),
    };

    let vvpat_tallied = McqData {
            question: "Are VVPAT (Voter-Verified Paper Audit Trail) and EVM (Electronic Voting Machine) votes tallied after an election to ensure accuracy?".to_string(),
            correct_answer: "No, only the votes from a random sample of VVPAT machines approximately 1% of VVPAT votes are compared with EVM results.".to_string(),
            options: vec![
                "Yes, all VVPAT and EVM votes are meticulously compared to identify any discrepancies.".to_string(),
                "Yes, but only in close elections where the margin of victory is narrow.".to_string(),
                "No, the VVPAT is solely for voter assurance and has no impact on the official count.".to_string(),
            ],
            context: r#""#
                .to_string(),
        };

    let open_source_evm = McqData {
        question: "Is the EVM (Electronic Voting Machine) system, including its components and software, open source and available for public audit by independent experts?".to_string(),
        correct_answer: "No, the EVM system is proprietary, and only authorized personnel have access to its inner workings.".to_string(),
        options: vec!["Yes, the EVM system is entirely open source, allowing for full transparency and public scrutiny.".to_string(), "The hardware components are open source, but the software remains closed-source and proprietary.".to_string(), "While the software is open source, the hardware specifications are kept confidential.".to_string()],
        context: r#""#.to_string(),
    };

    let security_required_evm = McqData {
        question: "How many days of security does an EVM require?".to_string(),
        correct_answer: "All the time".to_string(),
        options: vec!["60 days before and after election day".to_string(), "30 days before the election".to_string(), "365 days before the election".to_string()],
        context: r#"EVMs demand protection for 365 days every year, whereas ballot papers need protection for only about few months, hardly 3-6 months after the election. Any unprotected EVM at any time is susceptible to hackers manipulating it and installing malware.It's unrealistic to have robust security for EVMs all the time."#.to_string(),
    };
    let discrepancies_2024_lok_sabha = McqData {
        question: "In how many constituency were discrepancies found between the number of voters polled on election day and the number of voters recorded in the EVMs during the vote-counting process in the 2024 Lok Sabha elections?".to_string(),
        correct_answer: "Almost all polling stations".to_string(),
        options: vec!["An insignificant number of polling stations".to_string(), "Less than half of the polling stations".to_string(), "Less than one-third of the polling stations".to_string()],
        context: r#"Data of all discrepancies is here. Only three constituencies have no discrepancies. (Amreli from Gujarat, Attingal from Kerala and Lakshadweep) [Discrepancies EVM votes during polling and evm votes during counting](https://github.com/silicology/election_data_analysis/blob/main/data/voter_turnout_data/evm_turnout_matching/evm_voter_turnout_comparison.json)"#.to_string(),
    };

    let how_many_evm_manipulation_required = McqData {
        question: "Which of the following statements is true regarding the impact of manipulating a single to three Electronic Voting Machine (EVM) in a constituency?".to_string(),
        correct_answer: "It can be significant as many candidates win or lose by a narrow margin under the First Past the Post (FPTP) system.".to_string(),
        options: vec!["It has little impact on the overall results as it is just one to three machine.".to_string(), "The margin of victory is always substantial, so altering a few votes has no real consequence.".to_string(), "The impact depends on the population density of the constituency, with urban areas being less affected.".to_string()],
        context: r#"In many democratic countries that employ the First Past the Post (FPTP) electoral system, elections are often decided by a small number of votes. Candidates frequently win or lose by margins of a few hundred or even fewer votes. In such scenarios, manipulating even a single EVM in a constituency can have a substantial impact. If an EVM is rigged to favor a particular candidate, it could potentially swing the result in their favor, especially in tightly contested races."#.to_string(),
    };

    let paper_ballot_more_secure = McqData {
        question: "Why is a paper ballot considered more secure than an Electronic Voting Machine (EVM) in terms of manipulation? Which of the statements is correct?".to_string(),
        correct_answer: "Evidence of paper ballot manipulation can be captured through CCTV or smartphone camera surveillance, while EVM manipulation occurs internally at the hardware and software levels, making it harder to detect.".to_string(),
        options: vec!["Paper ballots cannot be tampered with.".to_string(), "With paper ballots, poll hacking is not possible, whereas with EVMs, it is possible due to their digital nature.".to_string(), "Paper ballots can be destroyed, whereas EVMs are indestructible and resistant to fire and water".to_string()],
        context: r#"Paper ballots allow for tangible evidence of manipulation, such as improper handling or ballot tampering, which can be documented through smartphone cameras, CCTV surveillance, or by physical inspection. This enables voters or observers to raise concerns and potentially call for a re-election. In contrast, EVMs operate digitally, and any manipulation within them typically involves hardware or software alterations inside the machine itself. This internal manipulation is not observable through external monitoring, making it much harder to detect and address potential fraud. Therefore, while both systems have vulnerabilities, paper ballots offer a transparent mechanism for identifying and addressing irregularities."#.to_string(),
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

                <Mcq data=ballot_stuffing/>

                <Mcq data=ballot_stuffing_vvpat/>

                <Mcq data=vvpat_tallied/>

                <Mcq data=secrecy_software/>

                <Mcq data=open_source_evm/>

                <Mcq data=security_required_evm/>

                <Mcq data=discrepancies_2024_lok_sabha/>

                <Mcq data=how_many_evm_manipulation_required/>

                <Mcq data=paper_ballot_more_secure/>
            </div>
        </div>
    }
}
