use dioxus::prelude::*;

use crate::{
    components::RedNote, 
    utils::{h3_heading, text_command, TextCommandColor}
};

#[component]
pub fn Content() -> Element {
    rsx! {
        p {
            "Cisco switches actually use two different sets of interface status codes—one set of two
            codes (words) that use the same conventions as do router interface status codes, and another
            set with a single code (word)."
            br {}
            "Both sets of status codes can determine whether an interface is working."
        }
    }
}