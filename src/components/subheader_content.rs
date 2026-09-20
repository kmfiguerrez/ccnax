use dioxus::prelude::*;
use crate::views::{volume1, volume2};

/// Display subheader content based on the provided identifiers.
#[component]
pub fn SubheaderContent(volume_id: u32, part_id: u32, chapter_id: u32, section_id: u32, subheader_id: u32) -> Element {
    rsx! {
        div { class: "pb-4",
            // This is for demostration purposes only.
            // For real application, use Database!

            // match (volume_id, part_id, chapter_id, section_id, subheader_id) {
            //     // Start of volume 1, part 3, chapter 9, section 1
            //     (1, 3, 9, 1, 1) => rsx! {
            //         volume1::chapter9::section1::subheader1_content::Content {}
            //     _ => rsx! {
            //         h3 { "get lost" }
            //     },
            // }

            // The reason I used nested matches is so I can collapse the code by volumes, parts and chapters.
            // Match volumes
            match volume_id {
                // Volume 1
                1 => rsx! {
                    // match parts
                    match part_id {
                        // part 2
                        2 => rsx! {
                            // match chapter
                            match chapter_id {
                                // chapter 6
                                6 => rsx! {
                                    match (section_id, subheader_id) {
                                        // section 1
                                        (1, 1) => rsx! {
                                            volume1::chapter6::section1::subheader1_content::Content {}
                                        },
                                        (1, 2) => rsx! {
                                            volume1::chapter6::section1::subheader2_content::Content {}
                                        },
                                        (1, 3) => rsx! {
                                            volume1::chapter6::section1::subheader3_content::Content {}
                                        },
                                        (1, 4) => rsx! {
                                            volume1::chapter6::section1::subheader4_content::Content {}
                                        },
                                        // section 2
                                        (2, 1) => rsx! {
                                            volume1::chapter6::section2::subheader1_content::Content {}
                                        },
                                        (2, 2) => rsx! {
                                            volume1::chapter6::section2::subheader2_content::Content {}
                                        },
                                        (2, 3) => rsx! {
                                            volume1::chapter6::section2::subheader3_content::Content {}
                                        },
                                        (2, 4) => rsx! {
                                            volume1::chapter6::section2::subheader4_content::Content {}
                                        },
                                        // section 3
                                        (3, 1) => rsx! {
                                            volume1::chapter6::section3::subheader1_content::Content {}
                                        },
                                        (3, 2) => rsx! {
                                            volume1::chapter6::section3::subheader2_content::Content {}
                                        },
                                        _ => rsx! {
                                            h3 { "Subheader content not found!" }
                                        },
                                    }
                                },
                                // chapter 7
                                7 => rsx! {
                                    match (section_id, subheader_id) {
                                        // section 1
                                        (1, 1) => rsx! {
                                            volume1::chapter7::section1::subheader1_content::Content {}
                                        },
                                        (1, 2) => rsx! {
                                            volume1::chapter7::section1::subheader2_content::Content {}
                                        },
                                        (1, 3) => rsx! {
                                            volume1::chapter7::section1::subheader3_content::Content {}
                                        },
                                        (1, 4) => rsx! {
                                            volume1::chapter7::section1::subheader4_content::Content {}
                                        },
                                        (1, 5) => rsx! {
                                            volume1::chapter7::section1::subheader5_content::Content {}
                                        },
                                        // section 2
                                        (2, 1) => rsx! {
                                            volume1::chapter7::section2::subheader1_content::Content {}
                                        },
                                        (2, 2) => rsx! {
                                            volume1::chapter7::section2::subheader2_content::Content {}
                                        },
                                        (2, 3) => rsx! {
                                            volume1::chapter7::section2::subheader3_content::Content {}
                                        },
                                        _ => rsx! {
                                            h3 { "Subheader content not found!" }
                                        },
                                    }
                                },

                                _ => rsx! {
                                    h3 { "Chapter {chapter_id} does not exist!" }
                                },
                            }
                        },
                        // part 3
                        3 => rsx! {
                            match chapter_id {
                                // chapter 8
                                8 => rsx! {
                                    match (section_id, subheader_id) {
                                        // section 1
                                        (1, 1) => rsx! {
                                            volume1::chapter8::section1::subheader1_content::Content {}
                                        },
                                        (1, 2) => rsx! {
                                            volume1::chapter8::section1::subheader2_content::Content {}
                                        },
                                        // section 2
                                        (2, 1) => rsx! {
                                            volume1::chapter8::section2::subheader1_content::Content {}
                                        },
                                        (2, 2) => rsx! {
                                            volume1::chapter8::section2::subheader2_content::Content {}
                                        },
                                        (2, 3) => rsx! {
                                            volume1::chapter8::section2::subheader3_content::Content {}
                                        },
                                        (2, 4) => rsx! {
                                            volume1::chapter8::section2::subheader4_content::Content {}
                                        },
                                        // section 3
                                        (3, 1) => rsx! {
                                            volume1::chapter8::section3::subheader1_content::Content {}
                                        },
                                        (3, 2) => rsx! {
                                            volume1::chapter8::section3::subheader2_content::Content {}
                                        },
                                        (3, 3) => rsx! {
                                            volume1::chapter8::section3::subheader3_content::Content {}
                                        },
                                        (3, 4) => rsx! {
                                            volume1::chapter8::section3::subheader4_content::Content {}
                                        },
                                        _ => rsx! {
                                            h3 { "Subheader content not found!" }
                                        },
                                    }
                                },
                                // chapter 9
                                9 => rsx! {
                                    match (section_id, subheader_id) {
                                        // start of chapter 9 section 1
                                        (1, 1) => rsx! {
                                            volume1::chapter9::section1::subheader1_content::Content {}
                                        },
                                        (1, 2) => rsx! {
                                            volume1::chapter9::section1::subheader2_content::Content {}
                                        },
                                        (1, 3) => rsx! {
                                            volume1::chapter9::section1::subheader3_content::Content {}
                                        },
                                        (1, 4) => rsx! {
                                            volume1::chapter9::section1::subheader4_content::Content {}
                                        },
                                        // start of chapter 9 section 2
                                        (2, 1) => rsx! {
                                            volume1::chapter9::section2::subheader1_content::Content {}
                                        },
                                        (2, 2) => rsx! {
                                            volume1::chapter9::section2::subheader2_content::Content {}
                                        },
                                        (2, 3) => rsx! {
                                            volume1::chapter9::section2::subheader3_content::Content {}
                                        },
                                        // start of chapter 9 section 3
                                        (3, 1) => rsx! {
                                            volume1::chapter9::section3::subheader1_content::Content {}
                                        },
                                        (3, 2) => rsx! {
                                            volume1::chapter9::section3::subheader2_content::Content {}
                                        },
                                        (3, 3) => rsx! {
                                            volume1::chapter9::section3::subheader3_content::Content {}
                                        },
                                        (3, 4) => rsx! {
                                            volume1::chapter9::section3::subheader4_content::Content {}
                                        },
                                        (3, 5) => rsx! {
                                            volume1::chapter9::section3::subheader5_content::Content {}
                                        },
                                        (3, 6) => rsx! {
                                            volume1::chapter9::section3::subheader6_content::Content {}
                                        },
                                        _ => rsx! {
                                            h3 { "Subheader content not found!" }
                                        },
                                    }
                                },
                                // chapter 10
                                10 => rsx! {
                                    match (section_id, subheader_id) {
                                        // start of chapter 9 section 1
                                        (1, 1) => rsx! {
                                            volume1::chapter10::section1::subheader1_content::Content {}
                                        },
                                        (1, 2) => rsx! {
                                            volume1::chapter10::section1::subheader2_content::Content {}
                                        },
                                        _ => rsx! {
                                            h3 { "Subheader content not found!" }
                                        },
                                    }
                                },
                                _ => rsx! {
                                    h3 { "Chapter {chapter_id} does not exist!" }
                                },

                            }
                        },
                        _ => rsx! {
                            h3 { "Part {part_id} does not exist!" }
                        },
                    }
                },
                // volume 2
                2 => rsx! {
                    // match parts
                    match part_id {
                        // part 3
                        3 => rsx! {
                            match chapter_id {
                                // chapter 9
                                9 => rsx! {
                                    match (section_id, subheader_id) {
                                        // section 1
                                        (1, 1) => rsx! {
                                            volume2::chapter9::section1::subheader1_content::Content {}
                                        },
                                        (1, 2) => rsx! {
                                            volume2::chapter9::section1::subheader2_content::Content {}
                                        },
                                        (1, 3) => rsx! {
                                            volume2::chapter9::section1::subheader3_content::Content {}
                                        },
                                        (1, 4) => rsx! {
                                            volume2::chapter9::section1::subheader4_content::Content {}
                                        },
                                        (1, 5) => rsx! {
                                            volume2::chapter9::section1::subheader5_content::Content {}
                                        },
                                        (1, 6) => rsx! {
                                            volume2::chapter9::section1::subheader6_content::Content {}
                                        },
                                        // section 3
                                        (3, 1) => rsx! {
                                            volume2::chapter9::section3::subheader1_content::Content {}
                                        },
                                        (3, 2) => rsx! {
                                            volume2::chapter9::section3::subheader2_content::Content {}
                                        },
                                        (3, 3) => rsx! {
                                            volume2::chapter9::section3::subheader3_content::Content {}
                                        },
                                        (3, 4) => rsx! {
                                            volume2::chapter9::section3::subheader4_content::Content {}
                                        },
                                        _ => rsx! {
                                            h3 { "Subheader content not found!" }
                                        },
                                    }
                                },
                                _ => rsx! {
                                    h3 { "Chapter {chapter_id} does not exist!" }
                                },
                            }
                        },
                        _ => rsx! {
                            h3 { "Part {part_id} does not exist!" }
                        },
                    }
                },
                _ => rsx! {
                    h3 { "Volume {volume_id} does not exist!" }
                },
            }
        
        }
    }
}