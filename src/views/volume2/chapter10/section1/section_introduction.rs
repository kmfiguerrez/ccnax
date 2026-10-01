use dioxus::prelude::*;

use crate::{components::GreenNote, utils::h3_heading};

#[component]
pub fn SectionIntroductionContent() -> Element {
    rsx! {
        p { class: "mb-4",
            "The original design for the Internet required every organization to ask for, and receive, one
            or more registered classful IPv4 network numbers."
            br {}
            "The people administering the program ensured that none of the IP networks were reused."
            br {}
            "As long as every organization used only IP addresses inside its own registered network numbers, IP addresses would 
            never be duplicated, and IP routing could work well."
        }

        p { class: "mb-4",
            "Connecting to the Internet using only a registered network number, or several registered
            network numbers, worked well for a while."
            br {}
            "In the early to mid-1990s, it became apparent that the Internet was growing so fast that all IP network numbers would 
            be assigned by the mid-1990s!"
            br {}
            "Concern arose that the available networks would be completely assigned, and some organizations would not be able to 
            connect to the Internet."
        }

        {h3_heading("The Advent of IPV6")}
        p { class: "mb-4",
            "The main long-term solution to the IPv4 address scalability problem was to increase the size
            of the IP address."
            br {}
            "This one fact was the most compelling reason for the advent of IP version
            6 (IPv6)."
            br {}
            "(Version 5 was defined much earlier but was never deployed, so the next attempt
            was labeled as version 6.)"
            br {}
            strong { "IPv6 uses a 128-bit address, instead of the 32-bit address in IPv4." }
            br {}
            "With the same or improved process of assigning unique address ranges to every organization connected to the Internet, 
            IPv6 can easily support every organization and individual on the planet, with the number of IPv6 addresses 
            theoretically reaching above 10"
            sup { "38" }
            "."
        }

        {h3_heading("The short-term solutions")}
        p { class: "mb-4",
            "Many short-term solutions to the addressing problem were suggested, but three standards worked together to solve the 
            problem."
            br {}
            strong {
                "Two of the standards work closely together:
                Network Address Translation (NAT) and private addressing."
            }
            br {}
            "These features together allow many organizations to use the same unregistered IPv4 network numbers internally—and
            still communicate well with the Internet."
            br {}
            strong { "The third standard, classless interdomain routing (CIDR)" }
            ", allows ISPs to reduce the wasting of IPv4 addresses by 
            assigning a company a subset of a network number rather than the entire network."
            br {}
            "CIDR also can allow Internet service providers (ISP) to summarize routes such that multiple Class A, B, or C networks 
            match a single route, which helps reduce the size of Internet routing tables "
        }

        GreenNote {
            p {
                strong { "NOTE" }
                " These tools have worked well. Estimates in the early 1990s predicted that the
                world would run out of IPv4 addresses by the mid-1990s, but IANA did not exhaust the
                IPv4 address space until February 2011, and ARIN (the RIR for North America) did not
                exhaust its supply of public IPv4 addresses until September 2015."
            }
        }

    }
}