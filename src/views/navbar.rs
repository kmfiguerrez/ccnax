use crate::{Route, components::demo::Demo};
use dioxus::prelude::*;

const NAVBAR_CSS: Asset = asset!("/assets/styling/navbar.css");

/// The Navbar component that will be rendered on all pages of our app since every page is under the layout.
///
///
/// This layout component wraps the UI of [Route::Home] and [Route::Blog] in a common navbar. The contents of the Home and Blog
/// routes will be rendered under the outlet inside this component
#[component]
pub fn Navbar() -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: NAVBAR_CSS }

        nav { id: "navbar", class: "mb-4",
            Link { to: Route::Home {}, "Home" }
            Link { to: Route::Page {}, "Page" }
            // Link { to: Route::Blog { id: 5 }, "Blog" }
            GoBackButton {
                span { class: "cursor-pointer", "Previous" }
            }
            Demo {}
        }

        // The `Outlet` component is used to render the next component inside the layout. In this case, it will render either
        // the [`Home`] or [`Blog`] component depending on the current route.
        Outlet::<Route> {}
    }
}
