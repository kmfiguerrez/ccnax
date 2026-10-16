use dioxus::prelude::*;

use crate::{
    components::{GreenNote, KeyTopic}, utils::{h3_heading, TextCommandColor, text_command}
};

#[component]
pub fn Content() -> Element {
    rsx! {
        p { class: "mb-1",
            "A Cisco router automatically adds a route to its routing table for the subnet connected to
            each interface, assuming that the following two facts are true:"
        }
        KeyTopic {}
        ol { class: "list-disc list-inside mb-4",
            li {
                "The interface is in a working state. In other words, the interface status in the "
                {text_command("show interfaces", TextCommandColor::Gold)}
                " command lists a line status of up and a protocol status of up."
            }
            li {
                "The interface has an IP address assigned through the "
                {text_command("ip address", TextCommandColor::Gold)}
                " interface subcommand."
            }
        }

        {h3_heading("The subnet ID")}
        p { class: "mb-4",
            "The concept of connected routes is relatively basic."
            br {}
            "The router, of course, needs to know the subnet number (subnet ID) connected to each of its interfaces, so the router can route 
            packets to that subnet."
            br {}
            strong {
                "The router does the math, taking the interface IP address and mask and calculating the subnet ID."
            }
            br {}
            "However, the router only needs that route when the interface is up and working, so the router includes a connected 
            route in the routing table only when the interface is working."
        }

        {h3_heading("The show ip route command")}
        p { class: "mb-4",
            "Example 16-1 shows the connected routes on Router R1 in Figure 16-10."
            br {}
            "The first part of the example shows the configuration of IP addresses on all three of R1's interfaces."
            br {}
            "The end of the example lists the output from the "
            {text_command("show ip route", TextCommandColor::Gold)}
            " command, which lists these routes with a "
            i { "c" }
            " as the route code, meaning connected."
        }

        img {
            class: "mb-4 rounded-lg",
            alt: "Figure 16-10 Sample Network to Show Connected Routes",
            loading: "lazy",
            src: asset!("/assets/static/v1p5c16s2sh1f16-10.png", AssetOptions::image().with_avif()),
        }

        p { class: "mb-4", "See Example 16-1 in volume 1 on page 377." }

        p { class: "mb-4",
            "Take a moment to look closely at each of the three highlighted routes in the output of "
            {text_command("show ip route", TextCommandColor::Gold)}
            "."
            br {}
            "Each lists a C in the first column, and each has text that says “directly connected”;
            both identify the route as connected to the router."
            br {}
            "The early part of each route lists the matching parameters (subnet ID and mask), as shown in the earlier example in 
            Figure 16-7."
            br {}
            "The end of each of these routes lists the outgoing interface."
        }

        {h3_heading("The local route")}
        p { class: "mb-4",
            "Note that the router also automatically produces a different kind of route, called a "
            i { "local route" }
            "."
            br {}
            strong {
                "The local routes define a route for the one specific IP address configured on the router interface."
            }
            br {}
            strong {
                "Each local route has a /32 prefix length, defining a host route, which defines a route just for that one IP address."
            }
            br {}
            "For example, the last local route, for 172.16.5.1/32, defines a route that matches only the IP address of 172.16.5.1."
            br {}
            strong {
                "Routers use these local routes that list their own local IP addresses to more efficiently forward packets sent to the 
            router itself."
            }
        }

        {h3_heading("Terms to note for the exam")}
        p { class: "mb-1",
            "For the CCNA 200-301 exam, note that this example of the "
            {text_command("show ip route", TextCommandColor::Gold)}
            " command reveals a few of the specific 
            subitems within exam topic 3.1, with later examples revealing even more details."
            br {}
            "This section shows details related to the following terms from the exam topics: "
        }
        ol { class: "list-disc list-inside mb-4",
            li {
                span { class: "font-bold mr-2", "Routing Protocol Code:" }
                "The legend at the top of the "
                {text_command("show ip route", TextCommandColor::Gold)}
                " output (about nine lines) lists all the routing protocol codes 
                (exam topic 3.1.a). This book references the codes for connected routes (C), local (L), static (S), and OSPF (O)."
            }
            li {
                span { class: "font-bold mr-2", "Prefix:" }
                "The word prefix (exam topic 3.1.b) is just another name for subnet ID."
            }
            li {
                span { class: "font-bold mr-2", "Mask:" }
                "Each route lists a prefix (subnet ID) and network mask (exam topic 3.1.c) in prefix
                format, for example, /24."
            }
        }

        {h3_heading("RECAP")}
        ol { class: "list-disc list-inside",
            li {
                "Routers find the subnet ID of each a connected route by taking the interface IP address and mask and calculating 
                the subnet ID."
            }
            li { "Each route in the routing cable is listed with a route code in the first column." }
            li {
                "The local routes define a route for the one specific IP address configured on the router interface."
                " Each local route has a /32 prefix length, defining a host route, which defines a route just for that one IP 
                address."
            }
            li {
                "Routers use these local routes that list their own local IP addresses to more efficiently forward packets sent to 
                the router itself."
            }
            li { "The word prefix is just another name for subnet ID." }
            li { "Subnet mask in prefix format is called prefix length." }
        }

    }
}