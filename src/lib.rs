use leptos::prelude::*;
use leptos_router::{components::*, path};

// Modules
mod components;
mod pages;

// Top-Level pages
use pages::home::Home;
use pages::not_found::NotFound;

/// An app router which renders the homepage and handles 404's
#[component]
pub fn App() -> impl IntoView {
    view! {
        <Router>
            <Routes fallback=|| view! { <NotFound /> }>
                <Route path=path!("/") view=Home />
            </Routes>
        </Router>
    }
}
