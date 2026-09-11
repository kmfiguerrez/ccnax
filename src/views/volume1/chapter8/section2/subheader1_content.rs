use dioxus::prelude::*;

use crate::{
    components::{
        ConfigChecklist,
        GreenNote,
        my_accordion::{Accordion, AccordionContent, AccordionItem, AccordionTrigger}
    }, 
    utils::{TextCommandColor, h3_heading, h4_heading, text_command}
};

#[component]
pub fn Content() -> Element {
    rsx! {
        p { class: "mb-4",
            "This section shows how to create a VLAN, give the VLAN a name, and assign interfaces to a VLAN."
            br {}
            "To focus on these basic details, this section shows examples using a single switch, so VLAN trunking is not needed."
        }

        {h3_heading("VLAN Configuration")}
        p { class: "mb-4",
            "For a Cisco switch to forward frames in a particular VLAN, the switch must be configured
            to believe that the VLAN exists."
            br {}
            "In addition, the switch must have nontrunking interfaces (called access interfaces, or static access interfaces) 
            assigned to the VLAN, and/or trunks that support the VLAN."
            br {}
            "The configuration steps for access interfaces are as follows:"
        }
        ConfigChecklist {}
        ol { class: "mb-4",
            // Step 1
            li { class: "flex flex-col md:flex-row md:gap-x-4",
                span { class: "text-sky-500 font-semibold shrink-0", "Step 1." }
                div {
                    span { "To configure a new VLAN, follow these steps:" }
                    ol {
                        // Step A
                        li {
                            span { class: "text-sky-500 font-semibold uppercase mr-2",
                                "a."
                            }
                            " From configuration mode, use the "
                            {text_command("vlan", TextCommandColor::Gold)}
                            i { " vlan-id" }
                            " command in global configuration mode to create the 
                            VLAN and to move the user into VLAN configuration mode."
                        }
                        // Step B
                        li {
                            span { class: "text-sky-500 font-semibold uppercase mr-2",
                                "b."
                            }
                            "(Optional) Use the "
                            {text_command("name", TextCommandColor::Gold)}
                            i { " name" }
                            " command in VLAN configuration mode
                            to list a name for the VLAN. If not configured, the VLAN name is
                            VLANZZZZ, where ZZZZ is the four-digit decimal VLAN ID."
                        }
                    }
                }
            
            }
            // Step 2
            li { class: "flex flex-col md:flex-row md:gap-x-4",
                span { class: "text-sky-500 font-semibold shrink-0", "Step 2." }
                div {
                    span { "For each access interface, follow these steps:" }
                    ol {
                        // Step A
                        li {
                            span { class: "text-sky-500 font-semibold uppercase mr-2",
                                "a."
                            }
                            " Use the "
                            {text_command("interface", TextCommandColor::Gold)}
                            i { " type number" }
                            " command in global configuration mode to
                            move into interface configuration mode for each desired interface."
                        }
                        // Step B
                        li {
                            span { class: "text-sky-500 font-semibold uppercase mr-2",
                                "b."
                            }
                            "Use the "
                            {text_command("switchport access vlan", TextCommandColor::Gold)}
                            i { " id-number" }
                            " command in interface configuration mode to specify the 
                            VLAN number associated with that interface."
                        }
                        // Step C
                        li {
                            span { class: "text-sky-500 font-semibold uppercase mr-2",
                                "b."
                            }
                            " (Optional) Use the "
                            {text_command("switchport mode access", TextCommandColor::Gold)}
                            " command in interface configuration mode to make this 
                            port always operate in access mode (that is, to not trunk)."
                        }
                    }
                }
            
            }
        }

        p { class: "mb-4",
            "While the list might look a little daunting, the process on a single switch is actually pretty
            simple."
            br {}
            "For example, if you want to put the switch's ports in three VLANs—11, 12, and 13—you first add three "
            {text_command("vlan", TextCommandColor::Gold)}
            " commands: "
            {text_command("vlan 11", TextCommandColor::Gold)}
            ", "
            {text_command("vlan 12", TextCommandColor::Gold)}
            ", and "
            {text_command("vlan 13", TextCommandColor::Gold)}
            "."
            br {}
            "Then, for each interface, add a "
            {text_command("switchport access vlan 11", TextCommandColor::Gold)}
            " (or "
            {text_command("12", TextCommandColor::Gold)}
            " or "
            {text_command("13", TextCommandColor::Gold)}
            ") command to assign that interface to the
            proper VLAN."
        }

        GreenNote {
            p {
                strong { "NOTE" }
                " The term "
                i { "default VLAN" }
                " (as shown in the exam topics) refers to the default setting
                on the "
                {text_command("switchport access vlan", TextCommandColor::Black)}
                i { " vlan-id" }
                " command, and that default is VLAN ID 1."
                " In other words, by default, each port is assigned to access VLAN 1."
            }
        }

        Accordion { class: "mb-4",
            AccordionItem {
                AccordionTrigger { {h3_heading("VLAN Configuration Example 1: Full VLAN Configuration")} }
                AccordionContent {
                    p { class: "mb-4",
                        "Examples 8-1, 8-2, and 8-3 work through one scenario with VLAN configuration and verification."
                        br {}
                        "To begin, Example 8-1 begins by showing the VLANs in switch SW1 in Figure 8-9,
                        with all default settings related to VLANs."
                    }

                    img {
                        class: "mb-4 rounded-lg",
                        alt: "Figure 8-9 Network with One Switch and Three VLANs",
                        loading: "lazy",
                        src: asset!("/assets/static/v1p3c8s2sh1f8-9.png", AssetOptions::image().with_avif()),
                    }

                    p { class: "mb-4", "See Example 8-1 in volume 1 on page 186." }

                    {h4_heading("The show vlan brief command")}
                    p { class: "mb-4",
                        "The example begins with the "
                        {text_command("show vlan brief", TextCommandColor::Gold)}
                        " command, confirming the default settings
                        of five nondeletable VLANs, with all interfaces assigned to VLAN 1."
                        br {}
                        "VLAN 1 cannot be deleted but can be used."
                        br {}
                        "VLANs 1002-1005 cannot be deleted and cannot be used as access VLANs today."
                        br {}
                        "In particular, note that this 2960 switch has 24 Fast Ethernet ports (Fa0/1-Fa0/24) and two 
                        Gigabit Ethernet ports (Gi0/1 and Gi0/2), all of which are listed as being in VLAN 1 per that first 
                        command's output, confirming that by default, Cisco switches assign all ports to VLAN 1."
                    }

                    p { class: "mb-4",
                        "Next, Example 8-2 shows steps that mirror the VLAN configuration checklist, namely the
                        configuration of VLAN 2, plus the assignment of VLAN 2 as the access VLAN on two
                        ports: Fa0/13 and Fa0/14."
                    }

                    p { class: "mb-4", "See Example 8-2 in volume 1 on page 187." }

                    p { class: "mb-4",
                        "Take a moment to compare the output of the "
                        {text_command("show vlan brief", TextCommandColor::Gold)}
                        " commands in Example 8-2 (after adding the configuration) versus Example 8-1."
                        br {}
                        "Example 8-2 shows new information about VLAN 2, with ports Fa0/13 and Fa0/14 no longer being listed 
                        with VLAN 1, but now listed as assigned to VLAN 2."
                    }

                    {h4_heading("Using the show running-config command to verify VLANs config")}
                    p { class: "mb-4",
                        "To complete this scenario, Example 8-3 shows a little more detail about the VLAN itself."
                        br {}
                        "First, the "
                        {text_command("show running-config", TextCommandColor::Gold)}
                        " command lists both the "
                        {text_command("vlan 2", TextCommandColor::Gold)}
                        " and "
                        {text_command("switchport access vlan 2", TextCommandColor::Gold)}
                        " commands as configured in Example 8-2."
                        br {}
                        "Also, note that earlier Example 8-2 uses the "
                        {text_command("interface range", TextCommandColor::Gold)}
                        " command, with one instance of the "
                        {text_command("switchport access vlan 2", TextCommandColor::Gold)}
                        "interface subcommand."
                        br {}
                        "However, Example 8-3 shows how the switch actually applied that command to both Fa0/13 and Fa0/14."
                        br {}
                        "Example 8-3 ends with the "
                        {text_command("show vlan id 2", TextCommandColor::Gold)}
                        " command, which confirms the operational status that ports 
                        Fa0/13 and Fa0/14 are assigned to VLAN 2."
                    }

                    p { class: "mb-4", "See Example 8-3 in volume 1 on page 188." }

                    {h4_heading("The switchport mode access")}
                    p { class: "mb-4",
                        "The example surrounding Figure 8-9 uses six switch ports, all of which need to operate as
                        access ports."
                        br {}
                        "That is, each port should not use trunking but instead should be assigned to
                        a single VLAN, as assigned by the "
                        {text_command("switchport access vlan", TextCommandColor::Gold)}
                        i { " vlan-id" }
                        " command."
                        br {}
                        "For ports that should always act as access ports, add the optional interface subcommand "
                        {text_command("switchport mode access.", TextCommandColor::Gold)}
                        br {}
                        "This command tells the switch to always be an access interface and disables
                        the protocol that negotiates trunking (Dynamic Trunking Protocol [DTP]) with the device on
                        the other end of the link."
                        br {}
                        "(The upcoming section “VLAN Trunking Configuration” discusses
                        more details about the commands that allow a port to negotiate whether it should use
                        trunking."
                    }
                
                }
            }
            AccordionItem {
                AccordionTrigger { {h3_heading("VLAN Configuration Example 2: Shorter VLAN Configuration")} }
                AccordionContent {}
            }
        }

        {h3_heading("RECAP")}
        ol { class: "list-disc list-inside",
            li {
                "For a Cisco switch to forward frames in a particular VLAN, the switch must be configured to believe that the 
                VLAN exists and an interface assigned to the VLAN."
            }
            li { "Non-trunking interfaces are called access interfaces, or static access interfaces." }
            li { "VLAN 1 cannot be deleted but can be used." }
            li { "VLANs 1002-1005 cannot be deleted and cannot be used as access VLANs today." }
            li {
                "Both the "
                {text_command("show vlan/show vlan brief", TextCommandColor::Gold)}
                " and "
                {text_command("show running-config", TextCommandColor::Gold)}
                " commands to verify VLANs configuration."
            }
            li {
                "For ports that should always act as access ports, add the optional interface subcommand "
                {text_command("switchport mode access.", TextCommandColor::Gold)}
                " This command tells the switch to always be an access interface and disables the protocol that negotiates trunking 
                (Dynamic Trunking Protocol [DTP]) with the device on the other end of the link."
            }
        }
    }
}