use dioxus::prelude::*;

use crate::utils::{text_command, TextCommandColor};

#[component]
pub fn Content() -> Element {
    rsx! {
        p { class: "mb-4",
            strong { "When using IP ACLs to filter packets, only one of two actions can be chosen." }
            br {}
            "The configuration commands use the keywords "
            {text_command("deny", TextCommandColor::Gold)}
            " and "
            {text_command("permit", TextCommandColor::Gold)}
            ", and they mean (respectively) to discard the packet or to 
            allow it to keep going as if the ACL did not exist."
        }

        p { class: "mb-4",
            strong {
                "This book focuses on using ACLs to filter packets, but IOS uses ACLs for many more features."
            }
            br {}
            "Those features typically use the same matching logic."
            br {}
            "However, in other cases, the "
            {text_command("deny", TextCommandColor::Gold)}
            " or "
            {text_command("permit", TextCommandColor::Gold)}
            " keywords imply some other action."
        }
    }
}