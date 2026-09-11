use dioxus::prelude::*;

use crate::{
    components::my_accordion::{Accordion, AccordionContent, AccordionItem, AccordionTrigger},
    utils::{h3_heading, h4_heading}
};

#[component]
pub fn Content() -> Element {
    rsx! {
        p { class: "mb-4",
            "If you create a campus LAN that contains many VLANs, you typically still need all devices
            to be able to send data to all other devices."
            br {}
            "This next topic discusses some concepts about how to route data between those VLANs."
        }

        Accordion { class: "mb-4",
            AccordionItem {
                AccordionTrigger { {h3_heading("The Need for Routing Between VLANs")} }
                AccordionContent {
                    {h4_heading("Layer 2 protocol concepts")}
                    p { class: "mb-4",
                        "LAN switches that forward data based on Layer 2 logic, as discussed so far in this book,
                        often go by the name Layer 2 switch."
                        br {}
                        "For example, Chapter 5, “Analyzing Ethernet LAN Switching,” discussed how LAN switches receive 
                        Ethernet frames (a Layer 2 concept), look at the destination Ethernet MAC address (a Layer 2 address), 
                        and forward the Ethernet frame out some other interface."
                        br {}
                        "All those concepts are defined by Layer 2 protocols, hence the name Layer 2 switch."
                    }

                    {h4_heading("Layer 2 switch does not forward data between VLANs")}
                    p { class: "mb-4",
                        "Layer 2 switches perform their logic per VLAN."
                        br {}
                        "For example, in Figure 8-7, the two PCs
                        on the left sit in VLAN 10, in subnet 10."
                        br {}
                        "The two PCs on the right sit in a different VLAN (20), with a different subnet (20)."
                        br {}
                        "Note that the figure repeats earlier Figure 8-2, but with the switch broken into halves, 
                        to emphasize the point that Layer 2 switches will not forward data between two VLANs."
                    }

                    p { class: "mb-4",
                        "As shown in the figure, when configured with some ports in VLAN 10 and others in VLAN
                        20, the switch acts like two separate switches in which it will forward traffic."
                        br {}
                        "In fact, one goal of VLANs is to separate traffic in one VLAN from another, preventing frames in one
                        VLAN from leaking over to other VLANs."
                        br {}
                        "For example, when Dino (in VLAN 10) sends any Ethernet frame, if SW1 is a Layer 2 switch, that switch 
                        will not forward the frame to the PCs on the right in VLAN 20."
                    }

                    img {
                        class: "mb-4 rounded-lg",
                        alt: "Figure 8-7 Layer 2 Switch Does Not Route Between the VLANs",
                        loading: "lazy",
                        src: asset!("/assets/static/v1p3c8s1sh2f8-7.png", AssetOptions::image().with_avif()),
                    }
                }
            }
            AccordionItem {
                AccordionTrigger { {h3_heading("Routing Packets Between VLANs with a Router")} }
                AccordionContent {
                    p { class: "mb-4",
                        "When including VLANs in a campus LAN design, the devices in a VLAN need to be in the
                        same subnet."
                        br {}
                        "Following the same design logic, devices in different VLANs need to be in different subnets."
                    }

                    {h4_heading("Multilayer switch")}
                    p { class: "mb-4",
                        "To forward packets between VLANs, the network must use a device that acts as a router."
                        br {}
                        "You can use an actual router, as well as some other switches that can perform some functions like a
                        router."
                        br {}
                        "These switches that also perform Layer 3 routing functions go by the name multilayer
                        switch or Layer 3 switch."
                        br {}
                        "This section first discusses how to forward data between VLANs
                        when using Layer 2 switches and ends with a brief discussion of how to use Layer 3 switches."
                    }

                    {h4_heading("Using Router interfaces to forward VLANs")}
                    p { class: "mb-4",
                        "For example, Figure 8-8 shows a router that can route packets between subnets 10 and 20."
                        br {}
                        "The figure shows the same Layer 2 switch as shown in Figure 8-7, with the same perspective
                        of the switch being split into parts with two different VLANs, and with the same PCs in the
                        same VLANs and subnets."
                        br {}
                        "Now Router R1 has one LAN physical interface connected to the switch and assigned to VLAN 10, 
                        and a second physical interface connected to the switch and assigned to VLAN 20."
                        br {}
                        "With an interface connected to each subnet, the Layer 2 switch can keep doing its job—forwarding frames 
                        inside a VLAN, while the router can do its job—routing IP packets between the subnets."
                    }

                    img {
                        class: "mb-4 rounded-lg",
                        alt: "Figure 8-8 Routing Between Two VLANs on Two Physical Interfaces",
                        loading: "lazy",
                        src: asset!("/assets/static/v1p3c8s1sh2f8-8.png", AssetOptions::image().with_avif()),
                    }

                    p { class: "mb-4",
                        "The figure shows an IP packet being routed from Fred, which sits in one VLAN/subnet, to
                        Betty, which sits in the other."
                        br {}
                        "The Layer 2 switch forwards two different Layer 2 Ethernet frames: one in VLAN 10, from Fred to R1's 
                        F0/0 interface, and the other in VLAN 20, from R1's F0/1 interface to Betty."
                        br {}
                        strong {
                            "From a Layer 3 perspective, Fred sends the IP packet to its default router (R1), and R1 routes the 
                            packet out another interface (F0/1) into another subnet where Betty resides."
                        }
                    }

                    {h4_heading("Other solutions")}
                    p { class: "mb-4",
                        "The design in Figure 8-8 works, but there are several different solutions for routing packets
                        between VLANs."
                        br {}
                        "This chapter shows the option of using a separate physical router, with a separate link per VLAN, 
                        because it can be the easiest of the options to understand and visualize."
                        br {}
                        "Chapter 17, “IP Routing in the LAN,” works through those other features for routing
                        packets between VLANs."
                    }
                }
            }
        }

        {h3_heading("RECAP")}
        ol { class: "list-disc list-inside",
            li { "Layer 2 switches perform their logic per VLAN." }
            li {
                "When including VLANs in a campus LAN design, the devices in a VLAN need to be in the same subnet, 
                and devices in different VLANs need to be in different subnets."
            }
            li {
                "To forward packets between VLANs, the network must use a device that acts as a router."
            }
            li {
                "Switches that also perform Layer 3 routing functions go by the name multilayer switch or Layer 3 switch."
            }
        }
    }
}