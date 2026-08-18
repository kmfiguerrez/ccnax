use dioxus::prelude::*;
use crate::{Route};

use crate::components::volume_card::VolumeCard;

/// The Home page component that will be rendered when the current route is `[Route::Home]`
#[component]
pub fn Home() -> Element {
    let nav = navigator();

    rsx! {
        h1 { class: "text-center mb-8 text-lg", "CCNA Topics" }
        div { class: "flex flex-col gap-y-4 items-center sm:flex-row sm:justify-center sm:gap-x-4",
            button {
                class: "text-left",
                onclick: move |_| {
                    nav.push(Route::Volume { volume_id: 1 });
                },
                VolumeCard { number: 1 }
                        // Link { to: Route::Volume { volume_id: 1 }, "Volume 1" }
            }
            button {
                class: "text-left",
                onclick: move |_| {
                    nav.push(Route::Volume { volume_id: 2 });
                },
                // Link { to: Route::Volume { volume_id: 2 }, "Volume 2" }
                VolumeCard { number: 2 }
            }
        }

    }
}
