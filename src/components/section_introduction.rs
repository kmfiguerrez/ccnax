use dioxus::prelude::*;
use crate::views::{volume1, volume2::chapter9::{section1, section3}};

/// Display section introduction content based on the provided identifiers.
#[component]
pub fn SectionIntroduction(volume_id: u32, part_id: u32, chapter_id: u32, section_id: u32) -> Element {
    rsx! {
        div {
            // This is for demostration purposes only.
            // For real application, use Database!
            match (volume_id, part_id, chapter_id, section_id) {
                (1, 1, 1, 1) => rsx! {
                    h3 { "sucker" }
                },
                // Start of volume 1, part 2, chapter 6, section 1
                (1, 2, 7, 1) => rsx! {
                    volume1::chapter6::section1::subheader1_content::Content {}
                },
                _ => rsx! {}, // h3 { "get lost" }
            }
        }
    }
}