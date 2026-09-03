use dioxus::prelude::*;

/// Returns an HTML span element.
/// The text is colored gold.
pub fn text_command(text: &str) -> Element {
    rsx! {
        span { class: "text-amber-400 font-semibold", "{text}" }
    }
}