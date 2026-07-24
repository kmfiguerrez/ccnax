use dioxus::prelude::*;
use crate::{Route, utils::slugify};


/// The SectionList component that will be rendered when the current route is `[Route::Chapter1]`
#[component]
pub fn SectionList () -> Element {
    let sections = ["Perspectives on Networking", "TCP/IP Networking Model", "Data Encapsulation Terminology"];

    rsx! {
        // Hero {}
        h1 { "Chapter 1" }
        p { "Sections" }
        ol {
            for (idx , section) in sections.iter().enumerate() {
                {
                    let section_idx = idx + 1;
                    rsx! {
                        // li { key: "{section_idx}", "Section {section_idx}: {section}" }
                        li { key: "{section_idx}",
                            Link {
                                to: Route::SectionsRenderer {
                                    section: slugify(section),
                                },
                                "Section {section_idx}: {section}"
                            }
                        }
                    }
                }
            }
        }

    }
}




