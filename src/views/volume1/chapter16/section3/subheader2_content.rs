use dioxus::prelude::*;

use crate::utils::{h3_heading, text_command, TextCommandColor};

#[component]
pub fn Content() -> Element {
    rsx! {
        p { class: "mb-4",
            "Earlier, this chapter defined a host route as a route to a single host address."
            br {}
            "To configure such a static route, the "
            {text_command("ip route", TextCommandColor::Gold)}
            " command uses an IP address plus a mask of 255.255.255.255 so that the matching logic matches just that one address."
        }

        p { class: "mb-4",
            strong {
                "An engineer might use host routes to direct packets sent to one host over one path, with
                all other traffic to that host's subnet over some other path."
            }
            br {}
            "For instance, you could define these two static routes for subnet 10.1.1.0/24 and host 10.1.1.9, with two different 
            next-hop addresses, as follows:"
            br {}
            br {}
            "ip route 10.1.1.0 255.255.255.0 10.2.2.2"
            br {}
            "ip route 10.1.1.9 255.255.255.255 10.9.9.9"
        }

        p { class: "mb-4",
            strong {
                "Note that these two routes overlap: a packet sent to 10.1.1.9 that arrives at the router would match both routes."
            }
            br {}
            "When that happens, "
            strong {
                "routers use the most specific route (that is, the route with the longest prefix length)."
            }
            br {}
            "So, a packet sent to 10.1.1.9 would be forwarded to next-hop router 10.9.9.9, and packets sent to other destinations 
            in subnet 10.1.1.0/24 would be sent to next-hop router 10.2.2.2."
        }

        p { class: "mb-4",
            "Note that the section “IP Forwarding with the Longest Prefix Match” later in this chapter
            gets into this topic in more detail."
        }

        {h3_heading("RECAP")}
        ol { class: "list-disc list-inside",
            li {
                "To configure a static host route, the "
                {text_command("ip route", TextCommandColor::Gold)}
                " command uses an IP address plus a mask of 255.255.255.255 so that the matching logic matches just that one address."
            }
            li {
                "An engineer might use host routes to direct packets sent to one host over one path, with all other traffic to that 
                host's subnet over some other path."
            }
            li {
                "When routes overlap with a host route involved, routers use the most specific route (that is, the route with 
                the longest prefix length) and that is the host route."
            }
        }
    }
}