use crate::components::CcnaBookPage;
use crate::components::button::Button;

use crate::components::dialog::{Dialog, DialogDescription, DialogTitle};
use crate::components::input::Input;
use dioxus::prelude::*;

#[css_module("/src/components/dialog/style.css")]
struct Styles;

#[component]
pub fn Demo() -> Element {
    let mut open = use_signal(|| false);

    rsx! {
        Button {
            r#type: "button",
            "data-style": "outline",
            onclick: move |_| open.set(true),
            "Show Dialog"
        }
        Dialog { open: open(), on_open_change: move |v| open.set(v),
            button {
                class: Styles::dx_dialog_close,
                r#type: "button",
                aria_label: "Close",
                tabindex: if open() { "0" } else { "-1" },
                onclick: move |_| open.set(false),
                "x"
            }
            DialogTitle { "Item information" }
            DialogDescription { "Here is some additional information about the item." }
            // Input {}
            CcnaBookPage {}
        }
    }
}