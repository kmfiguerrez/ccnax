use dioxus::prelude::*;

use crate::{components::KeyTopic, utils::h3_heading};

#[component]
pub fn SectionIntroductionContent() -> Element {
    rsx! {
        p { class: "mb-4",
            strong {
                "NAT , defined in RFC 3022, allows a host that does not have a valid, registered, globally
                unique IP address to communicate with other hosts through the Internet."
            }
            br {}
            "The hosts might be using private addresses or addresses assigned to another organization."
            br {}
            "In either case, NAT allows these addresses that are not Internet ready to continue to be used and still
            allows communication with hosts across the Internet."
        }

        p { class: "mb-4",
            strong {
                "NAT achieves its goal by using a valid registered IP address to represent the private address
                to the rest of the Internet."
            }
            br {}
            "The NAT function changes the private IP addresses to publicly registered IP addresses inside each IP packet, as 
            shown in Figure 10-2."
        }

        KeyTopic {}
        img {
            class: "mb-4 rounded-lg",
            loading: "lazy",
            alt: "Figure 10-2 NAT IP Address Swapping: Private Addressing",
            src: asset!("/assets/static/v2p3c10s2f10-2.png", AssetOptions::image().with_avif()),
        }

        p { class: "mb-4",
            strong {
                "Notice that the router, performing NAT, changes the packet's source IP address when the
                packet leaves the private organization."
            }
            br {}
            "The router performing NAT also changes the destination address in each packet that is forwarded back into the 
            private network."
            br {}
            "(Network 200.1.1.0 is a registered network in Figure 10-2.)"
            br {}
            "The NAT feature, configured in the router labeled NAT, performs the translation."
        }

        p { class: "mb-4",
            "This book discusses source NAT, which is the type of NAT that allows enterprises to use
            private addresses and still communicate with hosts in the Internet."
            br {}
            "Within source NAT, Cisco IOS supports several different ways to configure NAT."
            br {}
            "The next few topics cover the concepts behind several of these variations."
        }

        {h3_heading("RECAP")}
        ol { class: "list-disc list-inside",
            li { "RFC 3022 defined NAT." }
            li {
                "The NAT function changes the private IP addresses to publicly registered IP addresses inside each IP packet."
            }
        }
    }
}