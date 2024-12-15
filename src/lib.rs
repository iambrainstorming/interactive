use leptos::prelude::*;
use leptos_meta::*;
use leptos_router::components::*;
use leptos_router::path;
// Modules
mod components;
mod pages;

// Top-Level pages
use crate::pages::evm_learning::EvmLearning;
use crate::pages::home::Home;
use crate::pages::not_found::NotFound;

/// An app router which renders the homepage and handles 404's
#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {
        <Html {..} lang="en" dir="ltr" attr:data-theme="light"/>

        // sets the document title
        <Title text="Welcome to Leptos CSR"/>

        // injects metadata in the <head> of the page
        <Meta charset="UTF-8"/>
        <Meta name="viewport" content="width=device-width, initial-scale=1.0"/>

        <Router>
            <Routes fallback=|| "Not Found.">
                <Route path=path!("/") view=Home/>
                <Route path=path!("/*") view=NotFound/>
                <Route path=path!("evm") view=EvmLearning/>
            </Routes>
        </Router>
    }
}
