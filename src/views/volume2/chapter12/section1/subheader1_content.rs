use dioxus::prelude::*;

use crate::{utils::h3_heading, components::GreenNote};

#[component]
pub fn Content() -> Element {
    rsx! {
        p { class: "mb-4",
            "Networks need redundant links to improve the availability of those networks."
            br {}
            "Eventually, something in a network will fail."
            br {}
            strong {
                "A router power supply might fail, or a cable might break, or a switch might lose power."
            }
            br {}
            "And those WAN links, shown as simple lines in most drawings in this book, are actually the most complicated physical 
            parts of the network, with many individual parts that can fail as well."
        }

        {h3_heading("Single point of failure")}
        p { class: "mb-4",
            "Depending on the design of the network, the failure of a single component might mean an
            outage that affects at least some part of the user population."
            br {}
            "Network engineers refer to any one component that, if it fails, brings down that part of the network as a "
            i { "single point of failure." }
            br {}
            "For instance, in Figure 12-1, the LANs appear to have some redundancy, whereas the WAN does not."
            br {}
            "If most of the traffic flows between sites, many single points of failure exist, as shown in the figure."
        }

        img {
            class: "mb-4 rounded-lg",
            loading: "lazy",
            alt: "Figure 12-1 R1 and the One WAN Link as Single Points of Failure",
            src: asset!("/assets/static/v2p3c12s1sh1f12-1.png", AssetOptions::image().with_avif()),
        }

        p { class: "mb-4",
            strong { "The figure notes several components as a single point of failure." }
            br {}
            "If any one of the noted parts of the network fails, packets cannot flow from the left side of the network to the right."
        }

        {h3_heading("Improving network availability")}
        p { class: "mb-1",
            strong {
                "Generally speaking, to improve availability, the network engineer first looks at a design and
                finds the single points of failure."
            }
            br {}
            "Then the engineer chooses where to add to the network so that one (or more) single point of failure now has redundant 
            options, increasing availability."
            br {}
            "In particular, the engineer"
        }
        ol { class: "list-disc list-inside mb-4 ",
            li { "Adds redundant devices and links" }
            li {
                "Implements any necessary functions that take advantage of the redundant device or link"
            }
        }

        p { class: "mb-4",
            "For instance, of all the single points of failure in Figure 12-1, the most expensive over the
            long term would likely be the WAN link because of the ongoing monthly charge."
            br {}
            "However, statistically, the WAN links are the most likely component to fail."
            br {}
            "So, a reasonable upgrade from the network in Figure 12-1 would be to add a WAN link and possibly even connect to
            another router on the right side of the network, as shown in Figure 12-2."
        }

        img {
            class: "mb-4 rounded-lg",
            loading: "lazy",
            alt: "Figure 12-2 Higher Availability but with R1 Still as a Single Point of Failure",
            src: asset!("/assets/static/v2p3c12s1sh1f12-2.png", AssetOptions::image().with_avif()),
        }

        p { class: "mb-4",
            "Many real enterprise networks follow designs like Figure 12-2, with one router at each
            remote site, two WAN links connecting back to the main site, and redundant routers at the
            main site (on the right side of the figure)."
            br {}
            strong {
                "Compared to Figure 12-1, the design in Figure 12-2 has fewer single points of failure."
            }
            br {}
            "Of the remaining single points of failure, a risk remains, but it is a calculated risk."
            br {}
            strong { "For many outages, a reload of the router solves the problem, and the outage is short." }
            br {}
            "But the risk still exists that the switch or router hardware fails completely and requires time to deliver a 
            replacement device on-site before that site can work again."
        }

        p { class: "mb-4",
            strong {
                "For enterprises that can justify more expense, the next step in higher availability for that
                remote site is to protect against those catastrophic router and switch failures."
            }
            br {}
            "In this particular design, adding one router on the left side of the network in Figure 12-2 removes all
            the single points of failure that had been noted earlier."
            br {}
            "Figure 12-3 shows the design with a second router, which connects to a different LAN switch so that SW1 is also no 
            longer a single point of failure."
        }

        GreenNote {
            p {
                strong { "NOTE" }
                " Medium to large enterprise networks work hard at striking a balance of high-availability features versus the 
                available budget dollars. Cisco.com has many design documents that discuss trade-offs in high-availability design. 
                If interested in learning more, search Cisco.com for “high availability campus network design.”"
            }
        }

        img {
            class: "mb-4 rounded-lg",
            loading: "lazy",
            alt: "Figure 12-3 Removing All Single Points of Failure from the Network Design",
            src: asset!("/assets/static/v2p3c12s1sh1f12-3.png", AssetOptions::image().with_avif()),
        }

        {h3_heading("RECAP")}
        ol { class: "list-disc list-inside",
            li {
                "Network engineers refer to any one component that, if it fails, brings down that part of the network as a "
                i { "single point of failure." }
            }
            li { "For many outages, a reload of the router solves the problem, and the outage is short." }
        }
    }
}