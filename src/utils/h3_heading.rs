use dioxus::prelude::*;

/// Returns an HTML h3 element.
/// The text is colored gold.
pub fn h3_heading(text: &str) -> Element {
    rsx! {
        h3 { class: "font-semibold underline underline-offset-4 mb-1", "{text}" }
    }
}