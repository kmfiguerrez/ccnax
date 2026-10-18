use dioxus::prelude::*;

use crate::utils::{h3_heading, text_command, TextCommandColor};

#[component]
pub fn Content() -> Element {
    rsx! {
        p { class: "mb-4",
            "When a router tries to route a packet, the router might not match the packet's destination IP
            address with any route."
            br {}
            "When that happens, the router normally just discards the packet."
        }

        p { class: "mb-4",
            strong {
                "Routers can be configured so that they use either a statically configured or dynamically
                learned default route."
            }
            br {}
            "The "
            i { "default route" }
            " matches all packets, so that if a packet does not match any other more specific route in the routing 
            table, the router can at least forward the packet based on the default route."
        }

        {h3_heading("Use case for a default route")}
        p { class: "mb-4",
            "One classic example in which companies might use static default routes in their enterprise
            TCP/IP networks is when the company has many remote sites, each with a single, relatively
            slow WAN connection."
            br {}
            "Each remote site has only one possible physical route to use to send packets to the rest of the network."
            br {}
            strong {
                "So, rather than use a routing protocol, which sends messages over the WAN and uses precious WAN bandwidth, each remote 
                router might use a default route that sends all traffic to the central site"
            }
            ", as shown in Figure 16-14."
        }

        img {
            class: "mb-4 rounded-lg",
            alt: "Figure 16-14 Example Use of Static Default Routes at 1000 Low-Speed Remote Sites",
            loading: "lazy",
            src: asset!("/assets/static/v1p5c16s3sh4ef16-14.png", AssetOptions::image().with_avif()),
        }

        {h3_heading("Static default route configuration")}
        p { class: "mb-4",
            "IOS allows the configuration of a static default route by using special values for the subnet
            and mask fields in the "
            {text_command("ip route", TextCommandColor::Gold)}
            " command: 0.0.0.0 and 0.0.0.0."
            br {}
            "For example, the command "
            {text_command("ip route 0.0.0.0 0.0.0.0 S0/0/1", TextCommandColor::Gold)}
            " creates a static default route on Router B1—a route that matches all IP packets—and sends those packets out interface 
            S0/0/1."
        }

        p { class: "mb-4", "See Example 16-6 in volume 1 on page 384." }

        {h3_heading("Facts in the output of the show ip route command")}
        p { class: "mb-4",
            "The output of the "
            {text_command("show ip route", TextCommandColor::Gold)}
            " command lists a few new and interesting facts."
            br {}
            "First, it lists the route with a code of S, meaning static, but also with a *, meaning it is a candidate default route."
            br {}
            strong {
                "A router can learn about more than one default route, and the router then has
                to choose which one to use; the * means that it is at least a candidate to become the default
                route."
            }
            br {}
            "Just above, "
            strong { "the “Gateway of Last Resort” refers to the chosen default route" }
            ", which in this case is the just-configured static route with outgoing interface S0/0/1."
        }

        {h3_heading("RECAP")}
        ol { class: "list-disc list-inside",
            li {
                "When a router tries to route a packet, the router might not match the packet's destination IP
                address with any route. When that happens, the router normally just discards the packet."
            }
            li { "Default routes can be configured statically or dynamically learned." }
            li {
                "The "
                i { "default route" }
                " matches all packets or destinations, so that if a packet does not match any other more specific route in the 
                routing table, the router can at least forward the packet based on the default route."
            }
            li {
                "Routes with a route code of S mean static but also with a *, meaning it is a candidate default route."
            }
            li {
                "A router can learn about more than one default route, and the router then has to choose which one to use; the * 
                means that it is at least a candidate to become the default route."
            }
            li { "The “Gateway of Last Resort” refers to the chosen default route." }
        }
    }
}
