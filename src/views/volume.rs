use dioxus::prelude::*;

use crate::{Route, utils::{db_models::Database}};

#[component]
pub fn Volume(volume_id: u32) -> Element {
    // Grab the database from context
    let db = use_context::<Signal<Database>>();
    let volume = db.read().get(&volume_id).cloned();

    rsx! {
        h1 { class: "text-lg font-bold mb-4", "Volume: {volume_id}" }
        if let Some(volume) = volume {
            ol {
                for (idx , part) in volume.parts {
                    li { key: "{idx}",
                        Link {
                            to: Route::Part {
                                volume_id,
                                part_id: idx,
                            },
                            "Part {idx}: {part.name}"
                        }
                    }
                }
            }
        } else {
            h2 { "Volume {volume_id} not found." }
        }
    }
}
