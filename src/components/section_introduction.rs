use dioxus::prelude::*;
use crate::views::{volume1, volume2};

/// Display section introduction content based on the provided identifiers.
#[component]
pub fn SectionIntroduction(volume_id: u32, part_id: u32, chapter_id: u32, section_id: u32) -> Element {
    rsx! {
        div {
            // This is for demostration purposes only.
            // For real application, use Database!
            // match (volume_id, part_id, chapter_id, section_id) {
            //     // Start of volume 1, part 2, chapter 7
            //     (1, 2, 7, 1) => rsx! {
            //         volume1::chapter7::section1::SectionIntroductionContent {}
            //     },
            //     // Start of volume 1, part 3, chapter 8
            //     (1, 3, 8, 1) => rsx! {
            //         volume1::chapter8::section1::SectionIntroductionContent {}
            //     },
            //     // Start of volume 1, part 3, chapter 9
            //     (1, 3, 9, 1) => rsx! {
            //         volume1::chapter9::section1::SectionIntroductionContent {}
            //     },
            //     _ => rsx! {
            //         h3 { "Section Introduction not found!" }
            //     },
            // }

            // I used if/else so I can collapse code by volume.
            // I also used nested match to collapse the code by part and chapter.
            if volume_id == 1 {
                match part_id {
                    2 => rsx! {
                        match chapter_id {
                            7 => rsx! {
                                match section_id {
                                    1 => rsx! {
                                        volume1::chapter7::section1::SectionIntroductionContent {}
                                    },
                                    2 => rsx! {
                                        volume1::chapter7::section2::SectionIntroductionContent {}
                                    },
                                    _ => rsx! {
                                        h3 { "Section introduction content not found!" }
                                    },
                                }
                            },
                            _ => rsx! {
                                h3 { "Chapter {chapter_id} does not exist!" }
                            },
                        }
                    },
                    3 => rsx! {
                        match chapter_id {
                            8 => rsx! {
                                match section_id {
                                    1 => rsx! {
                                        volume1::chapter8::section1::SectionIntroductionContent {}
                                    },
                                    2 => rsx! {
                                        volume1::chapter8::section2::SectionIntroductionContent {}
                                    },
                                    3 => rsx! {
                                        volume1::chapter8::section3::SectionIntroductionContent {}
                                    },
                                    _ => rsx! {
                                        h3 { "Section introduction content not found!" }
                                    },
                                }
                            },
                            9 => rsx! {
                                match section_id {
                                    1 => rsx! {
                                        volume1::chapter9::section1::SectionIntroductionContent {}
                                    },
                                    2 => rsx! {
                                        volume1::chapter9::section2::SectionIntroductionContent {}
                                    },
                                    3 => rsx! {
                                        volume1::chapter9::section3::SectionIntroductionContent {}
                                    },
                                    4 => rsx! {
                                        volume1::chapter9::section4::SectionIntroductionContent {}
                                    },
                                    _ => rsx! {
                                        h3 { "Section introduction content not found!" }
                                    },
                                }
                            },
                            10 => rsx! {
                                match section_id {
                                    1 => rsx! {
                                        volume1::chapter10::section1::SectionIntroductionContent {}
                                    },
                                    2 => rsx! {
                                        volume1::chapter10::section2::SectionIntroductionContent {}
                                    },
                                    _ => rsx! {
                                        h3 { "Section introduction content not found!" }
                                    },
                                }
                            },
                            _ => rsx! {
                                h3 { "Chapter {chapter_id} does not exist!" }
                            },
                        }
                    },
                    5 => rsx! {
                        match chapter_id {
                            17 => rsx! {
                                match section_id {
                                    1 => rsx! {
                                        volume1::chapter17::section1::SectionIntroductionContent {}
                                    },
                                    2 => rsx! {
                                        volume1::chapter17::section2::SectionIntroductionContent {}
                                    },
                                    _ => rsx! {
                                        h3 { "Section introduction content not found!" }
                                    },
                                }
                            },
                            _ => rsx! {
                                h3 { "Chapter {chapter_id} does not exist!" }
                            },
                        }
                    },
                    _ => rsx! {
                        h3 { "Part {part_id} does not exist" }
                    },
                }
            }
        
        }
    }
}