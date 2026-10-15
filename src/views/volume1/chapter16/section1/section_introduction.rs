use dioxus::prelude::*;

use crate::{
    components::{KeyTopic, GreenNote},
    utils::h3_heading
};

#[component]
pub fn SectionIntroductionContent() -> Element {
    rsx! {
        p { class: "mb-4",
            "IP routing—the process of forwarding IP packets—delivers packets across entire TCP/IP networks, from the device that 
            originally builds the IP packet to the device that is supposed to receive the packet."
            br {}
            strong {
                "In other words, IP routing delivers IP packets from the sending host to the destination host."
            }
        }

        p { class: "mb-4",
            "The complete end-to-end routing process relies on network layer logic on hosts and on routers."
            br {}
            "The sending host uses Layer 3 concepts to create an IP packet, forwarding the IP packet
            to the host's default gateway (default router)."
            br {}
            "The process requires Layer 3 logic on the routers as well, by which the routers compare the destination address in the 
            packet to their routing tables, to decide where to forward the IP packet next."
        }

        p { class: "mb-4",
            "The routing process also relies on data-link and physical details at each link."
            br {}
            strong {
                "IP routing relies on serial WAN links, Ethernet WAN links, Ethernet LANs, wireless LANs, and many other
                networks that implement data-link and physical layer standards."
            }
            br {}
            "These lower-layer devices and protocols move the IP packets around the TCP/IP network by encapsulating and 
            transmitting the packets inside data-link layer frames."
        }

        p { class: "mb-4",
            "The previous two paragraphs summarize the key concepts about IP routing as introduced back
            in Chapter 3, “Fundamentals of WANs and IP Routing.”"
            br {}
            "Next, this section reviews IP routing, while taking the discussion another step or two deeper, taking advantage of 
            the additional depth of knowledge discussed in all the earlier chapters in this book."
        }
    }
}