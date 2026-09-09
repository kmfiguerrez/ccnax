use dioxus::prelude::*;

/// Returns an HTML h3 element.
/// The text is colored gold.
pub fn h3_heading(text: &str) -> Element {
    rsx! {
        // I make the display properties responsive because this heading is being used inside
        // the accordion trigger.
        h3 { class: "text-lg font-semibold underline underline-offset-4 mb-1 inline sm:block",
            "{text}"
        }
    }
}