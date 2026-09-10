use dioxus::prelude::*;

/// Returns an HTML h3 element.
/// The text is colored gold.
pub fn h4_heading(text: &str) -> Element {
    rsx! {
        h4 { class: "text-blue-400 font-semibold underline underline-offset-4 mb-1",
            "{text}"
        }
    }
}