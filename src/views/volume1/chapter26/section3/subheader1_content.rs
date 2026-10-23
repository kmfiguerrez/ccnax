use dioxus::prelude::*;

use crate::{components::{KeyTopic, GreenNote}, utils::h3_heading};

#[component]
pub fn Content() -> Element {
    rsx! {
        p { class: "mb-4",
            "Normally, each AP in a wireless network has a wired connection back to the DS or switched infrastructure."
            br {}
            "To extend wireless coverage beyond a normal AP's cell footprint, additional APs and their wired connections can be 
            added."
            br {}
            strong {
                "In some scenarios, it is not possible to run a wired connection to a new AP because the cable distance is too great to 
                support Ethernet communication."
            }
        }

        p { class: "mb-4",
            "In that case, you can add an additional AP that is configured for repeater mode."
            br {}
            strong {
                "A wireless repeater takes the signal it receives and repeats or retransmits it in a new cell area around the repeater."
            }
            br {}
            "The idea is to move the repeater out away from the AP so that it is still within range of both the AP and the distant 
            client, as shown in Figure 26-10."
        }

        img {
            class: "mb-4 rounded-lg",
            alt: "Figure 26-4 802.11 Basic Service Set",
            loading: "lazy",
            src: asset!("/assets/static/v1p8c26s2sh1f26-4.png", AssetOptions::image().with_avif()),
        }

        p { class: "mb-4",
            "If the repeater has a single transmitter and receiver, it must operate on the same channel that the AP is using."
            br {}
            strong {
                "That can create the possibility that the AP's signal will be received and retransmitted by the repeater, only to be 
                received again by the AP—halving the effective throughput because the channel will be kept busy twice as long as before."
            }
            br {}
            "As a remedy, some repeaters can use two transmitters and receivers to keep the original and repeated signals isolated 
            on different channels."
            br {}
            strong {
                "One transmitter and receiver pair is dedicated to signals in the AP's cell, while the other pair is dedicated to 
                signals in the repeater's own cell."
            }
        }

        {h3_heading("RECAP")}
        ol { class: "list-disc list-inside",
            li {
                "A wireless repeater takes the signal it receives and repeats or retransmits it in a new cell area around the repeater."
            }
            li {
                "If the repeater has a single transmitter and receiver, it must operate on the same channel that the AP is using. 
                But will cause interference with the AP."
            }
            li {
                "If the repeater has a single transmitter and receiver, it must operate on the same channel that the AP is using. 
                But will cause interference with the AP."
            }
            li {
                "Some repeaters prevent interference by using two transmitters and receivers to keep the original and repeated 
                signals isolated on different channels."
            }
        }
    }
}