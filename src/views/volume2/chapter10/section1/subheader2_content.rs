use dioxus::prelude::*;

use crate::{components::KeyTopic, utils::h3_heading};


#[component]
pub fn Content() -> Element {
    rsx! {
        p { class: "mb-4",
            "Some computers might never be connected to the Internet."
            br {}
            "These computers' IP addresses could be duplicates of registered IP addresses in the Internet."
            br {}
            "When designing the IP addressing convention for such a network, an organization could pick and use any network
            number(s) it wanted, and all would be well."
            br {}
            "For example, you can buy a few routers, connect them in your office, and configure IP addresses in 
            network 1.0.0.0, and it would work."
            br {}
            "The IP addresses you use might be duplicates of real IP addresses in the Internet, but if all
            you want to do is learn on the lab in your office, everything will be fine."
        }

        {h3_heading("Private Internets")}
        p { class: "mb-4",
            "When building a private network that will have no Internet connectivity, you can use IP
            network numbers called "
            i { "private internets" }
            ", as defined in RFC 1918, “Address Allocation
            for Private Internets.”"
            br {}
            strong {
                "This RFC defines a set of networks that will never be assigned to any organization as a registered network number."
            }
            br {}
            "Instead of using someone else's registered network numbers, you can use numbers in a range that are not used by 
            anyone else in the public Internet."
            br {}
            "Table 10-2 shows the private address space defined by RFC 1918 ."
        }

        KeyTopic {}
        img {
            class: "mb-4 rounded-lg",
            loading: "lazy",
            alt: "Table 10-2 RFC 1918 Private Address Space",
            src: asset!("/assets/static/v2p3c10s1sh2t10-2.png", AssetOptions::image().with_avif()),
        }

        p { class: "mb-4",
            "In other words, any organization can use these network numbers."
            br {}
            "However, no organization is allowed to advertise these networks using a routing protocol on the Internet."
        }

        p { class: "mb-4",
            "Table 10-3 summarizes these important features that have helped extend the life of IPv4 by decades."
        }

        img {
            class: "mb-1 rounded-lg",
            loading: "lazy",
            alt: "Table 10-3 Three Important Functions That Extended the Life of IPv4",
            src: asset!("/assets/static/v2p3c10s1sh2t10-3.png", AssetOptions::image().with_avif()),
        }
        p { class: "text-sm mb-4",
            "*CIDR and NAT may be better known for their original RFCs (1518, 1519 for CIDR; 1631 for NAT)."
        }

        {h3_heading("RECAP")}
        ol { class: "list-disc list-inside",
            li { "RFC 1918 defined private IP address ranges called private internets." }
            li {
                "Any organization can use private IP address ranges.
                However, no organization is allowed to advertise these networks using a routing protocol on the Internet."
            }
        }
    }
}