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

    let anonymity_votes = McqData {
        question: "Why is the anonymity of voters essential in an election?".to_string(),
        correct_answer: "To prevent coercion and undue influence on voters. Maintaining voter anonymity ensures that no one can trace an individual's vote. This eliminates the possibility of anyone forcing, threatening, or bribing voters to cast their vote in a particular way. It upholds the principle of free choice.".to_string(),
        options: vec!["To encourage higher voter turnout. Knowing their votes are private can encourage more people to participate in elections.".to_string(), " To ensure unbiased election results. Voter anonymity prevents external influences from affecting the authenticity of the results, as individuals vote based on their own preferences rather than under duress.".to_string(), "To simplify the vote-counting process. Ensuring voter anonymity avoids linking votes to individuals, which could otherwise complicate the logistics of vote counting and raise questions about procedural fairness.".to_string()],
        context: r#"The secret ballot, is a voting method in which a voter's identity in an election or a referendum is anonymous. This forestalls attempts to influence the voter by intimidation, blackmailing, and potential vote buying. This system is one means of achieving the goal of political privacy."#.to_string(),
    };

    let anonymity_votes_evm = McqData {
            question: "Is the anonymity of voters using Electronic Voting Machines (EVMs) questionable?".to_string(),
            correct_answer: "EVMs might store the sequence of votes cast, which could potentially be linked to individual voters.".to_string(),
            options: vec![
                "Votes are anonymous with EVMs, so no concerns should arise".to_string(),
                "EVMs do not store any data, only the final result".to_string(),
                "Manual voting methods face the same issues as EVMs".to_string(),
            ],
            context: r#"Electronic Voting Machines (EVMs) are proprietary systems, and their inner workings are not fully transparent to the public. This lack of transparency raises concerns about how data is stored in EVMs (both Control Unit and VVPAT). Specifically, there is uncertainty over whether the EVM records only the total number of votes for each party or if it retains the sequence of votes cast. If EVMs store the order in which votes are cast, this sequence could be cross-referenced with the order of voters recorded at polling stations, compromising the secrecy of the vote. This is a significant concern, as it undermines the principle of anonymous voting."#
                .to_string(),
        };
    let one_time_programmable_memory = McqData {
            question: "What is One-Time Programmable (OTP) memory?".to_string(),
            correct_answer: "A special type of non-volatile memory (NVM) that permits data to be written only once".to_string(),
            options: vec![
                "A type of memory that allows data to be written and erased multiple times".to_string(),
                "A temporary memory used for short-term data storage".to_string(),
                "A memory type that automatically deletes data after power loss".to_string(),
            ],
            context: r#"OTP (one time programmable) memory is a special type of non-volatile memory (NVM) that permits data to be written to memory only once. Once the memory has been programmed, it retains its value upon loss of power (i.e., is non-volatile).  Once programmed, the data cannot be altered or erased. OTP memory is used in applications where reliable and repeatable reading of data is required."#
                .to_string(),
        };

    let one_time_progammable_evm = McqData {
            question: "Is the memory used in Electronic Voting Machines (EVMs) one-time programmable (OTP)?".to_string(),
            correct_answer: "EVMs use volatile memory to store votes temporarily during operation. This design choice ensures reusability but raises concerns about vulnerability to tampering or memory manipulation.".to_string(),
            options: vec![
                "EVMs use OTP memory to ensure complete security of votes".to_string(),
                "EVMs use OTP memory but reset it before each election".to_string(),
                "No, EVMs rely on encrypted permanent memory for storing votes".to_string(),
            ],
            context: r#"EVMs are reused multiple times in elections, and the votes are stored in volatile memory rather than OTP memory. This makes the memory potentially vulnerable to manipulation or tampering. Additionally, there is limited transparency about the origins of the microcontrollers used in EVMs. Information regarding their manufacture is not disclosed, even under the Right to Information (RTI) Act. The companies Bharat Electronics Limited (BEL) and Electronics Corporation of India Limited (ECIL) only assemble these devices.<br/> <br/> BEL had shared this information under RTI in May 2019. The microchips from our EVMs are supplied by NXP. But the more crucial RTI revelation was this, while the [Election Commission has always claimed that the EVM microchip is one-time programmable only](https://www.youtube.com/watch?v=LDrhK7TIdDo), experts went on to the NXP website and found that its microchips have FLASH memory which are not just one time programmable. Meaning, if accessed, then can be re-programmed opening the EVM to manipulation."#
                .to_string(),
        };
    let paper_obsolete = McqData {
            question: "Why is using paper ballots not an obsolete technology?".to_string(),
            correct_answer: "All of the above".to_string(),
            options: vec![
                "It decreases the cost of the voting process because paper is much cheaper than EVMs.".to_string(),
                "With paper ballots, it is easier to maintain security due to the simplicity of the process, which is easily understood by everyone, whereas EVMs are complicated technologies with opaque and intricate processes.".to_string(),
                "Automation of paper ballot counting can be achieved using current advancements in machine learning algorithms or AI. This can be implemented transparently through an open-source program that anyone can audit.".to_string(),
            ],
            context: r#""#
                .to_string(),
        };
    let paper_ballot_booth_capturing = McqData {
            question: "How can we make ballot boxes resistant to booth capturing and ballot stuffing?".to_string(),
            correct_answer: "All of the Above".to_string(),
            options: vec![
                "Use of robotic secure robotic ballot boxes that open and close at specific intervals with the press of a button by polling officers. ".to_string(),
                "A robotic ballot box can include a serial number cutter, which removes the serial number from the ballot paper. This is provided to the polling officer to prevent multiple ballot insertions or ballot stuffing while maintaining voter anonymity.".to_string(),
                "The time delay for starting the counting process needs to be reduced and can be initiated the day after polling. ".to_string(),
                "Ballot box surveillance by people and stakeholder through CCTV cameras and mobile phone cameras.".to_string(),
            ],
            context: r#"Booth capturing, also known as booth looting, is electoral fraud in which party loyalists or hired criminals "capture" a polling booth and vote on behalf of legitimate voters to ensure a particular candidate's victory. One argument is that Electronic Voting Machines (EVMs) help decrease booth capturing by enabling a five-minute delay in voting process, polling officer can also press a close button to deactivate the EVM. However, this measure doesn't entirely prevent booth capturing. Examples of booth capturing date back to the 1990s, but with the advancement of media, surveillance technology and redundant camera based mobile phones, it has become increasingly difficult to conceal such activities. Since booth capturing is a criminal offense, re-elections can be conducted in places where it occurs.<br/><br/>Regarding the time delay, we can now implement secure robotic ballot boxes that open and close at specific intervals with the press of a button by polling officers. The slit or hole can be thin enough so that a single ballot can enter. A robotic ballot box can include a serial number cutter, which removes the serial number from the ballot paper. This is provided to the polling officer to prevent multiple ballot insertions or ballot stuffing while maintaining voter anonymity. It is a simple machine. Such a device will be much cheaper than an EVM assembly. While booth capturing can be detected, it remains challenging to ascertain what the EVM software is doing in the background.<br/><br/>The time delay for starting the counting process needs to be reduced and can be initiated the day after polling. This minimizes the need for prolonged ballot box surveillance, reduces surveillance resources, and makes ballot box replacement by criminals significantly more difficult."#
                .to_string(),
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

                <Mcq data=anonymity_votes/>

                <Mcq data=anonymity_votes_evm/>

                <Mcq data=one_time_programmable_memory/>

                <Mcq data=one_time_progammable_evm/>

                <Mcq data=paper_obsolete/>

                <Mcq data=paper_ballot_booth_capturing/>
            </div>
        </div>
    }
}
