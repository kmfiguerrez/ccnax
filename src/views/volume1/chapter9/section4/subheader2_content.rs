use dioxus::prelude::*;

use crate::utils::{TextCommandColor , text_command};

#[component]
pub fn Content() -> Element {
    rsx! {
        p {
            "The "
            {text_command("show spanning-tree detail", TextCommandColor::Gold)}
            " list much more detailed RSTP/STP information per vlan."
            br {}
            "The command dispalys two sections of messages:"
        }
        ol { class: "list-disc list-inside mb-4",
            li {
                "The first section lists information you would see in the general RSTP/STP information in the output of "
                {text_command("show spanning-tree", TextCommandColor::Gold)}
                " command, plus more information that includes whether the VLAN uses RSTP-based or STP-based protocol, the number
                of topology changes, last topology change and which interface caused a topology change."
            }
            li {
                "The second section displays a list of RSTP/STP participating interfaces with detail information per interface.
                Like the role and state of each interface plus more information."
            }
        }

        p { class: "mb-4",
            "Example O-1 shows a sample output of the "
            {text_command("show spanning-tree detail", TextCommandColor::Gold)}
            "."
        }

        figure {
            figcaption { class: "text-sm",
                span { class: "font-bold", "Example O-1" }
                " Output of the show spanning-tree detail command"
            }
            img {
                class: "mb-4 border border-gray-700 rounded-lg",
                alt: "Example o-1 Output of the show spanning-tree detail",
                loading: "lazy",
                src: asset!("/assets/static/v1p3c9s4sh2exo-1.png", AssetOptions::image().with_avif()),
            }
        }
    }
}