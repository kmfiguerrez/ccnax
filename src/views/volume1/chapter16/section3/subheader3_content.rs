use dioxus::prelude::*;

use crate::utils::{h3_heading, text_command, TextCommandColor};

#[component]
pub fn Content() -> Element {
    rsx! {
        p { class: "mb-4",
            "Next, consider the case in which a static route competes with other static routes or routes
            learned by a routing protocol."
            br {}
            "That is, the "
            {text_command("ip route", TextCommandColor::Gold)}
            " command defines a route to a subnet, but the router also knows of other static or dynamically 
            learned routes to reach that same  subnet."
            br {}
            "In these cases, the router must first decide which routing source has the better administrative distance, with lower 
            being better, and then use the route learned from the better source."
        }

        {h3_heading("Routes over the same destination")}
        p { class: "mb-4",
            "To see how that works, consider the example illustrated in Figure 16-13, which shows a different design than in the 
            previous examples, this time with a branch office with two WAN
            links: one very fast Gigabit Ethernet link and one rather slow (but cheap) T1."
            br {}
            "In this design, the network uses Open Shortest Path First Version 2 (OSPFv2) over the primary link, learning a route 
            for subnet 172.16.2.0/24."
            br {}
            "R1 also defines a static route over the backup link to that exact same subnet, so R1 must 
            choose whether to use the static route or the OSPFlearned route."
        }

        img {
            class: "mb-4 rounded-lg",
            alt: "Figure 16-13 Using a Floating Static Route to Key Subnet 172.16.2.0/24",
            loading: "lazy",
            src: asset!("/assets/static/v1p5c16s3sh3f16-13.png", AssetOptions::image().with_avif()),
        }

        {h3_heading("Administrative Distance")}
        p { class: "mb-4",
            strong { "By default, IOS considers static routes better than OSPF-learned routes." }
            br {}
            "By default, IOS gives static routes an administrative distance of 1 and OSPF routes an administrative distance of 110."
            br {}
            "Using these defaults in Figure 16-13, R1 would use the T1 to reach subnet 172.16.2.0/24 in this case, which is not the 
            intended design."
            br {}
            "Instead, the engineer prefers to use the OSPFlearned routes over the much-faster primary link and use the static route 
            over the backup link only as needed when the primary link fails."
        }

        p { class: "mb-4",
            "To instead prefer the OSPF routes, the configuration would need to change the administrative distance settings and use 
            what many networkers call a floating static route."
            br {}
            "A "
            i { "floating static route" }
            " floats or moves into and out of the IP routing table depending on whether the
            better (lower) administrative distance route learned by the routing protocol happens to exist currently."
            br {}
            strong {
                "Basically, the router ignores the static route during times when the better routing protocol route is known."
            }
        }

        {h3_heading("Implementing a floating static route")}
        p { class: "mb-4",
            "To implement a floating static route, you need to use a parameter on the "
            {text_command("ip route", TextCommandColor::Gold)}
            " command that sets the administrative distance for just that route, making the value larger than the
            default administrative distance of the routing protocol."
            br {}
            "For example, the "
            {
                text_command(
                    "ip route 172.16.2.0 255.255.255.0 172.16.5.3 130",
                    TextCommandColor::Gold,
                )
            }
            " command on R1 would do exactly that—setting the static route's administrative distance to 130."
            br {}
            "As long as the primary link stays up, and OSPF on R1 learns a route for 172.16.2.0/24, with a default administrative 
            distance of 110, R1 ignores the static route."
        }

        p { class: "mb-4",
            "Finally, note that while the "
            {text_command("show ip route", TextCommandColor::Gold)}
            " command lists the administrative distance of most routes, as the first of two numbers inside two brackets, the "
            {text_command("show ip route", TextCommandColor::Gold)}
            i { " subnet " }
            "command plainly lists the administrative distance."
            br {}
            "Example 16-5 shows a sample, matching this most recent example."
        }

        img {
            class: "mb-4 rounded-lg",
            alt: "Example 16-5 Displaying the Administrative Distance of the Static Route",
            loading: "lazy",
            src: asset!("/assets/static/v1p5c16s3sh3ex16-5.png", AssetOptions::image().with_avif()),
        }

        {h3_heading("RECAP")}
        ol { class: "list-disc list-inside",
            li { "Routes with the lower administrative distance is the better route." }
            li { "By default, IOS considers static routes better than dynamically learned routes." }
            li {
                "By default, IOS gives static routes an administrative distance of 1 and OSPF routes an administrative distance of 110."
            }
            li {
                "A floating static route floats or moves into and out of the IP routing table depending on whether the better (lower) 
                administrative distance route learned by the routing protocol happens to exist currently."
            }
        }
    }
}