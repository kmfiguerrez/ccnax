use dioxus::prelude::*;

use crate::{Route, utils::{db_models::Database}};
use crate::components::{svg::CaretRightSVG};


#[component]
pub fn Volume(volume_id: u32) -> Element {
    // Grab the database from context
    let db = use_context::<Signal<Database>>();
    let volume = db.read().get(&volume_id).cloned();

    rsx! {
        h1 { class: "text-lg font-bold mb-4", "Volume: {volume_id}" }
        if let Some(volume) = volume {
            ol { class: "flex flex-col gap-y-1",
                for (idx , part) in volume.parts {
                    li {
                        key: "{idx}",
                        class: "border border-zinc-600 w-fit py-2 px-4 rounded-lg flex items-center",
                        CaretRightSVG {}
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
