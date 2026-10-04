use dioxus::prelude::*;

use crate::utils::h3_heading;

#[component]
pub fn SectionIntroductionContent() -> Element {
    rsx! {
        p { class: "mb-4",
            strong {
                "IPv4 access control lists (IP ACL) give network engineers a way to identify different types of packets."
            }
            br {}
            "To do so, the ACL configuration lists values that the router can see in the IP,
            TCP, UDP, and other headers."
            br {}
            "For example, an ACL can match packets whose source IP address is 1.1.1.1, or packets whose destination IP address is 
            some address in subnet 10.1.1.0/24, or packets with a destination port of TCP port 23 (Telnet)."
        }

        p { class: "mb-4",
            strong {
                "IPv4 ACLs perform many functions in Cisco routers, with the most common use as a packet filter."
            }
            br {}
            "Engineers can enable ACLs on a router so that the ACL sits in the forwarding path of
            packets as they pass through the router."
            br {}
            "After it is enabled, the router considers whether each IP packet will either be discarded or allowed to continue as if 
            the ACL did not exist."
        }

        {h3_heading("Other Uses of ACLs")}
        p { class: "mb-4",
            "However, ACLs can be used for many other IOS features as well."
            br {}
            "As an example, "
            strong { "ACLs can be used to match packets for applying Quality of Service (QoS) features" }
            "."
            br {}
            "QoS allows a router to give some packets better service, and other packets worse service."
            br {}
            "For example, packets that hold digitized voice need to have very low delay, so ACLs can match voice packets,
            with QoS logic in turn forwarding voice packets more quickly than data packets."
        }

        {h3_heading("Packet Filtering")}
        p { class: "mb-4",
            "This first section introduces IP ACLs as used for packet filtering, focusing on these aspects
            of ACLs: the locations and direction in which to enable ACLs, matching packets by examining headers, 
            and taking action after a packet has been matched."
        }

        {h3_heading("RECAP")}
        ol { class: "list-disc list-inside",
            li {
                "IPv4 access control lists (IP ACL) give network engineers a way to identify different types of packets."
                " To do so, the ACL configuration lists values that the router can see in the IP, TCP, UDP, and other headers."
            }
            li {
                "IPv4 ACLs perform many functions in Cisco routers, with the most common use as a packet filter."
            }
            li {
                "ACLs can be used for many other IOS features as well, such as matching packets for 
                applying Quality of Service (QoS) features."
            }
        }
    }
}