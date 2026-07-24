use dioxus::prelude::*;

use crate::utils::format_section_title;

/// The SectionList component that will be rendered when the current route is `[Route::Chapter1]`
#[component]
pub fn SectionsRenderer (section: String) -> Element {
    let display_title = format_section_title(&section);

    rsx! {
        h1 { "Section: {display_title}" }
    }
}
