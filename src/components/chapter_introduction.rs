use dioxus::prelude::*;
use crate::views::{volume1, volume2};

/// Display section introduction content based on the provided identifiers.
#[component]
pub fn ChapterIntroduction(volume_id: u32, part_id: u32, chapter_id: u32) -> Element {
    rsx! {
        div {
            // This is for demostration purposes only.
            // For real application, use Database!
            match (volume_id, part_id, chapter_id) {
                // Start of volume 1, part 2
                (1, 2, 7) => rsx! {
                    volume1::chapter7::ChapterIntroductionContent {}
                },
                // Start of volume 1, part 3
                (1, 3, 8) => rsx! {
                    volume1::chapter8::ChapterIntroductionContent {}
                },
                // Start of volume 1, part 3
                (1, 3, 9) => rsx! {
                    volume1::chapter9::ChapterIntroductionContent {}
                },
                (1, 3, 10) => rsx! {
                    volume1::chapter10::ChapterIntroductionContent {}
                },
                (1, 5, 17) => rsx! {
                    volume1::chapter17::ChapterIntroductionContent {}
                },
                // Start of volume 2, part 1
                (2, 1, 2) => rsx! {
                    volume2::chapter2::ChapterIntroductionContent {}
                },
                // Start of volume 2, part 3
                (2, 3, 10) => rsx! {
                    volume2::chapter10::ChapterIntroductionContent {}
                },
                _ => rsx! {
                    h3 { "Chapter Introduction not found!" }
                },
            }
        }
    }
}