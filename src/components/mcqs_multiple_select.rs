use crate::components::markdown::parse_text_to_html;
use crate::components::mcq_struct::McqDataMultipleSelect;
use leptos::either::Either;
use leptos::prelude::*;
use rand::prelude::SliceRandom;

#[component]
pub fn McqMultipleSelect(data: McqDataMultipleSelect) -> impl IntoView {
    let McqDataMultipleSelect {
        question,
        correct_answers,
        mut options,
        context,
    } = data;

    // Add correct answers to options and shuffle
    options.extend(correct_answers.clone());
    options.dedup();
    let mut rng = rand::thread_rng();
    options.shuffle(&mut rng);

    let (selected_answers, set_selected_answers) = signal(Vec::new());
    let (show_result, set_show_result) = signal(false);

    view! {
        <>
            <div>
                <section class="bg-white dark:bg-slate-800 py-12 rounded">
                    <div class="px-4">
                        <div class="flex flex-col items-left">
                            <p
                                inner_html=parse_text_to_html(&question)
                                class="text-gray-800 mb-4 text-lg dark:text-white"
                            ></p>

                            <p>
                                <ul class="text-gray-600 mb-8 text-lg dark:text-white">
                                    {options
                                        .iter()
                                        .map(|option| {
                                            let option_clone = option.clone();
                                            let option_clone2 = option.clone();
                                            view! {
                                                // Clone here for use inside the closure
                                                <li class="space-y-4 py-3">
                                                    <button
                                                        class=move || {
                                                            if selected_answers.get().contains(&option_clone) {
                                                                "bg-green-300 text-black hover:bg-green-400 block w-full px-4 py-2 rounded font-medium"
                                                                    .to_string()
                                                            } else {
                                                                "bg-yellow-400 text-black hover:bg-yellow-500 block w-full px-4 py-2 rounded font-medium"
                                                                    .to_string()
                                                            }
                                                        }

                                                        on:click=move |_| {
                                                            let mut answers = selected_answers.get();
                                                            if answers.contains(&option_clone2) {
                                                                answers.retain(|x| x != &option_clone2);
                                                            } else {
                                                                answers.push(option_clone2.clone());
                                                            }
                                                            set_selected_answers(answers);
                                                        }
                                                    >

                                                        {option.clone()}
                                                    // <span class="text-xs text-gray-500">
                                                    // {move || format!("Selected: {:?}", selected_answers.get())}
                                                    // </span>
                                                    </button>
                                                </li>
                                            }
                                        })
                                        .collect::<Vec<_>>()}
                                </ul>

                            </p>

                            <div class="flex space-x-4">
                                <button
                                    class="bg-blue-500 text-white font-semibold py-2 px-4 rounded hover:bg-blue-600"
                                    on:click=move |_| set_show_result(true)
                                >
                                    "Check Answers"
                                </button>
                                <button
                                    class="bg-red-500 text-white font-semibold py-2 px-4 rounded hover:bg-red-600"
                                    on:click=move |_| {
                                        set_selected_answers(Vec::new());
                                        set_show_result(false);
                                    }
                                >

                                    <strong>"Clear Answers"</strong>
                                </button>
                            </div>

                            <section class="text-gray-600 mt-6 mb-8 text-lg dark:text-white">
                                {move || {
                                    if show_result.get() {
                                        let selected = selected_answers.get();
                                        let is_correct = selected.len() == correct_answers.len()
                                            && selected
                                                .iter()
                                                .all(|answer| correct_answers.contains(answer));
                                        Either::Left(
                                            view! {
                                                <div>
                                                    {if is_correct {
                                                        view! {
                                                            <h3 class="bg-green-500 text-white font-semibold py-2 px-4 rounded hover:bg-green-600">
                                                                "Correct!"
                                                            </h3>
                                                        }
                                                            .into_any()
                                                    } else {
                                                        view! {
                                                            <h3 class="bg-red-500 text-white font-semibold py-2 px-4 rounded hover:bg-red-600">
                                                                "Wrong Answer!"
                                                            </h3>
                                                        }
                                                            .into_any()
                                                    }}
                                                    <p>
                                                        {"You selected: "}
                                                        <strong inner_html=selected.join("<br/>")></strong>
                                                    </p>
                                                    {if !is_correct {
                                                        Either::Left(
                                                            view! {
                                                                <p>
                                                                    "Correct answers: " <br/>
                                                                    <strong inner_html=correct_answers.join("<br/>")></strong>
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
