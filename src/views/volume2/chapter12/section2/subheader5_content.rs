use dioxus::prelude::*;

use crate::{
    components::{
        ConfigChecklist, GreenNote, my_accordion::{Accordion, AccordionContent, AccordionItem, AccordionTrigger}
    }, utils::{TextCommandColor, h3_heading, h4_heading, text_command}
};

#[component]
pub fn Content() -> Element {
    rsx! {
        p { class: "mb-4",
            "SNMPv1 and SNMPv2c configuration is very similar, because both use communities."
            br {}
            "SNMPv3 varies quite a bit, mainly to implement the better SNMPv3 security features."
        }

        Accordion { class: "mb-4",
            AccordionItem {
                AccordionTrigger { {h3_heading("Configuring SNMPv2c Support for Get and Set")} }
                AccordionContent {
                    p { class: "mb-4",
                        "SNMP configuration in Cisco IOS routers and switches works a little differently than many
                        other IOS features."
                        br {}
                        strong {
                            "First, the SNMP configuration exists in a series of global commands;
                            there is no SNMP agent configuration mode in which to collect subcommands."
                        }
                        br {}
                        "Secondly, no single command enables the SNMP agent."
                        br {}
                        "Instead, IOS typically defaults for the SNMP agent to be disabled."
                        br {}
                        "Then, the first time an "
                        {text_command("snmp-server", TextCommandColor::Gold)}
                        " global command is configured, IOS enables the SNMP agent."
                    }

                    GreenNote {
                        p {
                            strong { "NOTE" }
                            " To disable the SNMP agent, you must remove all the "
                            {text_command("snmp-server", TextCommandColor::Black)}
                            " commands. You can do this with a single "
                            {text_command("no snmp-server", TextCommandColor::Black)}
                            " command (with no parameters)."
                        }
                    }

                    p { class: "mb-4",
                        "With that backdrop, a typical SNMPv2c configuration requires only one or two settings."
                        br {}
                        strong { "To be useful, the agent needs at least a read-only (RO) community string." }
                        br {}
                        "The agent will not reply to SNMPv2c Get messages without at least the RO community string configured."
                        br {}
                        "The network engineer may also want the agent to have a read-write (RW) community string, to
                        support Set messages."
                    }

                    GreenNote {
                        p {
                            strong { "NOTE" }
                            " When configuring an RW community, use some caution: configuring an RW community means that you have 
                            defined a clear-text password that can be used to configure many settings on the router or switch."
                        }
                    }

                    ConfigChecklist {}
                    p { class: "mb-1",
                        "The following checklist details the commands used to configure SNMPv2c on a Cisco router or switch."
                        br {}
                        "This list shows the method to configure the RO and RW communities, plus a few optional but common settings 
                        (location and contact information)."
                    }
                    ol { class: "mb-4",
                        li { class: "flex gap-x-4",
                            span { class: "font-bold text-blue-500 shrink-0", "Step 1." }
                            span {
                                "Use the "
                                {text_command("snmp-server community", TextCommandColor::Gold)}
                                i { " communitystring " }
                                {text_command("RO", TextCommandColor::Gold)}
                                " ["
                                {text_command("ipv6", TextCommandColor::Gold)}
                                i { " acl-name" }
                                "] ["
                                i { "acl-name" }
                                "] command in global configuration mode to enable the SNMP
                                agent (if not already started), set the read-only community string, and restrict
                                incoming SNMP messages based on the optional referenced IPv4 or IPv6 ACL."
                            }
                        }
                        li { class: "flex gap-x-4",
                            span { class: "font-bold text-blue-500 shrink-0", "Step 2." }
                            span {
                                "(Optional) Use the "
                                {text_command("snmp-server community", TextCommandColor::Gold)}
                                i { " communitystring " }
                                {text_command("RW", TextCommandColor::Gold)}
                                " ["
                                {text_command("ipv6", TextCommandColor::Gold)}
                                i { " acl-name" }
                                "] ["
                                i { "acl-name" }
                                "] command in global configuration mode to enable the SNMP
                                agent (if not already started), set the read-only community string, and restrict
                                incoming SNMP messages based on the optional referenced IPv4 or IPv6 ACL."
                            }
                        }
                        li { class: "flex gap-x-4",
                            span { class: "font-bold text-blue-500 shrink-0", "Step 3." }
                            span {
                                "(Optional) If referenced by an "
                                {text_command("snmp-server community", TextCommandColor::Gold)}
                                " command, configure an IPv4 or IPv6 ACL, with the same name or number referenced by the "
                                {text_command("snmp-server community", TextCommandColor::Gold)}
                                " command, with the ACL permitting by matching the source IPv4 or IPv6 address 
                                of the allowed SNMP management hosts."
                            }
                        }
                        li { class: "flex gap-x-4",
                            span { class: "font-bold text-blue-500 shrink-0", "Step 4." }
                            span {
                                "(Optional) Use the "
                                {text_command("snmp-server location", TextCommandColor::Gold)}
                                i { " text-describing-location" }
                                " command in global configuration mode to document the location of the device."
                            }
                        }
                        li { class: "flex gap-x-4",
                            span { class: "font-bold text-blue-500 shrink-0", "Step 5." }
                            span {
                                "(Optional) Use the "
                                {text_command("snmp-server contact", TextCommandColor::Gold)}
                                i { " contact-name" }
                                " command in global configuration mode to document the person to contact if problems occur."
                            }
                        }
                    }

                    GreenNote {
                        p {
                            strong { "NOTE" }
                            " In the SNMP model, the SNMP agent acts as a server, with the NMS (SNMP Manager) acting as an SNMP 
                            client by requesting information with Get messages. The IOS "
                            {text_command("snmp-server", TextCommandColor::Black)}
                            " command happens to emphasize  the idea that the SNMP agent on a router or switch acts as the SNMP 
                            server."
                        }
                    }

                    p { class: "mb-4",
                        "Example D-20 shows a sample configuration based on Figure D-12."
                        br {}
                        "The examples in this section come from Router R1, although the exact same SNMP configuration syntax could
                        be used in the LAN switches or in R2."
                        br {}
                        "(The configuration of the location information would likely differ for each device, however.)"
                        br {}
                        "Note that the configuration creates an IPv4 ACL that permits traffic with source IP address 10.1.3.3, 
                        which is the address of the NMS shown in the figure."
                        br {}
                        "It then defines read-only and read-write communities, along with the location and contact name for the 
                        router."
                    }

                    img {
                        class: "mb-4 rounded-lg",
                        loading: "lazy",
                        alt: "Figure D-12 Sample Network for SNMP Examples, with NMS at 10.1.3.3",
                        src: asset!("/assets/static/v2p3c12s2sh5fD-12.png", AssetOptions::image().with_avif()),
                    }

                    img {
                        class: "mb-4 rounded-lg",
                        loading: "lazy",
                        alt: "Example D-20 Configuring SNMP Version 2c on Router R1 to Support Get and Set",
                        src: asset!("/assets/static/v2p3c12s2sh5exD-20.png", AssetOptions::image().with_avif()),
                    }

                    p { class: "mb-4",
                        "To begin managing Router R1 (or any of the other devices that use the same community strings), the SNMP 
                        manager at address 10.1.3.3 now needs to configure the community strings listed in Example D-20."
                    }
                }
            }
            AccordionItem {
                AccordionTrigger { {h3_heading("Configuring SNMPv2c Support for Trap and Inform")} }
                AccordionContent {
                    p { class: "mb-4",
                        "For an SNMPv2c agent in a router or switch to be able to send unsolicited notifications to
                        an SNMP manager (that is, to send Trap and Inform messages), the device needs to be configured with the "
                        {text_command("snmp-server host", TextCommandColor::Gold)}
                        " command."
                        br {}
                        "This command references the NMS to which the Traps or Informs should be sent, along with the SNMP version."
                    }

                    p { class: "mb-4",
                        strong {
                            "Beyond telling the SNMP agent the hostname or address of the NMS, the agent typically
                            needs to know the notification community string used by the NMS."
                        }
                        br {}
                        "Think of the RO and RW community strings as protecting the SNMP agent from the messages originated by an
                        NMS (Get or Set Requests), so the agent requires the NMS to supply the correct RO or
                        RW community string."
                        br {}
                        "For Traps and Informs, the NMS can protect itself from the Trap and Inform messages originated by SNMP 
                        agents by requiring those agents to include the notification community with those messages."
                        br {}
                        "The agent can configure this value on the "
                        {text_command("snmp-server host", TextCommandColor::Gold)}
                        " command as well."
                    }

                    ConfigChecklist {}
                    p { class: "mb-1",
                        "The following list details the command to enable the sending of SNMPv2c Trap or Inform
                        messages to an NMS:"
                    }
                    ol { class: "mb-4",
                        li { class: "flex gap-x-4",
                            span { class: "font-bold text-blue-500 shrink-0", "Step 1." }
                            span {
                                "Use the "
                                {text_command("snmp-server host", TextCommandColor::Gold)}
                                " {{"
                                i { "hostname" }
                                " | "
                                i { "ip-address" }
                                "}} "
                                "["
                                {text_command("informs", TextCommandColor::Gold)}
                                "] "
                                {text_command("version 2c", TextCommandColor::Gold)}
                                i { " notification-community" }
                                " command in global configuration mode to configure the SNMP agent to send either SNMPv2c Traps 
                                (default) or Informs to the listed host. Use this command once for each host to which this device 
                                should send Traps."
                            }
                        }
                        li { class: "flex gap-x-4",
                            span { class: "font-bold text-blue-500 shrink-0", "Step 2." }
                            span {
                                "Use the "
                                {text_command("snmp-server enable traps", TextCommandColor::Gold)}
                                " command in global configuration mode to enable the sending of all supported types of Trap and 
                                Inform messages."
                            }
                        }
                    }

                    p { class: "mb-4",
                        "Example D-21 shows a sample configuration."
                        br {}
                        strong {
                            "In most cases, you would send either Traps or Informs to a particular NMS, but not both."
                        }
                        br {}
                        "So, for this example, the configuration shows how to configure to send Traps to one host (10.1.3.3), and 
                        Informs to another host (10.1.3.4)."
                        br {}
                        "Note that this configuration is added to Router R1 from Figure D-12, but it could have been added to 
                        Router R2 or to any of the LAN switches as well."
                    }

                    img {
                        class: "mb-4 rounded-lg",
                        loading: "lazy",
                        alt: "Example D-21 Configuring SNMP Version 2c on Router R1 to Support Sending Traps",
                        src: asset!("/assets/static/v2p3c12s2sh5exD-21.png", AssetOptions::image().with_avif()),
                    }
                }
            }
            AccordionItem {
                AccordionTrigger { {h3_heading("Verifying SNMPv2c Operation")} }
                AccordionContent {
                    p { class: "mb-4",
                        "Example D-22 displays some of the status information based on the configuration seen in
                        the previous two examples."
                        br {}
                        "The variations on the "
                        {text_command("show snmp", TextCommandColor::Gold)}
                        " command highlight several configuration settings."
                        br {}
                        "For example, the "
                        {text_command("show snmp community", TextCommandColor::Gold)}
                        " command repeats the community string values, with reference to any 
                        attached IPv4 or IPv6 ACLs."
                        br {}
                        "The "
                        {text_command("show snmp host", TextCommandColor::Gold)}
                        " command lists the IP address or hostname of the NMS referenced by each "
                        {text_command("snmp-server host", TextCommandColor::Gold)}
                        " configuration command."
                    }

                    p { class: "mb-4", "See Example D-22 in volume 2, appendix D on page 654." }

                    {h4_heading("The show snmp command")}
                    p { class: "mb-4",
                        "The "
                        {text_command("show snmp", TextCommandColor::Gold)}
                        " command takes the opposite approach from the commands in Example D-22, focusing almost 
                        completely on status and counter information, rather than repeating configuration settings."
                        br {}
                        "This command lists dozens of lines of detailed information, so the sample in Example D-23 shows just 
                        enough of the output to give you a sense of the kinds of information found there, with comments following 
                        the example."
                    }

                    p { class: "mb-4", "See Example D-23 in volume 2, appendix D on page 655." }

                    p { class: "mb-4",
                        "The output in Example D-23 was taken from Router R1 as shown in the earlier examples,
                        after doing some testing from the NMS at address 10.1.3.3."
                        br {}
                        "The highlighted items point out the number of SNMP packets received (input) and sent (output), as well as 
                        the number of requested MIB variables—that is, the number of variables requested in different SNMP Get
                        requests."
                        br {}
                        "(Note that SNMP also supports the GetNext and GetBulk commands, so a single NMS user click can cause the 
                        NMS to Get many variables from an agent; thus, it is not unusual for the requested variables counter to get 
                        very large.)"
                        br {}
                        "The output also shows that seven Set requests were received, resulting in two changes to variables."
                        br {}
                        "The fact that two Set requests changed variables is a good fact to know if you are wondering if someone 
                        has reconfigured something on the device using SNMP."
                    }
                
                }
            }
        }

        {h3_heading("RECAP")}
        ol { class: "list-disc list-inside",
            li {
                "The SNMP configuration exists in a series of global commands; there is no SNMP agent configuration mode in which 
                to collect subcommands."
            }
            li { "IOS typically defaults for the SNMP agent to be disabled." }
            li {
                "Then, the first time an "
                {text_command("snmp-server", TextCommandColor::Gold)}
                " global command is configured, IOS enables the SNMP agent."
            }
            li { "To be useful, the agent needs at least a read-only (RO) community string." }
            li {
                "In most cases, you would send either Traps or Informs to a particular NMS, but not both."
            }
        }
    }
}