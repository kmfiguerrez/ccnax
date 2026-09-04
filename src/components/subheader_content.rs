use dioxus::prelude::*;
use crate::views::{volume1, volume2::chapter9::{section1, section3}};

/// Display subheader content based on the provided identifiers.
#[component]
pub fn SubheaderContent(volume_id: u32, part_id: u32, chapter_id: u32, section_id: u32, subheader_id: u32) -> Element {
    rsx! {
        div { class: "pb-4",
            // This is for demostration purposes only.
            // For real application, use Database!
            match (volume_id, part_id, chapter_id, section_id, subheader_id) {
                (1, 1, 1, 1, 1) => rsx! {
                    h3 { "sucker" }
                },
                // Start of volume 1, part 2, chapter 6, section 1
                (1, 2, 6, 1, 1) => rsx! {
                    volume1::chapters::chapter6::section1::subheader1_content::Content {}
                },
                (1, 2, 6, 1, 2) => rsx! {
                    volume1::chapters::chapter6::section1::subheader2_content::Content {}
                },
                (1, 2, 6, 1, 3) => rsx! {
                    volume1::chapters::chapter6::section1::subheader3_content::Content {}
                },
                (1, 2, 6, 1, 4) => rsx! {
                    volume1::chapters::chapter6::section1::subheader4_content::Content {}
                },
                (2, 3, 9, 1, 1) => rsx! {
                    section1::subheader1_content::Content {}
                },
                (2, 3, 9, 1, 2) => rsx! {
                    section1::subheader2_content::Content {}
                },
                (2, 3, 9, 1, 3) => rsx! {
                    section1::subheader3_content::Content {}
                },
                (2, 3, 9, 1, 4) => rsx! {
                    section1::subheader4_content::Content {}
                },
                (2, 3, 9, 1, 5) => rsx! {
                    section1::subheader5_content::Content {}
                },
                (2, 3, 9, 1, 6) => rsx! {
                    section1::subheader6_content::Content {}
                },
                // Start of volume 2, part 3, chapter 9, section 3
                (2, 3, 9, 3, 1) => rsx! {
                    section3::subheader1_content::Content {}
                },
                (2, 3, 9, 3, 2) => rsx! {
                    section3::subheader2_content::Content {}
                },
                (2, 3, 9, 3, 3) => rsx! {
                    section3::subheader3_content::Content {}
                },
                (2, 3, 9, 3, 4) => rsx! {
                    section3::subheader4_content::Content {}
                },
                _ => rsx! {
                    h3 { "get lost" }
                },
            }
        }
    }
}