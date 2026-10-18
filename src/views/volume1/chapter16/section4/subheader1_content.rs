use dioxus::prelude::*;

use crate::{
    components::GreenNote,
    utils::{h3_heading, text_command, TextCommandColor}
};

#[component]
pub fn Content() -> Element {
    rsx! {
        p { class: "mb-4",
            "We humans have a couple of ways to figure out what choice a router makes for choosing the best route."
            br {}
            "One way uses the "
            {text_command("show ip route", TextCommandColor::Gold)}
            " command, plus some subnetting math, to decide the route the router will choose."
            br {}
            "To let you see how to use this option, Example 16-8 shows a series of overlapping routes, all created with OSPF, so 
            the output lists only OSPF-learned routes."
        }

        p { class: "mb-4", "See Example 16-8 in volume 1 on page 387." }

        p { class: "mb-4",
            strong {
                "To predict which of its routes a router will match, two pieces of information are required: the
                destination IP address of the packet and the contents of the router's routing table."
            }
            br {}
            "The subnet ID and mask listed for a route define the range of addresses matched by that route."
            br {}
            "With a little subnetting math, a network engineer can find the range of addresses matched by each route."
            br {}
            "For instance, Table 16-2 lists the five subnets listed in Example 16-8 and the address ranges implied by each."
        }

        img {
            class: "mb-4 rounded-lg",
            alt: "Table 16-2 Analysis of Address Ranges for the Subnets in Example 16-8",
            loading: "lazy",
            src: asset!("/assets/static/v1p5c16s4sh1ex16-2.png", AssetOptions::image().with_avif()),
        }

        GreenNote {
            p {
                strong { "NOTE" }
                " The route listed as 0.0.0.0/0 is the default route."
            }
        }

        p { class: "mb-4",
            "As you can see from these ranges, several of the routes' address ranges overlap."
            br {}
            strong { "When matching more than one route, the route with the longer prefix length is used." }
            br {}
            "That is, a route with /16 is better than a route with /10; a route with a /25 prefix is better than a route with a
            /20 prefix; and so on."
        }

        p { class: "mb-4",
            "For example, a packet sent to 172.16.1.1 actually matches all five routes listed in the routing
            table in Example 16-8."
            br {}
            "The various prefix lengths range from /0 to /32."
            br {}
            "The longest prefix (largest /P value, meaning the best and most specific route) is /32."
            br {}
            "So, a packet sent to 172.16.1.1 uses the route to 172.16.1.1/32, and not the other routes."
        }

        p { class: "mb-1",
            "The following list gives some examples of destination IP addresses."
            br {}
            "For each address, the list describes the routes from Table 16-2 that the router would match, and which specific 
            route the router would use."
        }
        ol { class: "pl-4 mb-4",
            li {
                span { class: "font-bold mr-1", "172.16.1.1:" }
                "Matches all five routes; the longest prefix is /32, the route to 172.16.1.1/32."
            }
            li {
                span { class: "font-bold mr-1", "172.16.1.2:" }
                "Matches the last four routes; the longest prefix is /24, the route to 172.16.1.0/24."
            }
            li {
                span { class: "font-bold mr-1", "172.16.2.3:" }
                "Matches the last three routes; the longest prefix is /22, the route to 172.16.0.0/22."
            }
            li {
                span { class: "font-bold mr-1", "172.16.4.3:" }
                "Matches the last two routes; the longest prefix is /16, the route to 172.16.0.0/16."
            }
        }

        {h3_heading("RECAP")}
        ol { class: "list-disc list-inside",
            li {
                "To predict which of its routes a router will match, two pieces of information are required: the destination IP 
                address of the packet and the contents of the router's routing table."
            }
            li {
                "The subnet ID and mask listed for a route define the range of addresses matched by that route."
            }
            li {
                "When routes' address ranges overlap,  the route with the longer prefix length is used."
            }
        }
    }
}