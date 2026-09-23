use dioxus::prelude::*;

use crate::utils::{TextCommandColor, text_command};

#[component]
pub fn Content() -> Element {
    rsx! {
        p { class: "mb-4",
            "The "
            {text_command("show spanning-tree summary", TextCommandColor::Gold)}
            " gives you a summary instead of all the details."
            br {}
            "The command list the exact type of STP the switch is using i.e. pvst, rapid-pvst, STP features, 
            the number of VLANs using RSTP/STP and counters for interface states for each VLAN."
            br {}
            "Example O-3 displays the output of the "
            {text_command("show spanning-tree summary", TextCommandColor::Gold)}
            "."
        }

        figure {
            figcaption { class: "text-sm",
                span { class: "font-bold", "Example O-3" }
                " Output of the show spanning-tree summary command"
            }
            img {
                class: "mb-4 border border-gray-700 rounded-lg",
                alt: "Example O-3 Output of the show spanning-tree summary command",
                loading: "lazy",
                src: asset!("/assets/static/v1p3c9s4sh4exo-3.png", AssetOptions::image().with_avif()),
            }
        }
    }
}