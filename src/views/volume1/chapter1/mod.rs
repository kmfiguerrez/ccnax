use dioxus::prelude::*;

mod section;
pub use section::*;

/// The Chapter1 page component that will be rendered when the current route is `[Route::Chapter1]`
#[component]
pub fn Chapter1 () -> Element {
    rsx! {
        section::SectionList {}
    }
}