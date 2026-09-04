use dioxus::prelude::*;

/// Returns an HTML span element.
/// The text is colored gold.
pub fn text_command(text: &str, color: TextCommandColor) -> Element {
    rsx! {
        span {
            class: match color {
                TextCommandColor::Gold => "text-amber-400 font-bold",
                TextCommandColor::Black => "text-black font-bold",
            },
            "{text}"
        }
    }
}

pub enum TextCommandColor {
    Gold,
    Black
}