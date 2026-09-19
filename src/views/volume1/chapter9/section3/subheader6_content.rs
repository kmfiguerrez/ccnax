use dioxus::prelude::*;

use crate::{
    components::{my_accordion::{Accordion, AccordionItem, AccordionTrigger, AccordionContent}},
    utils::{h3_heading, h4_heading}
};

#[component]
pub fn Content() -> Element {
    rsx! {
        p { class: "mb-4",
            "To close out the chapter, the last few topics introduce a few optional features that make STP
            work even better or be more secure: EtherChannel, PortFast, and BPDU Guard."
        }

        Accordion { class: "mb-4",
            AccordionItem {
                AccordionTrigger { {h3_heading("EtherChannel")} }
                AccordionContent {
                    p { class: "mb-4",
                        "One of the best ways to lower STP's convergence time is to avoid convergence altogether."
                        br {}
                        strong {
                            "EtherChannel provides a way to prevent STP convergence from being needed when only a
                            single port or cable failure occurs."
                        }
                    }

                    {h4_heading("EtherChannel Combines multiple links")}
                    p { class: "mb-4",
                        strong {
                            "EtherChannel combines multiple parallel segments of equal speed (up to eight) between the
                        same pair of switches, bundled into an EtherChannel."
                        }
                        br {}
                        "The switches treat the EtherChannel as a single interface with regard to STP."
                        br {}
                        "As a result, if one of the links fails, but at least one of the links is up, 
                        STP convergence does not have to occur."
                        br {}
                        "For example, Figure 9-12 shows the familiar three-switch network, but now with two Gigabit Ethernet 
                        connections between each pair of switches."
                    }

                    p { class: "mb-4",
                        "With each pair of Ethernet links configured as an EtherChannel, STP treats each
                        EtherChannel as a single link."
                        br {}
                        "In other words, both links to the same switch must fail for a switch to need to cause STP convergence."
                        br {}
                        strong {
                            "Without EtherChannel, if you have multiple parallel links between two switches, STP blocks all the 
                            links except one."
                        }
                        br {}
                        "With EtherChannel, all the parallel links can be up and working at the same time, while reducing the 
                        number of times STP must converge, which in turn makes the network more available."
                    }

                    img {
                        class: "mb-4 rounded-lg",
                        alt: "Figure 9-11 RSTP Link Types",
                        loading: "lazy",
                        src: asset!("/assets/static/v1p3c9s3sh6f9-12.png", AssetOptions::image().with_avif()),
                    }

                    {h4_heading("Layer 2 vs Layer 3 EtherChannels")}
                    p { class: "mb-4",
                        strong {
                            "The current CCNA exam blueprint includes a topic for the configuration of both Layer 2
                            EtherChannels (as described here) as well as Layer 3 EtherChannels."
                        }
                        br {}
                        "Chapter 10, “RSTP and EtherChannel Configuration,” shows how to configure Layer 2 EtherChannels, while 
                        Chapter 17, “IP Routing in the LAN,” shows how to configure Layer 3 EtherChannels."
                        br {}
                        "Note that Layer 2 EtherChannels combine links that switches use as switch ports, with the switches
                        using Layer 2 switching logic to forward and receive Ethernet frames over the EtherChannels."
                        br {}
                        "Layer 3 EtherChannels also combine links, but the switches use Layer 3 routing logic to forward 
                        packets over the EtherChannels."
                    }
                }
            }
            AccordionItem {
                AccordionTrigger { {h3_heading("PortFast")} }
                AccordionContent {
                    p { class: "mb-4",
                        "PortFast allows a switch to immediately transition from blocking to forwarding, bypassing listening and 
                        learning states."
                        br {}
                        strong {
                            "However, the only ports on which you can safely enable PortFast are ports on which you know that no 
                            bridges, switches, or other STP-speaking devices are connected."
                        }
                        br {}
                        "Otherwise, using PortFast risks creating loops, the very thing that the listening and learning states 
                        are intended to avoid."
                    }

                    p { class: "mb-4",
                        "PortFast is most appropriate for connections to end-user devices."
                        br {}
                        "If you turn on PortFast on ports connected to end-user devices, when an end-user PC boots, the switch 
                        port can move to an STP forwarding state and forward traffic as soon as the PC NIC is active."
                        br {}
                        strong {
                            "Without PortFast, each port must wait while the switch confirms that the port is a DP."
                        }
                        br {}
                        "With STP in particular (and not RSTP), the switch waits in the temporary listening and learning states
                        before settling into the forwarding state."
                    }

                    {h4_heading("RSTP includes PortFast")}
                    p { class: "mb-4",
                        "As you might guess from the fact that PortFast speeds convergence, RSTP includes PortFast."
                        br {}
                        "You might recall the mention of RSTP port types, particularly point-to-point edge port
                        types, around Figure 9-11."
                        br {}
                        "RSTP, by design of the protocol, converges quickly on these point-to-point edge type ports by 
                        bypassing the learning state, which is the same idea Cisco originally introduced with PortFast."
                        br {}
                        "In practice, Cisco switches enable RSTP point-to-point edge ports by enabling PortFast on the port."
                    }
                }
            }
            AccordionItem {
                AccordionTrigger { {h3_heading("BPDU Guard")} }
                AccordionContent {
                    p {
                        "STP and RSTP open up the LAN to several different types of possible security exposures."
                        br {}
                        "For example:"
                    }
                    ul { class: "list-disc list-inside mb-4",
                        li {
                            "An attacker could connect a switch to one of these ports, one with a low STP/RSTP priority value, 
                            and become the root switch. The new STP/RSTP topology could have worse
                            performance than the desired topology."
                        }
                        li {
                            "The attacker could plug into multiple ports, into multiple switches, become root, and
                            actually forward much of the traffic in the LAN. Without the networking staff realizing
                            it, the attacker could use a LAN analyzer to copy large numbers of data frames sent
                            through the LAN."
                        }
                        li {
                            "Users could innocently harm the LAN when they buy and connect an inexpensive
                            consumer LAN switch (one that does not use STP/RSTP). Such a switch, without any
                            STP/RSTP function, would not choose to block any ports and could cause a loop."
                        }
                    }

                    {h4_heading("Cisco BPDU Guard must be enabled on access ports only")}
                    p { class: "mb-4",
                        "The "
                        i { "Cisco BPDU Guard" }
                        " feature helps defeat these kinds of problems by disabling a port
                        if any BPDUs are received on the port."
                        br {}
                        strong {
                            "So, this feature is particularly useful on ports that should be used only as an access port and 
                            never connected to another switch."
                        }
                    }

                    p { class: "mb-4",
                        strong { "In addition, the BPDU Guard feature helps prevent problems with PortFast." }
                        br {}
                        "PortFast should be enabled only on access ports that connect to user devices, not to other LAN switches."
                        br {}
                        "Using BPDU Guard on these same ports makes sense because if another switch connects to
                        such a port, the local switch can disable the port before a loop is created."
                    }
                }
            }
        }

        {h3_heading("RECAP")}
        ol { class: "list-disc list-inside",
            li {
                "One of the best ways to lower STP's convergence time is to avoid convergence altogether."
            }
            li {
                "EtherChannel provides a way to prevent STP convergence from being needed when only a single port or cable 
                failure occurs."
            }
            li {
                "EtherChannel combines multiple parallel segments of equal speed (up to eight) between the same pair of switches, 
                bundled into an EtherChannel."
            }
            li {
                "Without EtherChannel, if you have multiple parallel links between two switches, STP blocks all the links except one."
            }
            li {
                "Layer 2 EtherChannels combine links that switches use as switch ports, with the switches using Layer 2 switching 
                logic to forward and receive Ethernet frames over the EtherChannels."
            }
            li {
                "Layer 3 EtherChannels also combine links, but the switches (Multilayer switch) use Layer 3 routing logic to 
                forward packets over the EtherChannels."
            }
            li {
                "PortFast allows a switch to immediately transition from blocking to forwarding, bypassing listening and learning 
                states."
                " However, the only ports on which you can safely enable PortFast are ports on which you know that no bridges, 
                switches, or other STP-speaking devices are connected."
            }
            li {
                "RSTP includes PortFast. In practice, Cisco switches enable RSTP point-to-point edge ports by enabling PortFast on the port."
            }
            li {
                "Without PortFast, each port must wait while the switch confirms that the port is a DP."
            }
            li {
                "The "
                i { "Cisco BPDU Guard" }
                " feature helps defeat the kinds of security exposures by disabling a port
                if any BPDUs are received on the port."
            }
            li {
                "In addition, the BPDU Guard feature helps prevent problems with PortFast (creating loops)."
            }
            li {
                "Using BPDU Guard on ports with PortFast makes sense because if another switch connects to such a port, the local 
                switch can disable the port before a loop is created."
            }
        }
    }
}