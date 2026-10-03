use dioxus::prelude::*;

use crate::utils::h3_heading;


#[component]
pub fn Content() -> Element {
    rsx! {
        p { class: "mb-4",
            "When you think about the location and direction for an ACL, you must already be thinking
            about what packets you plan to filter (discard), and which ones you want to allow through."
            br {}
            "To tell the router those same ideas, you must configure the router with an IP ACL that
            matches packets."
            br {}
            i { "Matching packets" }
            " refers to how to configure the ACL commands to look at each packet, listing how to identify which 
            packets should be discarded and which should be allowed through."
        }

        p { class: "mb-4",
            strong {
                "Each IP ACL consists of one or more configuration commands, with each command listing
                details about values to look for inside a packet's headers."
            }
            br {}
            "Generally, an ACL command uses
            logic like “look for these values in the packet header, and if found, discard the packet.” (The
            action could instead be to allow the packet, rather than discard.)"
            br {}
            strong {
                "Specifically, the ACL looks for header fields you should already know well, including the source and destination IP
                addresses, plus TCP and UDP port numbers."
            }
        }

        p { class: "mb-4",
            "For example, consider an example with Figure 2-2, in which you want to allow packets from
            host A to server S1, but to discard packets from host B going to that same server."
            br {}
            "The hosts all now have IP addresses, and the figure shows pseudocode for an ACL on R2."
            br {}
            "Figure 2-2 also shows the chosen location to enable the ACL: inbound on R2's S0/0/1 interface."
        }

        p { class: "mb-4",
            "Figure 2-2 shows a two-line ACL in a rectangle at the bottom, with simple matching logic:
            both statements just look to match the source IP address in the packet."
            br {}
            "When enabled, R2 looks at every inbound IP packet on that interface and compares each packet to those two
            ACL commands."
            br {}
            "Packets sent by host A (source IP address 10.1.1.1) are allowed through,
            and those sourced by host B (source IP address 10.1.1.2) are discarded."
        }

        img {
            class: "mb-4 rounded-lg",
            loading: "lazy",
            alt: "Figure 2-2 Pseudocode to Demonstrate ACL Command-Matching Logic",
            src: asset!("/assets/static/v2p3c2s1sh2f2-2.png", AssetOptions::image().with_avif()),
        }

        {h3_heading("RECAP")}
        ol { class: "list-disc list-inside",
            li {
                i { "Matching packets" }
                " refers to how to configure the ACL commands to look at each packet, listing how to identify which 
                packets should be discarded and which should be allowed through."
            }
            li {
                "Each IP ACL consists of one or more configuration commands, with each command listing details about values to 
                look for inside a packet's headers."
            }
            li {
                "Specifically, the ACL looks for header fields you should already know well, including the source and destination 
                IP addresses, plus TCP and UDP port numbers."
            }
        
        }
    }
}