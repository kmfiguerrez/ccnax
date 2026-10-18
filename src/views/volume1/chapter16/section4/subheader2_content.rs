use dioxus::prelude::*;

use crate::{
    components::GreenNote,
    utils::{h3_heading, text_command, TextCommandColor}
};

#[component]
pub fn Content() -> Element {
    rsx! {
        p { class: "mb-4",
            "A second way to identify the route a router will use, one that does not require any subnetting math, is the "
            {text_command("show ip route", TextCommandColor::Gold)}
            i { " address " }
            "command."
            br {}
            "The last parameter on this command is the IP address of an assumed IP packet."
            br {}
            "The router replies by listing the route it would use to route a packet sent to that address."
        }

        p { class: "mb-4",
            "For example, Example 16-9 lists the output of the show ip route 172.16.4.3 command
            on the same router used in Example 24-4."
            br {}
            "The first line of (highlighted) output lists the matched route: the route to 172.16.0.0/16."
            br {}
            "The rest of the output lists the details of that particular route, like the outgoing interface of 
            GigabitEthernet0/1/0 and the next-hop router of 172.16.25.129."
        }

        img {
            class: "mb-4 rounded-lg",
            alt: "Example 16-9 show ip route Command with Overlapping Routes",
            loading: "lazy",
            src: asset!("/assets/static/v1p5c16s4sh2ex16-9.png", AssetOptions::image().with_avif()),
        }

        p { class: "mb-4",
            "Certainly, if you have an option, just using a command to check what the router actually
            chooses is a much quicker option than doing the subnetting math."
        }
    }
}