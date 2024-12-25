// use crate::components::counter_btn::Button;
use crate::components::navigation::nav::Nav;
use leptos::prelude::*;

/// Default Home Page
#[component]
pub fn Home() -> impl IntoView {
    view! {
        <ErrorBoundary fallback=|errors| {
            view! {
                <h1>"Uh oh! Something went wrong!"</h1>

                <p>"Errors: "</p>
                // Render a list of errors as strings - good for development purposes
                <ul>
                    {move || {
                        errors
                            .get()
                            .into_iter()
                            .map(|(_, e)| view! { <li>{e.to_string()}</li> })
                            .collect_view()
                    }}

                </ul>
            }
        }>
            <Nav/>
            <div class="container mx-auto bg-white border-gray-200 dark:bg-gray-900 dark:border-gray-700">
                <div class="p-6 bg-white dark:bg-gray-900 text-gray-800 dark:text-gray-200">
                    <h1 class="text-3xl font-bold text-center mb-4">
                        "Welcome to Interactive Learning"
                    </h1>
                    <p class="text-lg text-center">
                        "Explore a range of topics, including mental health awareness, the principles of nonviolent struggle,
                        and the workings of electronic voting machines, among others. Each course is designed to provide insightful
                        knowledge and practical understanding to empower you in these areas. Simply use the navigation menu to browse
                        and access the courses that interest you, and start your journey of learning today!"
                    </p>
                </div>
            </div>
        </ErrorBoundary>
    }
}
