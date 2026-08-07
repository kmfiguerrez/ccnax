use dioxus::prelude::*;
use crate::components::separator::Separator;

use crate::{
    utils::db_models::Database, 
    views::volume2::chapter9::{section1, section3}
};

#[component]
pub fn Subheader(volume_id: u32, part_id: u32, chapter_id: u32, section_id: u32, subheader_id: u32) -> Element {
    // let current_route: Route = use_route();
    // let subheader_title = format_section_title(&name);

    let db = use_context::<Signal<Database>>();

    // If you chain it all in one line, the guard gets dropped at the semicolon, 
    // and Rust panics because your reference points to dropped memory.

    // 1. Store the read guard in a variable. 
    // This keeps the borrow alive until the component function finishes.
    let db_guard = db.read();

    // 2. Get an Option<&Subheader>.
    let subheader = db_guard.get(&volume_id)
        .and_then(|v| v.parts.get(&part_id))
        .and_then(|p| p.chapters.get(&chapter_id))   
        .and_then(|s| s.sections.get(&section_id))
        .and_then(|sh| sh.subheaders.get(&subheader_id));


    
    rsx! {
        if let Some(subheader) = subheader {
            h1 { class: "text-xl font-bold mb-4", "{subheader.name}" }
            // p { "what" }
            // Display subheader content.
            // This is for demostration purposes only.
            // For real application, use Database!
            match (volume_id, part_id, chapter_id, section_id, subheader_id) {
                (1, 1, 1, 1, 1) => rsx! {
                    h3 { "sucker" }
                },
                (2, 3, 9, 1, 1) => rsx! {
                    Separator { class: "mb-4", horizontal: true, decorative: true }
                    section1::subheader1_content::Content {}
                },
                (2, 3, 9, 1, 2) => rsx! {
                    Separator { class: "mb-4", horizontal: true, decorative: true }
                    section1::subheader2_content::Content {}
                },
                (2, 3, 9, 1, 3) => rsx! {
                    Separator { class: "mb-4", horizontal: true, decorative: true }
                    section1::subheader3_content::Content {}
                },
                (2, 3, 9, 1, 4) => rsx! {
                    Separator { class: "mb-4", horizontal: true, decorative: true }
                    section1::subheader4_content::Content {}
                },
                (2, 3, 9, 1, 5) => rsx! {
                    Separator { class: "mb-4", horizontal: true, decorative: true }
                    section1::subheader5_content::Content {}
                },
                (2, 3, 9, 1, 6) => rsx! {
                    Separator { class: "mb-4", horizontal: true, decorative: true }
                    section1::subheader6_content::Content {}
                },
                // Start of volume 2, part 3, chapter 9, section 3
                (2, 3, 9, 3, 1) => rsx! {
                    Separator { class: "mb-4", horizontal: true, decorative: true }
                    section3::subheader1_content::Content {}
                },
                (2, 3, 9, 3, 2) => rsx! {
                    Separator { class: "mb-4", horizontal: true, decorative: true }
                    section3::subheader2_content::Content {}
                },
                (2, 3, 9, 3, 3) => rsx! {
                    Separator { class: "mb-4", horizontal: true, decorative: true }
                    section3::subheader3_content::Content {}
                },
                _ => rsx! {
                    h3 { "get lost" }
                },
            }
        } else {
            h1 { "Subheader {subheader_id} not found." }
        }
    }
}
