use dioxus::prelude::*;

use crate::{
    components::{
        KeyTopic, my_accordion::{Accordion, AccordionContent, AccordionItem, AccordionTrigger}
    }, utils::{h3_heading, h4_heading, TextCommandColor, text_command}
};

#[component]
pub fn Content() -> Element {
    rsx! {
        p { class: "mb-4",
            strong {
                "HSRP operates with an active/standby model (also more generally called active/passive)."
            }
            br {}
            "HSRP allows two (or more) routers to cooperate, all being willing to act as the default router."
            br {}
            "However, at any one time, only one router actively supports the end-user traffic."
            br {}
            "The packets sent by hosts to their default router flow to that one active router."
            br {}
            "Then the other routers, with an HSRP standby state, sit there patiently waiting to take over should the
            active HSRP router have a problem."
        }

        {h3_heading("Virtual IP and MAC addresses")}
        p { class: "mb-4",
            strong {
                "The HSRP active router implements a virtual IP address and matching virtual MAC address."
            }
            br {}
            "This virtual IP address exists as part of the HSRP configuration, which is an additional configuration item compared 
            to the usual "
            {text_command("ip address interface", TextCommandColor::Gold)}
            " subcommand."
            br {}
            "This virtual IP address is in the same subnet as the interface IP address, but it is a different IP address."
            br {}
            strong { "The router then automatically creates the virtual MAC address." }
            br {}
            "All the cooperating HSRP routers know these virtual addresses, but only the HSRP active router uses these addresses at
            any one point in time."
        }

        {h3_heading("Virtual IP address as the default gateway")}
        p { class: "mb-4",
            strong {
                "Hosts refer to the virtual IP address as their default router address, instead of any one
                router's interface IP address."
            }
            br {}
            "For instance, in Figure 12-5, R1 and R2 use HSRP."
            br {}
            "The HSRP virtual IP address is 10.1.1.1, with the virtual MAC address referenced as VMAC1 for simplicity's sake."
        }

        KeyTopic {}
        img {
            class: "mb-4 rounded-lg",
            loading: "lazy",
            alt: "Figure 12-5 All Traffic Goes to .1 (R1, Which Is Active); R2 Is Standby",
            src: asset!("/assets/static/v2p3c12s1sh4f12-5.png", AssetOptions::image().with_avif()),
        }

        Accordion { class: "mb-4",
            AccordionItem {
                AccordionTrigger { {h3_heading("HSRP Failover")} }
                AccordionContent {
                    {h4_heading("HSRP Messages")}
                    p { class: "mb-4",
                        "HSRP on each router has some work to do to make the network function as shown in Figure 12-5."
                        br {}
                        strong {
                            "The two routers need HSRP configuration, including the virtual IP address."
                        }
                        br {}
                        "The two routers send HSRP messages to each other to negotiate and decide which router should
                        currently be active and which should be standby."
                        br {}
                        "Then the two routers continue to send messages to each other so that the standby router knows when the 
                        active router fails so that it can take over as the new active router."
                    }

                    p { class: "mb-4",
                        "Figure 12-6 shows the result when R1, the HSRP active router in Figure 12-5, fails."
                        br {}
                        "R1 quits using the virtual IP and MAC address, while R2, the new active router, starts using these
                        addresses."
                        br {}
                        "The hosts do not need to change their default router settings at all, with traffic
                        now flowing to R2 instead of R1."
                    }

                    KeyTopic {}
                    img {
                        class: "mb-4 rounded-lg",
                        loading: "lazy",
                        alt: "Figure 12-6 Packets Sent Through R2 (New Active) Once It Takes Over for Failed R1",
                        src: asset!("/assets/static/v2p3c12s1sh4f12-6.png", AssetOptions::image().with_avif()),
                    }

                    {h4_heading("Hosts do not react to outages")}
                    p { class: "mb-4",
                        "When the failover happens, some changes do happen, but none of those changes happen
                        on the hosts."
                        br {}
                        "The host keeps the same default router setting, set to the virtual IP address
                        (10.1.1.1 in this case)."
                        br {}
                        "The host's ARP table does not have to change either, with the HSRP
                        virtual MAC being listed as the MAC address of the virtual router."
                    }

                    {h4_heading("Routers and Switches react to outages")}
                    p { class: "mb-4",
                        "When the failover occurs, changes happen on both the routers and the LAN switches."
                        br {}
                        "Clearly, the new active router has to be ready to receive packets (encapsulated inside
                        frames) using the virtual IP and MAC addresses."
                        br {}
                        "However, the LAN switches, hidden in the last few figures, formerly sent frames destined for VMAC1 to 
                        router R1."
                        br {}
                        "Now the switches must know to send the frames to the new active router, R2."
                    }

                    {h4_heading("Gratuitous ARP")}
                    p { class: "mb-4",
                        "To make the switches change their MAC address table entries for VMAC1, R2 sends an
                        Ethernet frame with VMAC1 as the source MAC address."
                        br {}
                        "The switches, as normal, learn the source MAC address (VMAC1), but with new ports that point toward R2."
                        br {}
                        "The frame is also a LAN broadcast, so all the switches learn a MAC table entry for VMAC1 that leads toward R2."
                        br {}
                        "("
                        strong {
                            "By the way, this Ethernet frame holds an ARP Reply message, called a gratuitous ARP,
                            because the router sends it without first receiving an ARP Request"
                        }
                        ".)"
                    }
                }
            }
            AccordionItem {
                AccordionTrigger { {h3_heading("HSRP Load Balancing")} }
                AccordionContent {
                    p { class: "mb-4",
                        strong {
                            "The active/standby model of HSRP means that in one subnet all hosts send their off-subnet
                            packets through only one router."
                        }
                        br {}
                        "In other words, the routers do not share the workload, with one router handling all the packets."
                        br {}
                        "For instance, back in Figure 12-5, R1 was the active router, so all hosts in the subnet sent their 
                        packets through R1, and none of the hosts in the subnet sent their packets through R2."
                    }

                    {h4_heading("VLANs with their own default router")}
                    p { class: "mb-4",
                        strong {
                            "HSRP does support load balancing by preferring different routers to be the active router in different 
                            subnets."
                        }
                        br {}
                        "Most sites that require a second router for redundancy are also big enough to use several VLANs and subnets 
                        at the site."
                        br {}
                        strong {
                            "The two routers will likely connect to all the VLANs, acting as the default router in each VLAN."
                        }
                        br {}
                        "HSRP then can be configured to prefer one router as active in one VLAN and another router as active in 
                        another VLAN, balancing the traffic."
                        br {}
                        "Or "
                        strong {
                            "you can configure multiple instances of HSRP in the same subnet (called multiple HSRP groups)"
                        }
                        ", preferring one router to be active in one group and the other router to be preferred as active in another."
                    }

                    {h4_heading("Load balancing using ROAS")}
                    p { class: "mb-4",
                        "For instance, Figure 12-7 shows a redesigned LAN, now with two hosts in VLAN 1 and two hosts in VLAN 2."
                        br {}
                        "Both R1 and R2 connect to the LAN, and both use a VLAN trunking and router-on-a-stick (ROAS) configuration."
                        br {}
                        "Both routers use HSRP in each of the two subnets, supporting each other."
                        br {}
                        "However, on purpose, R1 has been configured so that it wins the negotiation to become HSRP active in 
                        VLAN 1, and R2 has been configured to win in VLAN 2."
                    }

                    img {
                        class: "mb-4 rounded-lg",
                        loading: "lazy",
                        alt: "Figure 12-7 Load Balancing with HSRP by Using Different Active Routers per Subnet",
                        src: asset!("/assets/static/v2p3c12s1sh4f12-7.png", AssetOptions::image().with_avif()),
                    }

                    p { class: "mb-4",
                        "Note that by having each router act as the HSRP active router in some subnets, the design
                        makes use of both routers and both WAN links."
                    }

                    {h4_heading("Where to configure FHRPs")}
                    p { class: "mb-4",
                        "FHRPs are needed on any device that acts as a default router, which of course includes both
                        traditional routers and Layer 3 switches."
                        br {}
                        strong {
                            "HSRP can be configured on routers and Layer 3 switches on interfaces that have IP addresses configured."
                        }
                        br {}
                        "However, in most cases, HSRP is used on interfaces to subnets that have hosts that need to use a default 
                        router."
                        br {}
                        "Those interfaces include router physical interfaces, router trunk subinterfaces, and Layer 3 switched 
                        virtual interfaces (SVI)."
                    }
                }
            }
        }

        {h3_heading("RECAP")}
        ol { class: "list-disc list-inside",
            li {
                "HSRP operates with an active/standby model (also more generally called active/passive)."
            }
            li {
                "HSRP allows two (or more) routers to cooperate, all being willing to act as the default router."
                " However, at any one time, only one router actively supports the end-user traffic."
            }
            li {
                "The packets sent by hosts to their default router flow to that one active router."
                " Then the other routers, with an HSRP standby state, sit there patiently waiting to take over should the
                active HSRP router have a problem."
            }
            li {
                "The HSRP active router implements a virtual IP address and matching virtual MAC address."
                " This virtual IP address exists as part of the HSRP configuration."
            }
            li {
                "This virtual IP address is in the same subnet as the interface IP address, but it is a different IP address."
                " The routers then each automatically creates their own virtual MAC address."
            }
            li {
                "Hosts refer to the virtual IP address as their default router address, instead of any one router's interface IP address."
            }
            li {
                "FHRP's HSRP protocol acts like STP, both use to create redundancy, STP for redundant switches and 
                HSRP for redundant default routers."
            }
            li {
                "Gratuitous ARP is a layer 2 broadcast ARP Reply message sent by a device without first receiving an ARP Request."
            }
            li {
                "Just like any type of RSTP/STP, FHRP family of protocols can do load balancing by preferring different routers to be the 
                active router in different subnets/VLANs."
            }
            li {
                "You can also configure multiple instances of HSRP in the same subnet (called multiple HSRP groups), preferring one 
                router to be active in one group and the other router to be preferred as active in another."
            }
        }
    }
}