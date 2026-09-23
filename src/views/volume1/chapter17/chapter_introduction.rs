use dioxus::prelude::*;

use crate::utils::h3_heading;

#[component]
pub fn ChapterIntroductionContent() -> Element {
    rsx! {
        p { class: "mb-4",
            "The preceding two chapters showed how to configure an IP address and mask on a router
            interface, making the router ready to route packets to/from the subnet implied by that
            address/mask combination."
            br {}
            "While true and useful, all the examples so far ignored the LAN switches and the possibility of VLANs."
            br {}
            "In fact, the examples so far show the simplest possible cases: the attached switches as Layer 2 switches, using 
            only one VLAN, with the router configured with one ip address command on its physical interface."
            br {}
            strong {
                "This chapter takes a detailed look at how to configure routers so that they route packets to/from the subnets that
                exist on each and every VLAN."
            }
        }

        {h3_heading("VLAN is not equivalent to a subnet")}
        p { class: "mb-4",
            "Because Layer 2 switches do not forward Layer 2 frames between VLANs, a network must
            use routers to route IP packets between subnets to allow those devices in different VLANs/
            subnets to communicate."
            br {}
            strong {
                "To review, Ethernet defines the concept of a VLAN, while IP defines the concept of an IP subnet, so a VLAN is 
                not equivalent to a subnet."
            }
            br {}
            "However, the set of devices in one VLAN are typically also in one subnet."
            br {}
            "By the same reasoning, devices in two different VLANs are normally in two different subnets."
            br {}
            "For two devices in different VLANs to communicate with each other, routers must connect to the subnets that exist on
            each VLAN, and then the routers forward IP packets between the devices in those subnets."
        }

        p { class: "mb-1",
            "This chapter discusses the configuration and verification steps related to three methods of
            routing between VLANs with three major sections:"
        }
        ol { class: "list-disc list-inside mb-4",
            li {
                span { class: "font-bold", "VLAN Routing with Router 802.1Q Trunks:" }
                " The first section discusses how to configure a router to use VLAN trunking as connected to a Layer 2 switch."
                " The router does the routing, with the switch creating the VLANs."
                " The link between the router and switch use trunking so that the router has an interface connected to each 
                VLAN/subnet."
                strong {
                    " This feature is known as routing over a VLAN trunk and also known as router-on-a-stick (ROAS)."
                }
            }
            li {
                span { class: "font-bold", "VLAN Routing with Layer 3 Switch SVIs:" }
                " The second section discusses using a LAN switch that supports both Layer 2 switching and Layer 3 routing 
                (called a Layer 3 switch or multilayer switch)."
                br {}
                " To route, the Layer 3 switch configuration uses interfaces called switched virtual interfaces (SVI), 
                which are also called VLAN interfaces."
            }
            li {
                span { class: "font-bold", "VLAN Routing with Layer 3 Switch Routed Ports:" }
                " The third major section of the chapter discusses an alternative to SVIs called routed ports, in which the 
                physical switch ports are made to act like interfaces on a router."
                br {}
                " This third section also introduces the concept of an EtherChannel as used as a routed port in a feature 
                called Layer 3 EtherChannel."
            }
        }

        {h3_heading("RECAP")}
        ol { class: "list-disc list-inside",
            li {
                "Layer 2 switches do not forward Layer 2 frames between VLANs, a network must use routers to route IP packets 
                between subnets to allow those devices in different VLANs/ subnets to communicate."
            }
            li {
                "To review, Ethernet defines the concept of a VLAN, while IP defines the concept of an IP subnet, so a VLAN is 
                not equivalent to a subnet."
            }
            li {
                "The set of devices in one VLAN are typically also in one subnet. By the same reasoning, devices in two different 
                VLANs are normally in two different subnets."
            }
            li {
                "For two devices in different VLANs to communicate with each other, routers must connect to the subnets that exist 
                on each VLAN, and then the routers forward IP packets between the devices in those subnets."
            }
            li { "Routers routing over a trunk link is known as router-on-a-stick (ROAS)." }
            li { "Switched virtual interfaces (SVI) are also called VLAN interfaces." }
        }
    }
}