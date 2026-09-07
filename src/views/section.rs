use dioxus::prelude::*;

use crate::{
    Route, 
    utils::db_models::Database, 
    components::{svg::CaretRightSVG, SectionIntroduction, separator::Separator}
};

#[component]
pub fn Section(volume_id: u32, part_id: u32, chapter_id: u32, section_id: u32) -> Element {
    // let current_route: Route = use_route();
    // let section_title = format_section_title(&name);

    let db = use_context::<Signal<Database>>();

    // If you chain it all in one line, the guard gets dropped at the semicolon, 
    // and Rust panics because your reference points to dropped memory.

    // 1. Store the read guard in a variable. 
    // This keeps the borrow alive until the component function finishes.
    let db_guard = db.read();

    // 2. Get an Option<&Section>.
    let section = db_guard.get(&volume_id)
        .and_then(|v| v.parts.get(&part_id))
        .and_then(|p| p.chapters.get(&chapter_id))   
        .and_then(|s| s.sections.get(&section_id));    
    
    rsx! {
        if let Some(section) = section {
            // Section Title
            h1 { class: "text-lg text-blue-500 font-bold mb-4",
                "Section {chapter_id}.{section_id}: {section.name}"
            }
            // Section Introduction
            SectionIntroduction {
                volume_id,
                part_id,
                chapter_id,
                section_id,
            }
            Separator { class: "my-4", horizontal: true, decorative: true }
            // Sections list
            ol { class: "flex flex-col gap-y-1",
                for (idx , subheader) in section.subheaders.iter() {
                    li {
                        key: "{idx}",
                        class: "border border-zinc-600 w-fit py-2 px-4 rounded-lg flex items-center",
                        CaretRightSVG {}
                        Link {
                            class: "font-semibold",
                            to: Route::Subheader {
                                volume_id,
                                part_id,
                                chapter_id,
                                section_id,
                                subheader_id: *idx,
                            },
                            "{subheader.name}"
                        }
                    }
                }
            }
        } else {
            h2 { "Section {section_id} not found." }
        }
    }
}
