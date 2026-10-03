use dioxus::prelude::*;

use crate::{components::KeyTopic, utils::h3_heading};

#[component]
pub fn Content() -> Element {
    rsx! {
        p { class: "mb-4",
            strong {
                "Cisco routers can apply ACL logic to packets at the point at which the IP packets enter an
                interface, or the point at which they exit an interface."
            }
            br {}
            "In other words, the ACL becomes associated with an interface and for a direction of packet flow (either in or out)."
            br {}
            "That is, the ACL can be applied inbound to the router, before the router makes its forwarding (routing)
            decision, or outbound, after the router makes its forwarding decision and has determined
            the exit interface to use."
        }

        p { class: "mb-4",
            "The arrows in Figure 2-1 show the locations at which you could filter packets flowing left
            to right in the topology."
            br {}
            "For example, imagine that you wanted to allow packets sent by host A to server S1, but to discard packets sent by 
            host B to server S1."
            br {}
            "Each arrowed line represents a location and direction at which a router could apply an ACL, filtering the packets sent 
            by host B."
        }

        {h3_heading("Inbound vs. Outbound direction")}
        p { class: "mb-4",
            "The four arrowed lines in the figure point out the location and direction for the router
            interfaces used to forward the packet from host B to server S1."
            br {}
            "In this particular example, those interfaces and direction are inbound on R1's F0/0 interface, outbound on 
            R1's S0/0/0 interface, inbound on R2's S0/0/1 interface, and outbound on R2's F0/0 interface."
            br {}
            strong {
                "If, for example, you enabled an ACL on R2's F0/1 interface, in either direction, that ACL could
                not possibly filter the packet sent from host B to server S1, because R2's F0/1 interface is
                not part of the route from B to S1."
            }
        }

        img {
            class: "mb-4 rounded-lg",
            loading: "lazy",
            alt: "Figure 2-1 Locations to Filter Packets from Hosts A and B Going Toward Server S1",
            src: asset!("/assets/static/v2p3c2s1sh1f2-1.png", AssetOptions::image().with_avif()),
        }

        KeyTopic {}
        p { class: "mb-4",
            "In short, to filter a packet, you must enable an ACL on an interface that processes the packet, in the same direction 
            the packet flows through that interface."
        }

        p { class: "mb-4",
            strong {
                "When enabled, the router then processes every inbound or outbound IP packet using that ACL."
            }
            br {}
            "For example, if enabled on R1 for packets inbound on interface F0/0, R1 would compare every inbound IP packet on F0/0 
            to the ACL to decide that packet's fate: to continue unchanged or to be discarded."
        }

        {h3_heading("RECAP")}
        ol { class: "list-disc list-inside",
            li {
                "Cisco routers can apply ACL logic to packets at the point at which the IP packets enter an interface, or the 
                point at which they exit an interface."
            }
            li {
                "The ACL can be applied inbound to the router, before the router makes its forwarding (routing) decision, or 
                outbound, after the router makes its forwarding decision and has determined the exit interface to use."
            }
            li {
                "In short, to filter a packet, you must enable an ACL on an interface that processes the packet, in the same 
                direction the packet flows through that interface."
            }
            li {
                "When ACL is enabled, the router then processes every inbound or outbound IP packet using that ACL."
            }
        }
    }
}