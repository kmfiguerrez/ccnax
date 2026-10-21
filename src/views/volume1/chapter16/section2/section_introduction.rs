use dioxus::prelude::*;

use crate::{components::KeyTopic, utils::{text_command, TextCommandColor}};

#[component]
pub fn SectionIntroductionContent() -> Element {
    rsx! {
        p { class: "mb-4",
            strong { "Cisco routers enable IPv4 routing globally, by default." }
            br {}
            "Then, to make the router be ready to route packets on a particular interface, the interface must be configured with 
            an IP address and the interface must be configured such that it comes up, reaching a “line status up, line protocol up”
            state."
            br {}
            "Only at that point can routers route IP packets in and out a particular interface."
        }

        p { class: "mb-1",
            "After a router can route IP packets out one or more interfaces, the router needs some routes."
            br {}
            "Routers can add routes to their routing tables through three methods:"
        }
        KeyTopic {}
        ol { class: "pl-4 mb-4",
            li {
                span { class: "font-bold mr-2", "Connected routes:" }
                "Added because of the configuration of the "
                {text_command("ip address", TextCommandColor::Gold)}
                " interface subcommand on the local router."
            }
            li {
                span { class: "font-bold mr-2", "Static routes:" }
                "Added because of the configuration of the "
                {text_command("ip route", TextCommandColor::Gold)}
                " global command on the local router."
            }
            li {
                span { class: "font-bold mr-2", "Routing protocols:" }
                "Added as a function by configuration on all routers, resulting in a process by which routers dynamically tell 
                each other about the network so that they all learn routes."
            }
        }

        p { class: "mb-4",
            "This second of three sections discusses several variations on how to configure connected
            routes, while the next major section discusses static routes."
        }
    }
}