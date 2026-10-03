use dioxus::prelude::*;

use crate::{components::KeyTopic, utils::{TextCommandColor, text_command}};

#[component]
pub fn Content() -> Element {
    rsx! {
        p { class: "mb-4",
            "In this section, practice getting comfortable with the syntax of the access-list command,
            particularly with choosing the correct matching logic."
            br {}
            "These skills will be helpful when reading about extended and named ACLs in the next chapter."
        }

        p { class: "mb-1",
            "First, the following list summarizes some important tips to consider when choosing matching parameters to any "
            {text_command("access-list", TextCommandColor::Gold)}
            " command:"
        }

        KeyTopic {}
        ol { class: "list-disc list-inside mb-4",
            li { "To match a specific address, just list the address." }
            li {
                "To match any and all addresses, use the "
                {text_command("any", TextCommandColor::Gold)}
                " keyword."
            }
            li {
                "To match based only on the first one, two, or three octets of an address, use the
                0.255.255.255, 0.0.255.255, and 0.0.0.255 WC masks, respectively. Also, make the
                source (address) parameter have 0s in the wildcard octets (those octets with 255 in the
                wildcard mask)."
            }
            li {
                "To match a subnet, use the subnet ID as the source, and find the WC mask by subtracting the DDN subnet mask from 
                255.255.255.255."
            }
        }

        p { class: "mb-4",
            "Table 2-2 lists the criteria for several practice problems."
            br {}
            "Your job: Create a one-line standard ACL that matches the packets."
            br {}
            "The answers are listed in the section “Answers to Earlier Practice Problems,” later in this chapter."
        }

        p { class: "mb-4", "See Table 2-2 in volume 2 on page 40." }

    }
}