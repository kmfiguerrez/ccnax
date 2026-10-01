use dioxus::prelude::*;

use crate::{components::KeyTopic, utils::h3_heading};

#[component]
pub fn Content() -> Element {
    rsx! {
        p { class: "mb-4",
            "Some networks need to have most, if not all, IP hosts reach the Internet."
            br {}
            "If that network uses private IP addresses, the NAT router needs a very large set of registered IP addresses."
            br {}
            "With static NAT, for each private IP host that needs Internet access, you need a publicly
            registered IP address, completely defeating the goal of reducing the number of public IPv4 
            addresses needed for that organization."
            br {}
            "Dynamic NAT lessens the problem to some degree, because every single host in an internetwork should seldom need 
            to communicate with the Internet at the same time."
            br {}
            "However, if a large percentage of the IP hosts in a network will need Internet access throughout that 
            company's normal business hours, NAT still requires a large number of registered IP addresses, again failing to 
            reduce IPv4 address consumption."
        }

        {h3_heading("Port Address Translation (PAT)")}
        p { class: "mb-4",
            "The NAT Overload feature, also called Port Address Translation (PAT), solves this problem."
            br {}
            "Overloading allows NAT to scale to support many clients with only a few public IP addresses."
        }

        p { class: "mb-4",
            strong {
                "The key to understanding how overloading works is to recall how hosts use TCP and User
                Datagram Protocol (UDP) ports."
            }
            br {}
            "To see why, first consider the idea of three separate TCP connections to a web server, from three different hosts, 
            as shown in Figure 10-6."
        }

        img {
            class: "mb-4 rounded-lg",
            loading: "lazy",
            alt: "Figure 10-6 Three TCP Connections from Three PCs",
            src: asset!("/assets/static/v2p3c10s2sh3f10-6.png", AssetOptions::image().with_avif()),
        }

        p { class: "mb-4",
            "Next, compare those three TCP connections in Figure 10-6 to three similar TCP connections, now with all three TCP 
            connections from one client, as shown in Figure 10-7."
            br {}
            "The server does realize a difference because the server sees the IP address and TCP port number
            used by the clients in both figures."
            br {}
            "However, "
            strong {
                "the server really does not care whether the TCP connections come from different hosts or the same host; the 
                server just sends and receives data over each connection."
            }
        }

        img {
            class: "mb-4 rounded-lg",
            loading: "lazy",
            alt: "Figure 10-7 Three TCP Connections from One PC",
            src: asset!("/assets/static/v2p3c10s2sh3f10-7.png", AssetOptions::image().with_avif()),
        }

        p { class: "mb-4",
            strong {
                "NAT takes advantage of the fact that, from a transport layer perspective, the server doesn't
            care whether it has one connection each to three different hosts or three connections to
            a single host IP address."
            }
            br {}
            "NAT overload (PAT) translates not only the address, but the port
            number when necessary, making what looks like many TCP or UDP flows from different
            hosts look like the same number of flows from one host."
            br {}
            "Figure 10-8 outlines the logic."
        }

        KeyTopic {}
        img {
            class: "mb-4 rounded-lg",
            loading: "lazy",
            alt: "Figure 10-8 NAT Overload (PAT)",
            src: asset!("/assets/static/v2p3c10s2sh3f10-8.png", AssetOptions::image().with_avif()),
        }

        {h3_heading("PAT with Layer 4 port numbers")}
        p { class: "mb-4",
            strong {
                "When PAT creates the dynamic mapping, it selects not only an inside global IP address but
                also a unique port number to use with that address."
            }
            br {}
            "The NAT router keeps a NAT table entry for every unique combination of inside local IP address and port, 
            with translation to the inside global address and a unique port number associated with the inside global
            address."
            br {}
            "And because the port number field has 16 bits, NAT overload can use more than 65,000 port numbers, allowing it to 
            scale well without needing many registered IP addresses—in many cases, needing only one inside global IP address."
        }

        p { class: "mb-4",
            strong {
                "Of the three types of NAT covered in this chapter so far, PAT is by far the most popular option."
            }
            br {}
            "Static NAT and Dynamic NAT both require a one-to-one mapping from the inside local to the inside global address."
            br {}
            "PAT significantly reduces the number of required registered IP addresses (public IP addresses) compared to these other 
            NAT alternatives."
        }

        {h3_heading("RECAP")}
        ol { class: "list-disc list-inside",
            li {
                "PAT solves the problem of limited public IP addresses by allowing multiple private IP hosts to share a single 
                public IP address."
            }
            li {
                "The key to understanding how overloading works is to recall how hosts use TCP and User Datagram Protocol (UDP) ports."
            }
            li {
                "NAT overload (PAT) translates not only the address, but the port number when necessary, making what looks like many TCP or 
                UDP flows from different hosts look like the same number of flows from one host."
            }
            li {
                "The NAT router keeps a NAT table entry for every unique combination of inside local IP address and port, with 
                translation to the inside global address and a unique port number associated with the inside global address."
            }
            li {
                "The port number field in layer 4 header has 16 bits, NAT overload can use more than 65,000 port numbers, allowing it to scale 
                well without needing many registered IP addresses—in many cases, needing only one inside global IP address."
            }
            li {
                "Static NAT and Dynamic NAT both require a one-to-one mapping from the inside local to the inside global address. 
                While PAT does not, it can use many-to-one mapping of multiple private IP addresses to a single public IP address."
            }
            li { "Source NAT changes only the IP address of inside hosts." }
        }
    }
}