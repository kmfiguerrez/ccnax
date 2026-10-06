use dioxus::prelude::*;

use crate::utils::h3_heading;

#[component]
pub fn Content() -> Element {
    rsx! {
        p { class: "mb-4",
            "CIDR is a global address assignment convention that defines how the Internet Assigned
            Numbers Authority (IANA), its member agencies, and ISPs should assign the globally
            unique IPv4 address space to individual organizations."
        }

        {h3_heading("Route Aggregation")}
        p { class: "mb-4",
            strong { "CIDR, defined in RFC 4632, has two main goals." }
            br {}
            "First, CIDR defines a way to assign public IP addresses, worldwide, to allow route aggregation or route summarization."
            br {}
            strong { "These route summaries greatly reduce the size of routing tables in Internet routers." }
        }

        p { class: "mb-4",
            "Figure 10-1 shows a typical case of CIDR route aggregation and how CIDR could be used
            to replace more than 65,000 routes with one route."
            br {}
            "First, imagine that ISP 1 owns Class C networks 198.0.0.0 through 198.255.255.0—not by accident, but by purposeful 
            and thoughtful design to make this route aggregation example possible."
            br {}
            "In other words, IANA allocated all addresses that begin with 198 to one of the five Regional Internet Registries
            (RIR), and that RIR assigned this entire range to one big ISP in that part of the world."
        }

        img {
            class: "mb-4 rounded-lg",
            loading: "lazy",
            alt: "Figure 10-1 Typical Use of CIDR",
            src: asset!("/assets/static/v2p3c10s1sh1f10-1.png", AssetOptions::image().with_avif()),
        }

        p { class: "mb-4",
            "The assignment of all addresses that begin with 198 to one ISP lets other ISPs use one
            route—a route for 198.0.0.0/8—to match all those addresses, forwarding packets for
            those addresses to ISP1."
            br {}
            "Figure 10-1 shows the ISPs on the left each with one route to
            198.0.0.0/8—in other words, a route to all hosts whose IP address begins with 198."
            br {}
            "This one summary route will match packets sent to all addresses in the 65,536 Class C IP networks
            that begin with 198."
        }

        {h3_heading("Preventing Unused IP Address Space")}
        p { class: "mb-1",
            "The second major CIDR feature allows RIRs and ISPs to reduce waste by assigning a subset
            of a classful network to a single customer."
            br {}
            "For example, imagine that ISP1's customer A needs only 10 IP addresses and that customer B needs 25 IP addresses."
            br {}
            "ISP1 does something like this:"
        }
        ol { class: "list-disc list-inside mb-4",
            li {
                "Assign customer A CIDR block 198.8.3.16/28, with 14 assignable addresses (198.8.3.17 to 198.8.3.30)."
            }
            li {
                "Assign customer B CIDR block 198.8.3.32/27, with 30 assignable addresses (198.8.3.33 to 198.8.3.62)."
            }
        }

        {h3_heading("CIDR Blocks")}
        p { class: "mb-4",
            "These "
            i { "CIDR blocks" }
            " act very much like a public IP network; in particular, they give each
            company a consecutive set of public IPv4 addresses to use."
            br {}
            "The public address assignment process has much less waste than before as well."
            br {}
            strong {
                "In fact, most public address assignments for the last 20 years have been a CIDR block rather than an 
                entire class A, B, or C network."
            }
        }

        {h3_heading("RECAP")}
        ol { class: "list-disc list-inside",
            li { "CIDR is a global address assignment convention." }
            li {
                "CIDR, defined in RFC 4632, has two main goals: route aggregation and to prevent unused IP address space."
            }
        }
    }
}