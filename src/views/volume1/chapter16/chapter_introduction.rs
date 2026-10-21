use dioxus::prelude::*;

use crate::utils::{TextCommandColor, h3_heading, text_command};

#[component]
pub fn ChapterIntroductionContent() -> Element {
    rsx! {
        p { class: "mb-4",
            "Routers route IPv4 packets."
            br {}
            "That simple statement actually carries a lot of hidden meaning."
            br {}
            "For routers to route packets, routers follow a routing process."
            br {}
            "That routing process relies on information called IP routes."
            br {}
            "Each IP route lists a destination—an IP network, IP subnet, or some other group of IP addresses."
            br {}
            "Each route also lists instructions that tell the router where
            to forward packets sent to addresses in that IP network or subnet."
            br {}
            "For routers to do a good job of routing packets, routers need to have a detailed, accurate list of IP routes."
        }

        {h3_heading("Three methods of adding IP routes")}
        p { class: "mb-1",
            "Routers use three methods to add IPv4 routes to their IPv4 routing tables."
        }
        ol { class: "list-disc list-inside mb-4",
            li {
                "Routers first learn "
                i { "connected routes" }
                ", which are routes for subnets attached to a router interface."
            }
            li {
                "Routers can  also use "
                i { "static routes" }
                ", which are routes created through a configuration command ("
                {text_command("ip route", TextCommandColor::Gold)}
                ") that tells the router what route to put in the IPv4 routing table."
            }
            li {
                "And routers can use a routing protocol, in which routers tell each other about all their known routes, so that all 
                routers can learn and build routes to all networks and subnets."
            }
        }

        p { class: "mb-4",
            "This chapter examines IP routing in depth with the most straightforward routes that can
            be added to a router's routing table."
            br {}
            "The router starts with a detailed look at the IP packet routing (forwarding process)—a process that relies on each 
            router having useful IP routes in their routing tables."
            br {}
            "The second section then examines connected routes, which are routes
            to subnets that exist on the interfaces connected to the local router."
            br {}
            "The third section then examines static routes, which are routes the network engineer configures directly."
            br {}
            "The chapter ends with a section that looks more specifically at the IP routing process in a router, how
            it matches packets to the routing table, and how to interpret all the details in the output of
            the "
            {text_command("show ip route", TextCommandColor::Gold)}
            " command."
        }

        {h3_heading("RECAP")}
        ol { class: "list-disc list-inside",
            li {
                "Routers route IPv4 packets using a routing process that relies on information called IP routes."
            }
            li {
                "Each IP route lists a destination—an IP network, IP subnet, or some other group of IP addresses."
            }
            li {
                "Each route also lists instructions that tell the router where to forward packets sent to addresses in that IP 
                network or subnet."
            }
            li {
                "Routers use three methods to add IPv4 routes to their IPv4 routing tables: "
                i { "connected routes" }
                ", "
                i { "static routes" }
                ", and "
                i { "routing protocols." }
            }
        }
    }
}