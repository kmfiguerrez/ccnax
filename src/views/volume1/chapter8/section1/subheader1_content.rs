use dioxus::prelude::*;

use crate::{
    components::{
        KeyTopic, my_accordion::{Accordion, AccordionContent, AccordionItem, AccordionTrigger}
    }, utils::{h3_heading, h4_heading}
};

#[component]
pub fn Content() -> Element {
    rsx! {
        p { class: "mb-4",
            "Configuring VLANs on a single switch requires only a little effort: you simply configure
            each port to tell it the VLAN number to which the port belongs."
            br {}
            "With multiple switches, you have to consider additional concepts about how to forward traffic between the switches."
        }

        {h3_heading("VLAN Trunking")}
        p { class: "mb-4",
            "When you are using VLANs in networks that have multiple interconnected switches, the
            switches need to use VLAN trunking on the links between the switches."
            br {}
            "VLAN trunking causes the switches to use a process called VLAN tagging, by which the sending switch
            adds another header (it's an insertion rather than encapsulation) to the frame before sending it over the trunk."
            br {}
            "This extra trunking header includes a VLAN identifier (VLAN ID) field so that the sending switch can associate the
            frame with a particular VLAN ID, and the receiving switch can then know in what VLAN
            each frame belongs."
        }

        {h3_heading("Multiswitch VLAN Without VLAN Trunking")}
        p { class: "mb-4",
            "Figure 8-3 shows an example that demonstrates VLANs that exist on multiple switches, but
            it does not use trunking."
            br {}
            "First, the design uses two VLANs: VLAN 10 and VLAN 20."
            br {}
            "Each switch has two ports assigned to each VLAN, so each VLAN exists in both switches."
            br {}
            "To forward traffic in VLAN 10 between the two switches, the design includes a link between
            switches, with that link fully inside VLAN 10."
            br {}
            "Likewise, to support VLAN 20 traffic between switches, the design uses a second link between switches, with that 
            link inside VLAN 20."
        }

        p { class: "mb-4",
            "The design in Figure 8-3 functions perfectly."
            br {}
            "For example, PC11 (in VLAN 10) can send a
            frame to PC14."
            br {}
            "The frame flows into SW1, over the top link (the one that is in VLAN 10) and over to SW2."
        }

        img {
            class: "mb-4 rounded-lg",
            alt: "Figure 8-3 Multiswitch VLAN Without VLAN Trunking",
            loading: "lazy",
            src: asset!("/assets/static/v1p3c8s1sh1f8-3.png", AssetOptions::image().with_avif()),
        }

        p { class: "mb-4",
            "The design shown in Figure 8-3 works, but it simply does not scale very well."
            br {}
            "It requires one physical link between switches to support every VLAN."
            br {}
            "If a design needed 10 or 20 VLANs, you would need 10 or 20 links between switches, and you would use 10 or 20 switch 
            ports (on each switch) for those links."
        }

        Accordion { class: "mb-4",
            AccordionItem {
                AccordionTrigger { {h3_heading("VLAN Tagging Concepts")} }
                AccordionContent {
                    p { class: "mb-4",
                        "VLAN trunking creates one link between switches that supports as many VLANs as you need."
                        br {}
                        "As a VLAN trunk, the switches treat the link as if it were a part of all the VLANs."
                        br {}
                        "At the same time, the trunk keeps the VLAN traffic separate, so frames in VLAN 10 would not
                        go to devices in VLAN 20, and vice versa, because each frame is identified by VLAN number as it crosses 
                        the trunk."
                        br {}
                        "Figure 8-4 shows the idea, with a single physical link between the two switches."
                    }

                    img {
                        class: "mb-4 rounded-lg",
                        alt: "Figure 8-4 Multiswitch VLAN with Trunking",
                        loading: "lazy",
                        src: asset!("/assets/static/v1p3c8s1sh1f8-4.png", AssetOptions::image().with_avif()),
                    }

                    p { class: "mb-4",
                        "The use of trunking allows switches to forward frames from multiple VLANs over a single
                        physical connection by adding a small header to the Ethernet frame."
                        br {}
                        "For example, Figure 8-5 shows PC11 sending a broadcast frame on interface Fa0/1 at Step 1."
                        br {}
                        "To flood the frame, switch SW1 needs to forward the broadcast frame to switch SW2."
                        br {}
                        "However, SW1 needs to let SW2 know that the frame is part of VLAN 10, so that after the frame is 
                        received, SW2 will flood the frame only into VLAN 10, and not into VLAN 20."
                        br {}
                        "So, as shown at Step 2, before sending the frame, SW1 adds a VLAN header to the original Ethernet frame, 
                        with the VLAN header listing a VLAN ID of 10 in this case"
                    }

                    KeyTopic {}
                    img {
                        class: "mb-4 rounded-lg",
                        alt: "Figure 8-5 VLAN Trunking Between Two Switches",
                        loading: "lazy",
                        src: asset!("/assets/static/v1p3c8s1sh1f8-5.png", AssetOptions::image().with_avif()),
                    }

                    p { class: "mb-4",
                        "When SW2 receives the frame, it understands that the frame is in VLAN 10."
                        br {}
                        "SW2 then removes the VLAN header, forwarding the original frame out its interfaces in VLAN 10 (Step 3)."
                    }

                    p { class: "mb-4",
                        "For another example, consider the case when PC21 (in VLAN 20) sends a broadcast."
                        br {}
                        "SW1 sends the broadcast out port Fa0/4 (because that port is in VLAN 20) and out Gi0/1
                        (because it is a trunk, meaning that it supports multiple different VLANs)."
                        br {}
                        "SW1 adds a trunking header to the frame, listing a VLAN ID of 20."
                        br {}
                        "SW2 strips off the trunking header after determining that the frame is part of VLAN 20, 
                        so SW2 knows to forward the frame out only ports Fa0/3 and Fa0/4, because they are in VLAN 20, and 
                        not out ports Fa0/1 and Fa0/2, because they are in VLAN 10."
                    }
                
                }
            }
            AccordionItem {
                AccordionTrigger { {h3_heading("The 802.1Q and ISL VLAN Trunking Protocols")} }
                AccordionContent {
                    p { class: "mb-4",
                        "Cisco has supported two different trunking protocols over the years: Inter-Switch Link (ISL)
                        and IEEE 802.1Q."
                        br {}
                        "Cisco created the ISL years before 802.1Q, in part because the IEEE had not yet defined a 
                        VLAN trunking standard."
                        br {}
                        "Today, 802.1Q has become the more popular trunking protocol, with Cisco not even bothering to support 
                        ISL in many of its switch models today. "
                    }

                    {h4_heading("802.1Q header size")}
                    p { class: "mb-4",
                        "While both ISL and 802.1Q tag each frame with the VLAN ID, the details differ."
                        br {}
                        "802.1Q inserts an extra 4-byte 802.1Q VLAN header into the original frame's Ethernet header, as
                        shown at the top of Figure 8-6."
                        br {}
                        "As for the fields in the 802.1Q header, only the 12-bit VLAN
                        ID field inside the 802.1Q header matters for topics discussed in this book."
                        br {}
                        "This 12-bit field supports a theoretical maximum of 212 (4096) VLANs, but in practice it supports a 
                        maximum of 4094."
                        br {}
                        "(Both 802.1Q and ISL use 12 bits to tag the VLAN ID, with two reserved values [0 and 4095].)"
                    }

                    {h4_heading("Cisco two range of VLAN IDs")}
                    p { "Cisco switches break the range of VLAN IDs (1-4094) into two ranges:" }
                    ul { class: "list-disc list-inside",
                        li { "the normal range" }
                        li { "and the extended range." }
                    }
                    p { class: "mb-4",
                        "All switches can use normal-range VLANs with values from 1 to 1005."
                        br {}
                        "Only some switches can use extended-range VLANs with VLAN IDs from 1006 to 4094."
                        br {}
                        "The rules for which switches can use extended-range VLANs depend on the configuration
                        of the VLAN Trunking Protocol (VTP), which is discussed briefly in the section “VLAN
                        Trunking Configuration,” later in this chapter."
                    }

                    KeyTopic {}
                    img {
                        class: "mb-4 rounded-lg",
                        alt: "Figure 8-6 802.1Q Trunking",
                        loading: "lazy",
                        src: asset!("/assets/static/v1p3c8s1sh1f8-6.png", AssetOptions::image().with_avif()),
                    }

                    {h4_heading("The Native VLAN")}
                    p { class: "mb-4",
                        "802.1Q also defines one special VLAN ID on each trunk as the native VLAN (defaulting to use VLAN 1)."
                        br {}
                        "By definition, 802.1Q simply does not add an 802.1Q header to frames in the native VLAN."
                        br {}
                        "When the switch on the other side of the trunk receives a frame that does not have an
                        802.1Q header, the receiving switch knows that the frame is part of the native VLAN."
                        br {}
                        "Note that because of this behavior, both switches must agree on which VLAN is the native VLAN."
                    }

                    p { class: "mb-4",
                        "The 802.1Q native VLAN provides some interesting functions, mainly to support connections to devices 
                        that do not understand trunking."
                        br {}
                        "For example, a Cisco switch could be cabled to a switch that does not understand 802.1Q trunking."
                        br {}
                        "The Cisco switch could send frames in the native VLAN—meaning that the frame has no trunking 
                        header—so that the other switch would understand the frame."
                        br {}
                        "The native VLAN concept gives switches the capability of at least passing traffic in one VLAN 
                        (the native VLAN), which can allow some basic functions, like reachability to telnet into a switch."
                    }
                
                }
            }
        }

        {h3_heading("RECAP")}
        ol { class: "list-disc list-inside",
            li {
                "When you are using VLANs in networks that have multiple interconnected switches, the switches need to use 
                VLAN trunking on  the links between the switches."
            }
            li {
                "VLAN trunking causes the switches to use a process called VLAN tagging, by which the sending switch 
                adds (Note that it's an insertion rather than encapsulation) another  header (This header contains a VLAN ID 
                field used for frame-VLAN association) to the frame before sending it over the trunk."
            }
            li { "As a VLAN trunk, the switches treat the link as if it were a part of all the VLANs." }
            li {
                "The use of trunking allows switches to forward frames from multiple VLANs over a single physical connection by adding a 
                small header to the Ethernet frame."
            }
            li {
                "If you're not going to use VLAN Trunking in networks that have multiple interconnected switches that uses 
                VLANs, you would need to use one link between switches for every VLAN supported on each switch."
            }

            li {
                "Cisco has supported two different trunking protocols over the years: Inter-Switch Link (ISL)
                and IEEE 802.1Q. But Cisco created the ISL years before 802.1Q"
            }
            li {
                "Both 802.1Q and ISL use 12 bits to tag the VLAN ID (4096 VLANs), with two reserved values [0 and 4095]."
            }
            li {
                "Cisco switches break the range of VLAN IDs (1-4094) into two ranges: the normal range and
                the extended range."
            }
            li {
                "ISL is largely obsolete today — 802.1Q became the IEEE standard and is what virtually all modern switches use 
                for VLAN trunking."
            }
            li {
                "802.1Q also defines one special VLAN ID on each trunk as the native VLAN (defaulting to use VLAN 1).
                "
            }
            li {
                "By definition, 802.1Q simply does not add an 802.1Q header to frames in the native VLAN."
            }
        }

    }
}