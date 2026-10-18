use dioxus::prelude::*;

use crate::{
    components::{KeyTopic, GreenNote},
    utils::h3_heading
};

#[component]
pub fn Content() -> Element {
    rsx! {
        p { class: "mb-4",
            "Because you already saw the basics back in Chapter 3, this section collects the routing process
            into steps for reference."
            br {}
            "The steps use many specific Ethernet LAN terms discussed in Parts II and III of this book and some IP addressing terms 
            discussed in Part IV."
            br {}
            "The upcoming descriptions and example then discuss these summaries of routing logic to make sure that each step is 
            clear."
        }

        {h3_heading("Hosts routing logic")}
        p { class: "mb-1",
            "The routing process starts with the host that creates the IP packet."
            br {}
            "First, the host asks the question: Is the destination IP address of this new packet in my local subnet?"
            br {}
            strong {
                "The host uses its own IP address/mask to determine the range of addresses in the local subnet."
            }
            br {}
            "Based on its own opinion of the range of addresses in the local subnet, a LAN-based host acts as follows:"
        }
        KeyTopic {}
        ol { class: "mb-4",
            li {
                span { class: "font-bold text-blue-500 mr-4", "Step 1." }
                "If the destination is local, send directly:"
                ol { class: "pl-16",
                    li { class: "flex gap-x-2",
                        span { class: "font-semibold text-blue-500 uppercase", "a." }
                        span {
                            "Find the destination host's MAC address. 
                            Use the already-known Address Resolution Protocol (ARP) table entry, or use ARP messages to learn the 
                            information."
                        }
                    }
                    li { class: "flex gap-x-2",
                        span { class: "font-semibold text-blue-500 uppercase", "b." }
                        span {
                            "Encapsulate the IP packet in a data-link frame, with the destination data-link
                            address of the destination host."
                        }
                    }
                }
            }
            li {
                span { class: "font-bold text-blue-500 mr-4", "Step 2." }
                "If the destination is not local, send to the default gateway:"
                ol { class: "pl-16",
                    li { class: "flex gap-x-2",
                        span { class: "font-semibold text-blue-500 uppercase", "a." }
                        span {
                            "Find the default gateway's MAC address. Use the already-known Address Resolution Protocol (ARP) 
                            table entry, or use ARP messages to learn the information."
                        }
                    }
                    li { class: "flex gap-x-2",
                        span { class: "font-semibold text-blue-500 uppercase", "b." }
                        span {
                            "Encapsulate the IP packet in a data-link frame, with the destination data-link
                            address of the default gateway."
                        }
                    }
                }
            }
        }

        p { class: "mb-4",
            "Figure 16-1 summarizes these same concepts."
            br {}
            "In the figure, host A sends a local packet directly to host D."
            br {}
            "However, for packets to host B, on the other side of a router and therefore
            in a different subnet, host A sends the packet to its default router (R1)."
            br {}
            "(As a reminder, the terms "
            i { "default gateway and default router" }
            " are synonyms.)"
        }

        img {
            class: "mb-4 rounded-lg",
            alt: "Figure 16-1 Host Routing Logic Summary",
            loading: "lazy",
            src: asset!("/assets/static/v1p5c16s1sh1f16-1.png", AssetOptions::image().with_avif()),
        }

        {h3_heading("Routers routing logic")}
        p { class: "mb-1",
            "Routers have a little more routing work to do as compared with hosts."
            br {}
            "While the host logic began with an IP packet sitting in memory, a router has some work to do before getting to
            that point."
            br {}
            "With the following five-step summary of a router's routing logic, the router takes the first two steps just to receive 
            the frame and extract the IP packet, before thinking about the packet's destination address at Step 3."
            br {}
            "The steps are as follows:"
        }
        KeyTopic {}
        ol { class: "mb-4",
            li {
                span { class: "font-bold text-blue-500 mr-4", "1." }
                "For each received data-link frame, choose whether or not to process the frame. Process it if"
                ol { class: "pl-8",
                    li { class: "flex gap-x-2",
                        span { class: "font-semibold text-blue-500 uppercase", "a." }
                        span {
                            "The frame has no errors (per the data-link trailer Frame Check Sequence [FCS] field)."
                        }
                    }
                    li { class: "flex gap-x-2",
                        span { class: "font-semibold text-blue-500 uppercase", "b." }
                        span {
                            "The frame's destination data-link address is the router's address (or an appropriate
                            multicast or broadcast address)."
                        }
                    }
                }
            }
            li { class: "flex gap-x-4",
                span { class: "font-bold text-blue-500", "2." }
                span {
                    "If choosing to process the frame at Step 1, de-encapsulate the packet from inside the data-link frame."
                }
            }
            li { class: "flex gap-x-4",
                span { class: "font-bold text-blue-500", "3." }
                span {
                    "Make a routing decision. To do so, compare the packet's destination IP address to the
                    routing table and find the route that matches the destination address. This route identifies the outgoing 
                    interface of the router and possibly the next-hop router."
                }
            }
            li { class: "flex gap-x-4",
                span { class: "font-bold text-blue-500", "4." }
                span {
                    "Encapsulate the packet into a data-link frame appropriate for the outgoing interface.
                    When forwarding out LAN interfaces, use ARP as needed to find the next device's MAC address."
                }
            }
            li { class: "flex gap-x-4",
                span { class: "font-bold text-blue-500", "5." }
                span { "Transmit the frame out the outgoing interface, as listed in the matched IP route." }
            }
        }

        p { class: "mb-4",
            strong {
                "This routing process summary lists many details, but sometimes you can think about the
                routing process in simpler terms."
            }
            br {}
            "For example, leaving out some details, this paraphrase of the step list details the same big concepts:"
            br {}
            br {}
            strong { class: "block pl-4",
                "The router receives a frame, removes the packet from inside the frame, decides where to
                forward the packet, puts the packet into another frame, and sends the frame."
            }
        }

        p { class: "mb-4",
            "To give you a little more perspective on these steps, Figure 16-2 breaks down the same fivestep routing process as a 
            diagram."
            br {}
            "The figure shows a packet arriving from the left, entering a router Ethernet interface, with an IP destination of host C."
            br {}
            "The figure shows the packet arriving, encapsulated inside an Ethernet frame (both header and trailer)."
        }

        KeyTopic {}
        img {
            class: "mb-4 rounded-lg",
            alt: "Figure 16-2 Router Routing Logic Summary",
            loading: "lazy",
            src: asset!("/assets/static/v1p5c16s1sh1f16-2.png", AssetOptions::image().with_avif()),
        }

        p { class: "mb-1",
            "Router R1 processes the frame and packet as shown with the numbers in the figure, matching the same five-step process 
            described just before the figure, as follows:"
        }
        ol { class: "mb-4",
            li { class: "flex gap-x-4",
                span { class: "font-bold text-blue-500", "1." }
                span {
                    "Router R1 notes that the received Ethernet frame passes the FCS check and that the
                    destination Ethernet MAC address is R1's MAC address, so R1 processes the frame."
                }
            }
            li { class: "flex gap-x-4",
                span { class: "font-bold text-blue-500", "2." }
                span {
                    "R1 de-encapsulates the IP packet from inside the Ethernet frame's header and trailer."
                }
            }
            li { class: "flex gap-x-4",
                span { class: "font-bold text-blue-500", "3." }
                span { "R1 compares the IP packet's destination IP address to R1's IP routing table." }
            }
            li { class: "flex gap-x-4",
                span { class: "font-bold text-blue-500", "4." }
                span {
                    "R1 encapsulates the IP packet inside a new data-link frame, in this case, inside a HighLevel Data Link 
                    Control (HDLC) header and trailer."
                }
            }
            li { class: "flex gap-x-4",
                span { class: "font-bold text-blue-500", "5." }
                span {
                    "R1 transmits the IP packet, inside the new HDLC frame, out the serial link on the right."
                }
            }
        }

        GreenNote {
            p {
                strong { "NOTE" }
                " This chapter uses several figures that show an IP packet encapsulated inside a datalink layer frame. 
                These figures often show both the data-link header as well as the data-link
                trailer, with the IP packet in the middle. The IP packets all include the IP header, plus any
                encapsulated data."
            }
        }

        {h3_heading("RECAP")}
        ol { class: "list-disc list-inside",
            li {
                "The host uses its own IP address/mask to determine the range of addresses in the local subnet 
                and to figure out if the destination address is in the local subnet."
            }
        }

    }
}