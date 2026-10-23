use dioxus::prelude::*;

use crate::{components::KeyTopic, utils::h3_heading};

#[component]
pub fn Content() -> Element {
    rsx! {
        p { class: "mb-4",
            strong { "Normally, one AP cannot cover the entire area where clients might be located." }
            br {}
            "For example, you might need wireless coverage throughout an entire floor of a business, hotel, hospital, or
            other large building."
            br {}
            strong {
                "To cover more area than a single AP's cell can cover, you simply need to add more APs and spread them out 
                geographically."
            }
        }

        {h3_heading("Interconnected APs")}
        p { class: "mb-4",
            "When APs are placed at different geographic locations, they can all be interconnected by a switched infrastructure."
            br {}
            strong { "The 802.11 standard calls this an extended service set (ESS)" }
            ", as shown in Figure 26-8."
        }

        {h3_heading("Consitent SSIDs on APs in an ESS")}
        p { class: "mb-4",
            "The idea is to make multiple APs cooperate so that the wireless service is consistent and seamless from the client's 
            perspective."
            br {}
            strong {
                "Ideally, any SSIDs that are defined on one AP should be defined on all the APs in an ESS; otherwise, it would be very 
                cumbersome and inconvenient for a client to be reconfigured each time it moves into a different AP's cell."
            }
        }

        {h3_heading("APs have unique BSSID within ESS")}
        p { class: "mb-4",
            "Notice that each cell in Figure 26-8 has a unique BSSID, but both cells share one common SSID."
            br {}
            strong {
                "Regardless of a client's location within the ESS, the SSID will remain the same but the client can always distinguish 
                one AP from another."
            }
        }

        {h3_heading("Roaming")}
        p { class: "mb-4",
            "In an ESS, a wireless client can associate with one AP while it is physically located near that AP."
            br {}
            "If the client later moves to a different location, it can associate with a different nearby AP automatically."
            br {}
            strong { "Passing from one AP to another is called roaming." }
            br {}
            strong {
                "Keep in mind that each AP offers its own BSS on its own channel,  to prevent interference between the APs."
            }
            br {}
            "As a client device roams from one AP to another, it must scan the available channels to find a new AP (and BSS) to 
            roam toward."
            br {}
            "In effect, the client is roaming from BSS to BSS, and from channel to channel. "
        }

        KeyTopic {}
        img {
            class: "mb-4 rounded-lg",
            alt: "Figure 26-8 Scaling Wireless Coverage with an 802.11 Extended Service Set",
            loading: "lazy",
            src: asset!("/assets/static/v1p8c26s2sh3f26-8.png", AssetOptions::image().with_avif()),
        }

        {h3_heading("RECAP")}
        ol { class: "list-disc list-inside",
            li {
                "To cover more area than a single AP's cell can cover, you simply need to add more APs and spread them out 
                geographically."
            }
            li {
                "When APs are placed at different geographic locations, they can all be interconnected by a switched infrastructure. 
                The 802.11 standard calls this an extended service set (ESS)"
            }
            li {
                "APs within ESS should have the same SSIDs to provide seamless connections as users move between APs' cells."
            }
            li {
                "Note that within an ESS, each AP still has a unique BSSID even though they share one commond SSID. So, that 
                clients (802.11 STA) can always distinguish one AP from another."
            }
            li { "Passing from one AP to another is called roaming." }
            li {
                "Keep in mind that each AP offers its own BSS on its own channel, to prevent interference between the APs."
            }
        }
    }
}