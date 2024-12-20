use leptos::prelude::*;
use leptos_meta::*;
use leptos_router::components::*;
use leptos_router::path;
use wasm_bindgen::JsCast;
use web_sys::{window, HtmlElement};
// Modules
mod components;
mod pages;

// Top-Level pages
use crate::pages::anxiety_phobia::anxiety_disorders::AnxietyDisorders;
use crate::pages::anxiety_phobia::causes_of_anxiety_disorders::CausesOfAnxietyDisorders;
use crate::pages::anxiety_phobia::relaxation::Relaxation;
use crate::pages::evm_learning::EvmLearning;
use crate::pages::home::Home;
use crate::pages::not_found::NotFound;

/// An app router which renders the homepage and handles 404's
#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    // Get the window object
    let window = window().expect("should have a Window");

    // Check if the user prefers a dark color scheme
    let prefers_dark_mode = match window.match_media("(prefers-color-scheme: dark)") {
        Ok(Some(media_query_list)) => media_query_list.matches(),
        _ => false,
    };

    // Set the class on the <html> tag
    if let Some(document) = window.document() {
        if let Some(html) = document.document_element() {
            let html: HtmlElement = html.unchecked_into();
            if prefers_dark_mode {
                html.set_class_name("dark w-full h-full bg-gray-900");
            } else {
                html.set_class_name("");
            }
        }
    }

    view! {
        <Html {..} lang="en" dir="ltr" attr:data-theme="light"/>

        // sets the document title
        <Title text="Interactive Learning"/>

        // injects metadata in the <head> of the page
        <Meta charset="UTF-8"/>
        <Meta name="viewport" content="width=device-width, initial-scale=1.0"/>

        <Router>
            <Routes fallback=|| "Not Found.">
                <Route path=path!("/") view=Home/>
                <Route path=path!("/*") view=NotFound/>
                <Route path=path!("evm") view=EvmLearning/>
                <Route path=path!("anxiety_disorders") view=AnxietyDisorders/>
                <Route path=path!("causes_anxiety") view=CausesOfAnxietyDisorders/>
                <Route path=path!("relaxation") view=Relaxation />
            </Routes>
        </Router>
    }
}
