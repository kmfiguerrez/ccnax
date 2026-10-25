use dioxus::prelude::*;

use crate::utils::h3_heading;

#[component]
pub fn Content() -> Element {
    rsx! {
        p { class: "mb-4",
            strong {
                "To provide wireless coverage over a very large area, it is not always practical to run Ethernet cabling to every AP 
            that would be needed."
            }
            br {}
            "Instead, you could use multiple APs configured in mesh mode."
            br {}
            strong {
                "In a mesh topology, wireless traffic is bridged from AP to AP, in a daisy-chain fashion, using another wireless 
                channel."
            }
        }

        p { class: "mb-4",
            strong {
                "Mesh APs can leverage dual radios—one using a channel in one range of frequencies and one a different range."
            }
            br {}
            "Each mesh AP usually maintains a BSS on one channel, with which wireless clients can associate."
            br {}
            "Client traffic is then usually bridged from AP to AP over other channels as a backhaul network."
            br {}
            "At the edge of the mesh network, the backhaul traffic is bridged to the wired LAN infrastructure."
            br {}
            "Figure 26-14 shows a typical mesh network."
            br {}
            strong { "With Cisco APs, you can build a mesh network indoors or outdoors." }
            br {}
            "The mesh network runs its own dynamic routing protocol to work out the best path for backhaul traffic to take 
            across the mesh APs."
        }

        img {
            class: "mb-4 rounded-lg",
            alt: "Figure 26-14 Typical Wireless Mesh Network",
            loading: "lazy",
            src: asset!("/assets/static/v1p8c26s3sh4f26-14.png", AssetOptions::image().with_avif()),
        }

        {h3_heading("RECAP")}
        ol { class: "list-disc list-inside",
            li {
                "To provide wireless coverage over a very large area, it is not always practical to run Ethernet cabling to every 
                AP that would be needed. Instead, you could use multiple APs configured in mesh mode."
            }
            li {
                "In a mesh topology, wireless traffic is bridged from AP to AP, in a daisy-chain fashion, using another wireless 
                channel."
            }
            li {
                "Each mesh AP usually maintains a BSS on one channel, with which wireless clients can associate."
            }
            li {
                "The mesh network runs its own dynamic routing protocol to work out the best path for backhaul traffic to take 
                across the mesh APs."
            }
        }
    }
}