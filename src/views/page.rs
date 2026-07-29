use dioxus::prelude::*;

use crate::components::CcnaBookPage;

#[component]
pub fn Page() -> Element {
    rsx! {
        CcnaBookPage {}
    }
}