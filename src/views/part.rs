use dioxus::prelude::*;

use crate::{Route, utils::{db_models::Database}};
use crate::components::{svg::CaretRightSVG};


#[component]
pub fn Part(volume_id: u32, part_id: u32) -> Element {
    let db = use_context::<Signal<Database>>();

    // If you chain it all in one line, the guard gets dropped at the semicolon, 
    // and Rust panics because your reference points to dropped memory.
    // let part = db.read().get(&volume_id).and_then(|v| v.parts.get(&part_id));

    // 1. Store the read guard in a variable. 
    // This keeps the borrow alive until the component function finishes.
    let db_guard = db.read();

    // 2. Get an Option<&Volume> by borrowing from the guard
    let volume = db_guard.get(&volume_id);
    
    // 3. Get an Option<&Part> (or whatever your part data is)
    let part = volume.and_then(|v| v.parts.get(&part_id));

    rsx! {
        if let Some(part) = part {
            h1 { class: "text-lg font-bold mb-4", "Part {part_id}: {part.name}" }
            ol { class: "flex flex-col gap-y-1",
                for (idx , chapter) in part.chapters.iter() {
                    li {
                        key: "{idx}",
                        class: "border border-zinc-600 w-fit py-2 px-4 rounded-lg flex items-center",
                        CaretRightSVG {}
                        Link {
                            to: Route::Chapter {
                                volume_id,
                                part_id,
                                chapter_id: *idx,
                            },
                            "Chapter {idx}: {chapter.name}"
                        }
                    }
                }
            }
        } else {
            h2 { "Part {part_id} not found." }
        }
    }
}
