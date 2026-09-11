use dioxus::prelude::*;
use crate::views::{volume1, volume2};

/// Display section introduction content based on the provided identifiers.
#[component]
pub fn SectionIntroduction(volume_id: u32, part_id: u32, chapter_id: u32, section_id: u32) -> Element {
    rsx! {
        div {
            // This is for demostration purposes only.
            // For real application, use Database!
            match (volume_id, part_id, chapter_id, section_id) {
                // Start of volume 1, part 2, chapter 7
                (1, 2, 7, 1) => rsx! {
                    volume1::chapter7::section1::SectionIntroductionContent {}
                },
                (1, 2, 7, 2) => rsx! {
                    volume1::chapter7::section2::SectionIntroductionContent {}
                },
                // Start of volume 1, part 3, chapter 8
                (1, 3, 8, 1) => rsx! {
                    volume1::chapter8::section1::SectionIntroductionContent {}
                },
                _ => rsx! {
                    h3 { "Section Introduction not found!" }
                },
            }
        }
    }
}