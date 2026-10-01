use dioxus::prelude::*;

use crate::{components::KeyTopic, utils::h3_heading};

#[component]
pub fn Content() -> Element {
    rsx! {
        p { class: "mb-4",
            "Static NAT works just like the example shown in Figure 10-2, but with the IP addresses
            statically mapped to each other."
            br {}
            "To help you understand the implications of static NAT and
            to explain several key terms, Figure 10-3 shows a similar example with more information."
        }

        img {
            class: "mb-4 rounded-lg",
            loading: "lazy",
            alt: "Figure 10-3 Static NAT Showing Inside Local and Global Addresses",
            src: asset!("/assets/static/v2p3c10s2sh1f10-3.png", AssetOptions::image().with_avif()),
        }

        p { class: "mb-4",
            "First , the concepts: The company's ISP has assigned it registered network 200.1.1.0."
            br {}
            "Therefore, the NAT router must make the private IP addresses look like they are in network 200.1.1.0."
            br {}
            "To do so, the NAT router changes the source IP addresses in the packets going from left to right in the figure."
        }

        {h3_heading("One-to-One Mapping")}
        p { class: "mb-4",
            "In this example, the NAT router changes the source address (SA in the figure) of 10.1.1.1
            to 200.1.1.1."
            br {}
            strong {
                "With static NAT, the NAT router simply configures a one-to-one mapping between the private address and the 
            registered address that is used on its behalf."
            }
            br {}
            "The NAT router has statically configured a mapping between private address 10.1.1.1 and public, registered address 200.1.1.1."
        }

        p { class: "mb-4",
            "Supporting a second IP host with static NAT requires a second static one-to-one mapping
            using a second IP address in the public address range."
            br {}
            "For example, to support 10.1.1.2, the router statically maps 10.1.1.2 to 200.1.1.2."
            br {}
            "Because the enterprise has a single registered Class C network, it can support at most 254 private IP addresses with 
            NAT, with the usual two reserved numbers (the network number and network broadcast address)."
        }

        {h3_heading("NAT Terminologies")}
        p { class: "mb-4",
            strong {
                "The terminology used with NAT, particularly with configuration, can be a little confusing."
            }
            br {}
            "Notice in Figure 10-3 that the NAT table lists the private IP addresses as “private” and
            the public, registered addresses from network 200.1.1.0 as “public.”"
            br {}
            "Cisco uses the term "
            i { "inside local" }
            " for the private IP addresses in this example and "
            i { "inside global" }
            " for the public IP addresses."
        }

        p { class: "mb-4",
            "Using NAT terminology, the enterprise network that uses private addresses, and therefore
            needs NAT, is the “inside” part of the network."
            br {}
            "The Internet side of the NAT function is the “outside” part of the network."
            br {}
            "A host that needs NAT (such as 10.1.1.1 in the example) has the IP address it uses inside the network, and it needs 
            an IP address to represent it in the outside network."
            br {}
            "So, because the host essentially needs two different addresses to represent
            it, you need two terms."
            br {}
            "Cisco calls the private IP address used in the inside network the "
            i { "inside local" }
            " address and the address used to represent the host to the rest of the Internet the "
            i { "inside global" }
            " address."
            br {}
            "Figure 10-4 repeats the same example, with some of the terminology shown."
        }

        KeyTopic {}
        img {
            class: "mb-4 rounded-lg",
            loading: "lazy",
            alt: "Figure 10-4 Static NAT Terminology",
            src: asset!("/assets/static/v2p3c10s2sh1f10-4.png", AssetOptions::image().with_avif()),
        }

        {h3_heading("Source NAT")}
        p { class: "mb-4",
            strong { "Source NAT changes only the IP address of inside hosts." }
            br {}
            "Therefore, the current NAT table shown in Figure 10-4 shows the inside local and corresponding inside global 
            registered addresses."
            br {}
            "The term inside local refers to the address used for the host inside the enterprise, the address used locally versus 
            globally, which means in the enterprise instead of the global Internet."
            br {}
            "Conversely, the term inside global still refers to an address used for the host
            inside the enterprise, but it is the global address used while the packet flows through the
            Internet."
        }

        {h3_heading("Destination NAT")}
        p { class: "mb-4",
            "Note that the NAT feature called "
            i { "destination NAT" }
            ", not covered in this book, uses similar terms "
            i { "outside local" }
            " and "
            i { "outside global" }
            "."
            br {}
            "However, with source NAT, one of the terms, "
            i { "outside global" }
            ", is used."
            br {}
            "This term refers to the host that resides outside the enterprise."
            br {}
            "Because source NAT does not change that address, the term outside global applies at all times."
        }

        p { class: "mb-4",
            "Table 10-4 summarizes these four similar terms and refers to the IPv4 addresses used as
            samples in the last three figures as examples."
        }

        KeyTopic {}
        img {
            class: "mb-4 rounded-lg",
            loading: "lazy",
            alt: "Table 10-4 NAT Addressing Terms",
            src: asset!("/assets/static/v2p3c10s2sh1t10-4.png", AssetOptions::image().with_avif()),
        }

        {h3_heading("RECAP")}
        ol { class: "list-disc list-inside",
            li {
                "With static NAT, the NAT router simply configures a one-to-one mapping between the private address and the 
                registered address that is used on its behalf."
            }
            li {
                "Cisco uses the term "
                i { "inside local" }
                " for the private IP addresses and "
                i { "inside global" }
                " for the public IP addresses."
            }
            li { "Source NAT changes only the IP address of inside hosts." }
        }

    }
}