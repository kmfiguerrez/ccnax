use dioxus::prelude::*;

use crate::utils::{ h3_heading, text_command, TextCommandColor };

#[component]
pub fn Content() -> Element {
    rsx! {
        p { class: "mb-4",
            "Dynamic NAT has some similarities and differences compared to static NAT."
            br {}
            "Like static NAT, the NAT router creates a one-to-one mapping between an inside local and inside global address,
            and changes the IP addresses in packets as they exit and enter the inside network."
            br {}
            "However, the mapping of an inside local address to an inside global address happens dynamically."
        }

        {h3_heading("NAT pool")}
        p { class: "mb-4",
            "Dynamic NAT sets up a pool of possible inside global addresses and defines matching criteria to determine which inside 
            local IP addresses should be translated with NAT."
            br {}
            "For example, in Figure 10-5, a pool of five inside global IP addresses has been established: 200.1.1.1
            through 200.1.1.5. NAT has also been configured to translate any inside local addresses that
            start with 10.1.1."
        }

        img {
            class: "mb-4 rounded-lg",
            loading: "lazy",
            alt: "Figure 10-5 Dynamic NAT",
            src: asset!("/assets/static/v2p3c10s2sh2t10-5.png", AssetOptions::image().with_avif()),
        }

        p { class: "mb-1",
            "The numbers 1, 2, 3, and 4 in the figure refer to the following sequence of events:"
        }
        ol { class: "list-decimal list-inside mb-4",
            li { "Host 10.1.1.1 sends its first packet to the server at 170.1.1.1." }
            li {
                "As the packet enters the NAT router, the router applies some matching logic to
                decide whether the packet should have NAT applied. 
                Because the logic has been configured to match source IP addresses that begin with 10.1.1, the router adds an 
                entry in the NAT table for 10.1.1.1 as an inside local address."
            }
            li {
                "The NAT router needs to allocate an IP address from the pool of valid inside global
                addresses. It picks the first one available (200.1.1.1, in this case) and adds it to the
                NAT table to complete the entry."
            }
            li { "The NAT router translates the source IP address and forwards the packet." }
        }

        {h3_heading("Timeouts and clearing the NAT table entries")}
        p { class: "mb-4",
            "The dynamic entry stays in the table as long as traffic flows occasionally."
            br {}
            "You can configure a timeout value that defines how long the router should wait, having not translated any
            packets with that address, before removing the dynamic entry."
            br {}
            "You can also manually clear the dynamic entries from the table using the "
            {text_command("clear ip nat translation *", TextCommandColor::Gold)}
            " command."
        }

        p { class: "mb-4",
            "NAT can be configured with more IP addresses in the inside local address list than in the
            inside global address pool."
            br {}
            strong { "The router allocates addresses from the pool until all are allocated." }
            br {}
            strong {
                "If a new packet arrives from yet another inside host, and it needs a NAT entry, but all
            the pooled IP addresses are in use, the router simply discards the packet."
            }
            br {}
            "The user must try again until a NAT entry times out, at which point the NAT function works for the next host
            that sends a packet."
            br {}
            strong {
                "Essentially, the inside global pool of addresses needs to be as large as the maximum number of concurrent hosts that 
                need to use the Internet at the same time—unless you use PAT"
            }
            ", as is explained in the next section."
        }

        {h3_heading("RECAP")}
        ol { class: "list-disc list-inside",
            li {
                "Like static NAT, the NAT router creates a one-to-one mapping between an inside local and inside global address,
                and changes the IP addresses in packets as they exit and enter the inside network in dynamic NAT."
            }
            li {
                "In dynamic NAT, the mapping of an inside local address to an inside global address happens dynamically."
            }
            li {
                "Dynamic NAT sets up a pool of possible inside global addresses and defines matching criteria to determine which 
                inside local IP addresses should be translated with NAT."
            }
            li { "The dynamic entry stays in the table as long as traffic flows occasionally." }
            li {
                "Dynamic entries each have a timeout value that defines how long the router should wait, having not translated 
                any packets with that address, before removing the dynamic entry."
            }
            li { "The router allocates addresses from the pool until all are allocated." }
            li {
                "Keep in mind that if a new packet arrives from yet another inside host, and it needs a NAT entry, 
                but all the pooled IP addresses are in use, the router simply discards the packet."
            }
            li { "Source NAT changes only the IP address of inside hosts." }
        }
    }
}