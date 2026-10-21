use dioxus::prelude::*;

use crate::{
    components::{
        GreenNote, KeyTopic, my_accordion::{Accordion, AccordionContent, AccordionItem, AccordionTrigger}
    }, utils::{h3_heading, h4_heading}
};

#[component]
pub fn Content() -> Element {
    rsx! {
        p { class: "mb-4",
            "The next several topics walk you through an example that discusses each routing step, in
            order, through multiple devices."
            br {}
            "The example uses a case in which host A (172.16.1.9) sends a packet to host B (172.16.2.9), with host routing logic 
            and the five steps showing how R1 forwards the packet."
        }

        p { class: "mb-4",
            "Figure 16-3 shows a typical IP addressing diagram for an IPv4 network with typical address abbreviations."
            br {}
            "The diagram can get a little too messy if it lists the full IP address for every router interface."
            br {}
            "When possible, these diagrams usually list the subnet and then the last octet or two of the individual IP 
            addresses—just enough so that you know the IP address but with less clutter."
            br {}
            "For example, host A uses IP address 172.16.1.9, taking from subnet 172.16.1.0/24 (in which all addresses begin 172.16.1) 
            and the .9 beside the host A icon."
            br {}
            "As another example, R1 uses address 172.16.1.1 on its LAN interface, 172.16.4.1 on one serial interface, and
            172.16.5.1 on an Ethernet WAN interface."
        }

        img {
            class: "mb-4 rounded-lg",
            alt: "Figure 16-3 IPv4 Network Used to Show Five-Step Routing Example",
            loading: "lazy",
            src: asset!("/assets/static/v1p5c16s1sh2f16-3.png", AssetOptions::image().with_avif()),
        }

        p { class: "mb-4",
            "Now on to the example, with host A (172.16.1.9) sending a packet to host B (172.16.2.9)."
        }

        Accordion { class: "mb-4",
            AccordionItem {
                AccordionTrigger { {h3_heading("Host Forwards the IP Packet to the Default Router (Gateway)")} }
                AccordionContent {
                    p { class: "mb-1",
                        "In this example, host A uses some application that sends data to host B (172.16.2.9)."
                        br {}
                        "After host A has the IP packet sitting in memory, host A's logic reduces to the following:"
                    }
                    ol { class: "list-disc list-inside mb-4",
                        li {
                            "My IP address/mask is 172.16.1.9/24, so my local subnet contains numbers 172.16.1.0-
                            172.16.1.255 (including the subnet ID and subnet broadcast address)."
                        }
                        li {
                            "The destination address is 172.16.2.9, which is clearly not in my local subnet."
                        }
                        li { "Send the packet to my default gateway, which is set to 172.16.1.1." }
                        li {
                            "To send the packet, encapsulate it in an Ethernet frame. Make the destination MAC
                            address be R1's G0/0 MAC address (host A's default gateway)."
                        }
                    }

                    p { class: "mb-4",
                        "Figure 16-4 pulls these concepts together, showing the destination IP address and destination MAC address 
                        in the frame and packet sent by host A in this case."
                        br {}
                        "Note that the figure uses a common drawing convention in networking, showing an Ethernet as a few lines, 
                        hiding all the detail of the Layer 2 switches."
                    }

                    img {
                        class: "mb-4 rounded-lg",
                        alt: "Figure 16-4 Host A Sends Packet to Host B",
                        loading: "lazy",
                        src: asset!("/assets/static/v1p5c16s1sh2f16-4.png", AssetOptions::image().with_avif()),
                    }
                }
            }
            AccordionItem {
                AccordionTrigger { {h3_heading("Routing Step 1: Decide Whether to Process the Incoming Frame")} }
                AccordionContent {
                    p { class: "mb-4",
                        "Routers receive many frames in an interface, particularly LAN interfaces."
                        br {}
                        "However, a router can and should ignore some of those frames."
                        br {}
                        "So, "
                        strong {
                            "the first step in the routing process begins with a decision of whether a router should process the 
                            frame or silently discard (ignore) the frame."
                        }
                    }

                    {h4_heading("FCS check")}
                    p { class: "mb-4",
                        "First, the router does a simple but important check (Step 1A in the process summary) so
                        that the router ignores all frames that had bit errors during transmission."
                        br {}
                        strong {
                            "The router uses the data-link trailer's FCS field to check the frame, and if errors occurred in 
                            transmission, the router discards the frame."
                        }
                        br {}
                        "(The router makes no attempt at error recovery; that is, the router
                        does not ask the sender to retransmit the data.)"
                    }

                    {h4_heading("Data-link addresss check")}
                    p { class: "mb-4",
                        "The router also checks the destination data-link address (Step 1B in the summary) to decide
                        whether the frame is intended for the router."
                        br {}
                        "For example, frames sent to the router's unicast MAC address for that interface are clearly sent to 
                        that router."
                        br {}
                        strong {
                            "However, a router can actually receive a frame sent to some other unicast MAC address, and routers should 
                            ignore these frames."
                        }
                    }

                    {h4_heading("Unkown unicast frames")}
                    p { class: "mb-4",
                        "For example, routers will receive some unicast frames sent to other devices in the VLAN just
                        because of how LAN switches work."
                        br {}
                        "Think back to how LAN switches forward unknown unicast frames—frames for which the switch does not list 
                        the destination MAC address in the MAC address table."
                        br {}
                        "The LAN switch floods those frames."
                        br {}
                        "The result? Routers sometimes receive frames destined for some other device, with some other device's MAC 
                        address listed as the destination MAC address."
                        br {}
                        "Routers should ignore those frames."
                    }

                    p { class: "mb-4",
                        "In this example, host A sends a frame destined for R1's MAC address."
                        br {}
                        "So, after the frame is received, and after R1 confirms with the FCS that no errors occurred, R1 confirms 
                        that the frame is destined for R1's MAC address (0200.0101.0101 in this case)."
                        br {}
                        "All checks have been passed, so R1 will process the frame, as shown in Figure 16-5."
                        br {}
                        "(Note that the large rectangle in the figure represents the internals of Router R1.)"
                    }

                    img {
                        class: "mb-4 rounded-lg",
                        alt: "Figure 16-5 Routing Step 1, on Router R1: Checking FCS and Destination MAC",
                        loading: "lazy",
                        src: asset!("/assets/static/v1p5c16s1sh2f16-5.png", AssetOptions::image().with_avif()),
                    }
                }
            }
            AccordionItem {
                AccordionTrigger { {h3_heading("Routing Step 2: De-encapsulation of the IP Packet")} }
                AccordionContent {
                    p { class: "mb-4",
                        "After the router knows that it ought to process the received frame (per Step 1), the next
                        step is relatively simple: de-encapsulating the packet."
                        br {}
                        strong {
                            "In router memory, the router no longer needs the original frame's data-link header and trailer, so the 
                            router removes and discards them"
                        }
                        ", leaving the IP packet, as shown in Figure 16-6."
                        br {}
                        "Note that the destination  IP address remains unchanged (172.16.2.9)."
                    }

                    img {
                        class: "mb-4 rounded-lg",
                        alt: "Figure 16-6 Routing Step 2 on Router R1: De-encapsulating the Packet",
                        loading: "lazy",
                        src: asset!("/assets/static/v1p5c16s1sh2f16-6.png", AssetOptions::image().with_avif()),
                    }
                }
            }
            AccordionItem {
                AccordionTrigger { {h3_heading("Routing Step 3: Choosing Where to Forward the Packet")} }
                AccordionContent {
                    p { class: "mb-4",
                        "While routing Step 2 required little thought, Step 3 requires the most thought of all the steps."
                        br {}
                        "At this point, the router needs to make a choice about where to forward the packet next."
                        br {}
                        strong {
                            "That process uses the router's IP routing table, with some matching logic to compare
                            the packet's destination address with the table."
                        }
                    }

                    {h4_heading("Routing table")}
                    p { class: "mb-4",
                        "First, an IP routing table lists multiple routes."
                        br {}
                        strong {
                            "Each individual route contains several facts, which in turn can be grouped as shown in Figure 16-7."
                        }
                        br {}
                        "Part of each route is used to match the destination address of the packet, while the rest of the route 
                        lists forwarding instructions: where to send the packet next."
                    }

                    KeyTopic {}
                    img {
                        class: "mb-4 rounded-lg",
                        alt: "Figure 16-7 Routing Step 3 on Router R1: Matching the Routing Table",
                        loading: "lazy",
                        src: asset!("/assets/static/v1p5c16s1sh2f16-7.png", AssetOptions::image().with_avif()),
                    }

                    p { class: "mb-4",
                        "Focus on the entire routing table for a moment, and notice the fact that it lists five routes."
                        br {}
                        "Earlier, Figure 16-3 showed the entire example network, with five subnets, so R1 has a route
                        for each of the five subnets."
                    }

                    {h4_heading("Matching instructions")}
                    p { class: "mb-4",
                        "Next, look at the part of the five routes that Router R1 will use to match packets."
                        br {}
                        "To fully define each subnet, each route lists both the subnet ID and the subnet mask."
                        br {}
                        strong {
                            "When matching the IP packet's destination with the routing table, the router looks at the packet's 
                            destination IP address (172.16.2.9) and compares it to the range of addresses defined by each subnet."
                        }
                        br {}
                        "Specifically, the router looks at the subnet and mask information; with a little math, the router
                        can figure out in which of these subnets 172.16.2.9 resides (the route for subnet 172.16.2.0/24)."
                    }

                    {h4_heading("Forwarding instructions")}
                    p { class: "mb-4",
                        "Finally, look to the right side of the figure, to the forwarding instructions for these five routes."
                        br {}
                        "After the router matches a specific route, the router uses the forwarding information
                        in the route to tell the router where to send the packet next."
                        br {}
                        "In this case, the router matched the route for subnet 172.16.2.0/24, so R1 will forward the packet out 
                        its own interface S0/0/0, to Router R2 next, listed with its next-hop router IP address of 172.16.4.2."
                    }

                    GreenNote {
                        p {
                            strong { "NOTE" }
                            " Routes for remote subnets typically list both an outgoing interface and next-hop
                            router IP address. 
                            Routes for subnets that connect directly to the router list only the outgoing interface because packets 
                            to these destinations do not need to be sent to another router"
                        }
                    }
                }
            }
            AccordionItem {
                AccordionTrigger { {h3_heading("Routing Step 4: Encapsulating the Packet in a New Frame")} }
                AccordionContent {
                    p { class: "mb-4",
                        "At this point, the router knows how it will forward the packet."
                        br {}
                        "However, routers cannot forward a packet without first wrapping a data-link header and trailer around it 
                        (encapsulation)."
                    }

                    {h4_heading("Serial WAN links")}
                    p { class: "mb-4",
                        "Encapsulating packets for serial links does not require a lot of thought, but the current
                        CCNA 200-301 exam does not require a lot from us."
                        br {}
                        strong {
                            "Point-to-point serial WAN links use either HDLC (the default) or PPP as the data-link protocol."
                        }
                        br {}
                        "However, "
                        strong {
                            "we can ignore any datalink logic, even ignoring data-link addressing, because serial links have 
                            only two devices on the link: the sender and the then-obvious receiver; the data-link addressing does not 
                            matter."
                        }
                        br {}
                        "In this example, R1 forwards the packet out S0/0/0, after encapsulating the packet inside an
                        HDLC frame, as shown in Figure 16-8."
                    }

                    img {
                        class: "mb-4 rounded-lg",
                        alt: "Figure 16-8 Routing Step 4 on Router R1: Encapsulating the Packet",
                        loading: "lazy",
                        src: asset!("/assets/static/v1p5c16s1sh2f16-8.png", AssetOptions::image().with_avif()),
                    }

                    {h4_heading("Ethernet WAN links")}
                    p { class: "mb-4",
                        "Note that with some other types of data links, the router has a little more work to do at this
                        routing step."
                        br {}
                        "For example, sometimes a router forwards packets out an Ethernet interface."
                        br {}
                        "To encapsulate the IP packet, the router would need to build an Ethernet header, and that
                        Ethernet header's destination MAC address would need to list the correct value."
                    }

                    p { class: "mb-4",
                        "For example, consider a packet sent by that same PC A (172.16.1.19) in Figure 16-3 but with
                        a destination of PC C (172.16.3.9)."
                        br {}
                        "When R1 processes the packet, R1 matches a route that tells R1 to forward the packet out R1's G0/1/0 
                        Ethernet interface to 172.16.5.3 (R3) next."
                        br {}
                        "R1 needs to put R3's MAC address in the header, and to do that, R1 uses its IP ARP table information, as 
                        shown in Figure 16-9."
                        br {}
                        "If R1 did not have an ARP table entry for 172.16.5.3, R1 would first have to use ARP to learn the matching 
                        MAC address and drop the packet."
                    }

                    img {
                        class: "mb-4 rounded-lg",
                        alt: "Figure 16-9 Routing Step 4 on Router R1 with a LAN Outgoing Interface",
                        loading: "lazy",
                        src: asset!("/assets/static/v1p5c16s1sh2f16-9.png", AssetOptions::image().with_avif()),
                    }
                }
            }
            AccordionItem {
                AccordionTrigger { {h3_heading("Routing Step 5: Transmitting the Frame")} }
                AccordionContent {
                    p {
                        "After the frame has been prepared, the router simply needs to transmit the frame."
                        br {}
                        "The router might have to wait, particularly if other frames are already waiting their turn to exit the 
                        interface."
                    }
                }
            }
        }

        {h3_heading("RECAP")}
        ol { class: "list-disc list-inside",
            li {
                "When a router receives a frame, it uses the data-link trailer's FCS field to check the frame, and if errors 
                occurred in transmission, the router discards the frame."
            }
            li {
                "The router makes no attempt at error recovery; that is, the router does not ask the sender to retransmit the data."
            }
            li {
                "Routes for remote subnets typically list both an outgoing interface and next-hop router IP address."
            }
            li {
                "Routes for subnets that connect directly to the router list only the outgoing interface because packets to these 
                destinations do not need to be sent to another router."
            }
            li {
                "Point-to-point serial WAN links use either HDLC (the default) or PPP as the data-link protocol."
            }
            li {
                "In Serial links, we can ignore any datalink logic, even ignoring data-link addressing, because serial links have 
                only two devices on the link: the sender and the then-obvious receiver; the data-link addressing does not matter."
            }
        }

    }
}