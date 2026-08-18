use crate::{Route};
use dioxus::prelude::*;

/// The Home page component that will be rendered when the current route is `[Route::Home]`
#[component]
pub fn Home() -> Element {
    rsx! {
        div { class: "container-x",
            h1 { class: "text-center", "CCNA topics" }
            button {
                Link { to: Route::Volume { volume_id: 1 }, "Volume 1" }
            }
            br {}
            button {
                Link { to: Route::Volume { volume_id: 2 }, "Volume 2" }
            }
        }

    }
}
