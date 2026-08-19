use dioxus::prelude::*;

use crate::{Route, utils::{db_models::Database}};
use crate::components::{svg::CaretRightSVG};

#[component]
pub fn Chapter(volume_id: u32, part_id: u32, chapter_id: u32) -> Element {
    let db = use_context::<Signal<Database>>();

    // If you chain it all in one line, the guard gets dropped at the semicolon, 
    // and Rust panics because your reference points to dropped memory.

    // 1. Store the read guard in a variable. 
    // This keeps the borrow alive until the component function finishes.
    let db_guard = db.read();

    // 2. Get an Option<&Chapter>.
    let chapter = db_guard.get(&volume_id)
        .and_then(|v| v.parts.get(&part_id))
        .and_then(|p| p.chapters.get(&chapter_id));
    
    rsx! {
        if let Some(chapter) = chapter {
            h1 { class: "text-lg font-bold mb-4", "Chapter {chapter_id}: {chapter.name}" }
            ol { class: "flex flex-col gap-y-1",
                for (idx , section) in chapter.sections.iter() {
                    li {
                        key: "{idx}",
                        class: "border border-zinc-600 w-fit py-2 px-4 rounded-lg flex items-center",
                        CaretRightSVG {}
                        Link {
                            to: Route::Section {
                                volume_id,
                                part_id,
                                chapter_id,
                                section_id: *idx,
                            },
                            "Section {chapter_id}.{idx}: {section.name}"
                        }
                    }
                }
            }
        } else {
            h2 { "Chapter {chapter_id} not found." }
        }
    }
}
