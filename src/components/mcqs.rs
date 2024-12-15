use crate::components::mcq_struct::McqData;
use leptos::either::Either;
use leptos::prelude::*;
use rand::prelude::SliceRandom;
#[component]
pub fn Mcq(data: McqData) -> impl IntoView {
    let McqData {
        question,
        correct_answer,
        mut options,
        context,
    } = data;

    // Add the correct answer to options and shuffle
    options.push(correct_answer.clone());
    let mut rng = rand::thread_rng();
    options.shuffle(&mut rng);

    let (selected_answer, set_selected_answer) = signal(None::<String>);
    let (show_result, set_show_result) = signal(false);

    view! {
        <>
            <h4>"MCQs"</h4>
            <div class="mcq-component">
                <h2>{question}</h2>
                <ul class="options">
                    {options
                        .iter()
                        .map(|option| {
                            let option = option.clone();
                            view! {
                                <li>
                                    <button on:click=move |_| {
                                        set_selected_answer(Some(option.clone()));
                                        set_show_result(true);
                                    }>

                                        {option.clone()}
                                    </button>
                                </li>
                            }
                        })
                        .collect::<Vec<_>>()}
                </ul>

                {move || {
                    if show_result.get() {
                        let selected = selected_answer.get().clone().unwrap_or_default();
                        let is_correct = selected == correct_answer;
                        Either::Left(
                            view! {
                                <div class="result">
                                    <h3>{if is_correct { "Correct!" } else { "Wrong Answer!" }}</h3>
                                    <p>

                                        {"You selected: "} <strong>{selected.clone()}</strong>
                                    </p>

                                    {if !is_correct {
                                        Either::Left(
                                            view! {
                                                <p>
                                                    "Correct answer: " <strong>{correct_answer.clone()}</strong>
                                                </p>
                                            },
                                        )
                                    } else {
                                        Either::Right(view! { <></> })
                                    }}

                                    <p>{context.clone()}</p>

                                </div>
                            },
                        )
                    } else {
                        Either::Right(view! { <></> })
                    }
                }}

            </div>
        </>
    }
}
