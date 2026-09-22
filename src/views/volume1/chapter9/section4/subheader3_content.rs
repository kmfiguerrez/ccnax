use dioxus::prelude::*;

use crate::utils::{TextCommandColor, text_command};

#[component]
pub fn Content() -> Element {
    rsx! {
        p { class: "mb-4",
            "The "
            {text_command("show spanning-tree interface", TextCommandColor::Gold)}
            " focuses on interfaces participating in RSTP/STP."
            br {}
            "The command displays the role and state of the switch's interface specified in the command for each VLAN."
            br {}
            "You can use the command by listing the type of an interface plus some optionals on some verions of
            IOS."
            br {}
        }

        p { class: "mb-4",
            "For example, on a layer 2 switch, you can use just the "
            {
                text_command(
                    "show spanning-tree interface gigabitEhternet 0/1",
                    TextCommandColor::Gold,
                )
            }
            " alone, that lists general RSTP/STP information in a table like format, shown in example O-2."
            br {}
            "You can use the command with the "
            {text_command("detail", TextCommandColor::Gold)}
            " option to list more detailed information about the interface for each VLAN."
        
        }

        figure {
            figcaption { class: "text-sm",
                span { class: "font-bold", "Example O-2" }
                " Output of the show spanning-tree interface gigabitEhternet 0/1 command"
            }
            img {
                class: "mb-4 border border-gray-700 rounded-lg",
                alt: "Example O-2 Output of the show spanning-tree interface gigabitEhternet 0/1 command",
                loading: "lazy",
                src: asset!("/assets/static/v1p3c9s4sh2exo-2.png", AssetOptions::image().with_avif()),
            }
        }
    }
}