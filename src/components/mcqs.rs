use crate::components::markdown::parse_text_to_html;
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
            <div>
                <section class="bg-white dark:bg-slate-800 py-12 rounded">
                    <div class="px-4">
                        <div class="flex flex-col items-left">
                            <h1 class="text-4xl font-bold text-gray-800 mb-4 text-2xl dark:text-white">
                                {question}
                            </h1>

                            <p>
                                <ul class="text-gray-600 mb-8 text-lg dark:text-white">
                                    {options
                                        .iter()
                                        .map(|option| {
                                            let option = option.clone();
                                            view! {
                                                <li class="space-y-4 py-3">
                                                    <button
                                                        class="bg-yellow-400 text-black rounded px-4 py-2 hover:bg-yellow-500 block w-full"
                                                        on:click=move |_| {
                                                            set_selected_answer(Some(option.clone()));
                                                            set_show_result(true);
                                                        }
                                                    >

                                                        {option.clone()}

                                                    </button>
                                                </li>
                                            }
                                        })
                                        .collect::<Vec<_>>()}
                                </ul>
                            </p>

                            <section class="text-gray-600 mb-8 text-lg dark:text-white">
                                {move || {
                                    if show_result.get() {
                                        let selected = selected_answer
                                            .get()
                                            .clone()
                                            .unwrap_or_default();
                                        let is_correct = selected == correct_answer;
                                        Either::Left(
                                            view! {
                                                <div>
                                                    {if is_correct {
                                                        view! { <h3 class="btn btn-success">"Correct!"</h3> }
                                                            .into_any()
                                                    } else {
                                                        view! { <h3 class="btn btn-error">"Wrong Answer!"</h3> }
                                                            .into_any()
                                                    }}
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
                                                    <br/>
                                                    <div inner_html=parse_text_to_html(&context.clone())></div>

                                                </div>
                                            },
                                        )
                                    } else {
                                        Either::Right(view! { <></> })
                                    }
                                }}

                            </section>
                        </div>
                    </div>
                </section>

            </div>
        </>
    }
}
