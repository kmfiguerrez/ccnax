use dioxus::prelude::*;
use crate::{Route, components::{CcnaBookPage}};

// const NAVBAR_CSS: Asset = asset!("/assets/styling/navbar.css");

/// The Navbar component that will be rendered on all pages of our app since every page is under the layout.
///
///
/// This layout component wraps the UI of [Route::Home] and [Route::Blog] in a common navbar. The contents of the Home and Blog
/// routes will be rendered under the outlet inside this component
#[component]
pub fn Navbar(volume_id: u32) -> Element {
    rsx! {
        // document::Link { rel: "stylesheet", href: NAVBAR_CSS }
        nav {
            id: "navbar",
            class: "fixed w-full bg-zinc-900 bg-linear-to-b from-zinc-400/30",
            // Link { to: Route::Blog { id: 5 }, "Blog" }
            div { class: "flex justify-between px-1 py-4 container-x sm:justify-start sm:gap-x-4",
                GoBackButton {
                    span { class: "cursor-pointer", "Previous" }
                }
                Link { to: Route::Home {}, "Home" }
                CcnaBookPage {}
            }
        }

        // The `Outlet` component is used to render the next component inside the layout. In this case, it will render either
        // the [`Home`] or [`Blog`] component depending on the current route.
        Outlet::<Route> {}
    }
}
