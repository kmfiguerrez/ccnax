use dioxus::prelude::*;

use crate::{
    components::{GreenNote, my_accordion::{Accordion, AccordionContent, AccordionItem, AccordionTrigger}},
    utils::{h3_heading, h4_heading, TextCommandColor, text_command}
};

#[component]
pub fn Content() -> Element {
    rsx! {
        p { class: "mb-4",
            "This content is found in Appendix D in volume 2 on page 22. I added this content to show enough of the operation of 
            each tool to reinforce your understanding of configuring the basic functions of HSRP."
        }

        Accordion { class: "mb-4",
            AccordionItem {
                AccordionTrigger { {h3_heading("Configuring and Verifying Basic HSRP")} }
                AccordionContent {
                    p { class: "mb-4",
                        strong {
                            "HSRP configuration requires only one command on the two (or more) routers that want to
                            share default router responsibilities with HSRP"
                        }
                        ": the "
                        {text_command("standby", TextCommandColor::Gold)}
                        i { " group " }
                        {text_command("ip", TextCommandColor::Gold)}
                        i { " virtual-ip" }
                        " interface subcommand."
                        br {}
                        "The first value defines the HSRP group number, which must match on both routers."
                        br {}
                        "The group number lets one router support multiple HSRP groups at a time on the
                        same interface, and it allows the routers to identify each other based on the group."
                        br {}
                        "The command also configures the virtual IP address shared by the routers in the same group; the
                        virtual IP address is the address the hosts in the VLAN use as their default gateway."
                    }

                    p { class: "mb-4",
                        "Example D-11 shows a configuration example where both routers use group 1, with virtual
                        IP address 10.1.1.1, with the "
                        {text_command("standby 1 ip 10.1.1.1", TextCommandColor::Gold)}
                        " interface subcommand."
                    }

                    p { class: "mb-4", "See Example D-11 in volume 2 on page 22." }

                    {h4_heading("Routers priority")}
                    p { class: "mb-4",
                        "The configuration shows other optional parameters, as well."
                        br {}
                        "For instance, R1 has a priority of 110 in this group, and R2 defaults to 100."
                        br {}
                        strong {
                            "With HSRP, if the two routers are brought up at the same time, the router with the higher priority wins 
                        the election to become the active router."
                        }
                        br {}
                        "The configuration also shows a name that can be assigned to the group (when using "
                        {text_command("shows", TextCommandColor::Gold)}
                        " commands) and a choice to use HSRP Version 2."
                        br {}
                        "(This chapter provides more details on these settings in the coming pages.)"
                    }

                    p { class: "mb-4",
                        strong {
                            "Once configured, the two routers negotiate the HSRP settings and choose which router will
                            currently be active and which will be standby."
                        }
                        br {}
                        "With the configuration as shown, R1 will win the election and become active because of its higher 
                        (better) priority."
                        br {}
                        "Both routers reach the same conclusion, as confirmed with the output of the "
                        {text_command("show standby brief", TextCommandColor::Gold)}
                        " command on both R1 and R2 in Example D-12."
                    }

                    p { class: "mb-4", "See Example D-12 in volume 2 on page 22." }

                    {h4_heading("The show standby brief command")}
                    p { class: "mb-4",
                        "The "
                        {text_command("show standby brief", TextCommandColor::Gold)}
                        " command packs a lot of detail in the output, so take your time and
                        work through the highlighted fields."
                        br {}
                        "First, look at the Grp column for each command."
                        br {}
                        "This lists the HSRP group number, so when looking at output from multiple routers, you need to look at
                        the lines with the same group number to make sure the data relates to that one HSRP group."
                        br {}
                        "In this case, both routers have only one group number (1), so it is easy to find the information."
                    }

                    p { class: "mb-1",
                        "Each line of output lists the local router's view of the HSRP status for that group."
                        br {}
                        "In particular, based on the headings, the "
                        {text_command("show standby brief", TextCommandColor::Gold)}
                        " command identifies the following:"
                    }
                    ol { class: "list-disc list-inside mb-4",
                        li {
                            span { class: "font-bold", "Interface" }
                            ": The local router's interface on which the HSRP group is configured"
                        }
                        li {
                            span { class: "font-bold", "Grp" }
                            ": The HSRP group number"
                        }
                        li {
                            span { class: "font-bold", "Pri" }
                            ": The local router's HSRP priority"
                        }
                        li {
                            span { class: "font-bold", "State" }
                            ": The local router's current HSRP state"
                        }
                        li {
                            span { class: "font-bold", "Active" }
                            ": The interface IP address of the currently active HSRP router (or “local” if the
                            local router is HSRP active)"
                        }
                        li {
                            span { class: "font-bold", "Standby" }
                            ": The interface IP address of the currently standby HSRP router (or “local” if the
                            local router is HSRP standby)"
                        }
                        li {
                            span { class: "font-bold", "Virtual IP" }
                            ": The virtual IP address defined by this router for this group"
                        }
                    }

                    p { class: "mb-4",
                        "For instance, following the highlighted text in Example D-12, R2 believes that its own current state is 
                        standby, that the router with interface address 10.1.1.9 is active (which happens
                        to be Router R1), with a confirmation that the “local” router (R2, on which this command
                        was issued) is the standby router."
                    }

                    {h4_heading("The show standby command")}
                    p { class: "mb-4",
                        "In comparison, the "
                        {text_command("show standby", TextCommandColor::Gold)}
                        " command  (without the "
                        {text_command("brief", TextCommandColor::Gold)}
                        " keyword) lists a more detailed description of the current state, while repeating many of the facts from 
                        the "
                        {text_command("show standby brief", TextCommandColor::Gold)}
                        " command."
                        br {}
                        "Example D-13 shows an example of the new information with the "
                        {text_command("show standby", TextCommandColor::Gold)}
                        " command, listing several counters and timers about the HSRP protocol itself,
                        plus the virtual MAC address 0000.0c9f.f001."
                    }
                }
            }
            AccordionItem {
                AccordionTrigger { {h3_heading("HSRP Active Role with Priority and Preemption")} }
                AccordionContent {
                    p { class: "mb-4",
                        strong {
                            "HSRP defines some rules to determine which router acts as the active HSRP router and
                            which acts as standby."
                        }
                        br {}
                        "Those rules also define details about when a standby router should take over as active."
                        br {}
                        "The following list summarizes the rules; following the list, this section
                        takes a closer look at those rules and the related configuration settings."
                    }

                    {h4_heading("The HSRP rules")}
                    p { class: "mb-1",
                        "First, the HSRP rules."
                        br {}
                        "When a router (call it the local router) has an HSRP-enabled interface,
                        and that interface comes up, the router sends HSRP messages to negotiate whether it should
                        be active or standby."
                        br {}
                        "When it sends those messages, if it…"
                    }
                    ol { class: "mb-4",
                        li { class: "flex gap-x-4",
                            span { class: "font-bold text-blue-500 shrink-0", "Step 1." }
                            span {
                                "…discovers no other HSRP routers in the subnet, the local router becomes the active router."
                            }
                        }
                        li { class: "flex gap-x-4",
                            span { class: "font-bold text-blue-500 shrink-0", "Step 2." }
                            span {
                                "…discovers an existing HSRP router, and both are currently negotiating to
                                decide which should become the HSRP active router, the routers negotiate,
                                with the router with the highest HSRP priority becoming the HSRP active
                                router."
                            }
                        }
                        li {
                            div { class: "flex gap-x-4",
                                span { class: "font-bold text-blue-500 shrink-0", "Step 3." }
                                span {
                                    "…discovers an existing HSRP router in the subnet, and that router is already acting as the active 
                                router:"
                                }
                            }
                            ol { class: "pl-16",
                                li {
                                    span { class: "font-bold text-blue-500 uppercase",
                                        "a."
                                    }
                                    " If configured with no preemption (the default; "
                                    {text_command("no standby preempt", TextCommandColor::Gold)}
                                    "), the local router becomes a standby router, even if it has a better (higher) priority."
                                }
                                li {
                                    span { class: "font-bold text-blue-500 uppercase",
                                        "b."
                                    }
                                    " If configured with preemption ("
                                    {text_command("standby preempt", TextCommandColor::Gold)}
                                    "), the local router checks its priority versus the active router; if the local router priority 
                                    is better (higher), the local router takes over (preempts) the existing active router to
                                    become the new active HSRP router."
                                }
                            }
                        }
                    }

                    p { class: "mb-4",
                        "Steps 1 and 2 in the list are pretty obvious, but steps 3A and 3B could use a little closer look."
                        br {}
                        "For instance, the examples so far in this chapter show R1's G0/0 with a priority of 110
                        versus R2's G0/0 with priority 100."
                        br {}
                        "The "
                        {text_command("show", TextCommandColor::Gold)}
                        " commands in Example D-13 show that R1 is currently the HSRP active router."
                        br {}
                        "That same example also lists a line for both R1 and R2
                        that confirms “preemption disabled,” which is the default."
                    }

                    {h4_heading("Test of step 3A logic")}
                    p { class: "mb-4",
                        "To show a test of step 3A logic, Example D-14 shows a process by which R1's G0/0 interface is disabled 
                        and then enabled again, but after giving Router R2 long enough to take over
                        and become active."
                        br {}
                        "That is, R1 comes up but R2 is already HSRP active for group 1."
                        br {}
                        "The bottom of the example lists output from the "
                        {text_command("show standby brief", TextCommandColor::Gold)}
                        " command from R2, confirming that R2 
                        becomes HSRP active and R1 becomes standby (10.1.1.9), proving that R1
                        does not preempt R2 in this case."
                    }

                    p { class: "mb-4", "See Example D-14 in appendix in volume 1 on page 25." }

                    {h4_heading("Test of step 3B logic")}
                    p { class: "mb-4",
                        "If R1 had been configured with preemption for that previous scenario, R1 would have taken
                        over from R2 when R1's interface came back up."
                        br {}
                        "Example D-15 shows exactly that."
                        br {}
                        "Before the output in Example D-15 was gathered, the network had been put back to the same
                        beginning state as at the beginning of Example D-14, with R1 active and R2 as standby."
                        br {}
                        "Within Example D-15, R1's interface is shut down, then configured with preemption using
                        the "
                        {text_command("standby 1 preempt", TextCommandColor::Gold)}
                        " command, enabling preemption."
                        br {}
                        "Then, after enabling the interface again, R1 takes over as HSRP active, as shown at the bottom of the 
                        example's "
                        {text_command("show standby brief", TextCommandColor::Gold)}
                        " command from R2."
                        br {}
                        "That output now shows the local router's state as Standby, and the
                        active as 10.1.1.9 (R1)."
                    }

                    p { class: "mb-4", "See Example D-15 in appendix in volume 1 on page 26." }

                    p { class: "mb-4",
                        strong {
                            "Note that it is the preemption setting on the router that is taking over (preempting) that
                            determines if preemption happens."
                        }
                        br {}
                        "For instance, in this case, R1 came up when R2 was active; R1 was set to preempt; so R1 preempted R2."
                    }
                }
            }
            AccordionItem {
                AccordionTrigger { {h3_heading("HSRP Versions")} }
                AccordionContent {
                    p { class: "mb-4",
                        strong {
                            "Cisco IOS on routers and Layer 3 switches supports two versions of HSRP: versions 1 and 2."
                        }
                        br {}
                        "The versions have enough differences, like multicast IP addresses used and message formats,
                        so that routers in the same HSRP group must use the same version."
                        br {}
                        strong {
                            "If two routers configured to be in the same HSRP group mistakenly configure to use different versions, 
                        they will not understand each other and ignore each other for the purposes of HSRP."
                        }
                    }

                    p { class: "mb-4",
                        "To configure the version, each interface/subinterface uses the "
                        {text_command("standby version {1 | 2}", TextCommandColor::Gold)}
                        " interface subcommand."
                        br {}
                        strong {
                            "Note that the HSRP group number is not included in the command, because it sets the version for all 
                        HSRP messages sent out that interface/subinterface."
                        }
                    }

                    {h4_heading("Reasons to use HSRPv2")}
                    p { class: "mb-4",
                        "There are some good reasons to want to use the more recent HSRP version 2 (HSRPv2)."
                        br {}
                        "For example, HSRPv1 existed before IPv6 became popular."
                        br {}
                        "Cisco enhanced HSRP to version 2 in part to make IPv6 support possible."
                        br {}
                        strong { "Today, to use HSRP with IPv6 requires HSRPv2." }
                    }

                    p { class: "mb-4",
                        "As another example of a benefit of HSRPv2, HSRP uses a Hello message, similar in concept
                        to routing protocols, so that HSRP group members can realize when the active router is no
                        longer reachable."
                        br {}
                        "HSRPv2 allows for shorter Hello timer configuration (as low as a small number of milliseconds), 
                        while HSRPv1 typically had a minimum of 1 second."
                        br {}
                        strong {
                            "So, HSRPv2 can be configured to react more quickly to failures with a lower Hello timer."
                        }
                    }

                    p { class: "mb-4",
                        "Beyond IPv6 support and shorter Hello timer options, other differences for version 2 versus version 1 
                        include a different virtual MAC address base value and a different multicast IP address used as the 
                        destination for all messages."
                        br {}
                        "Table D-3 lists the differences between HSRPv1 and HSRPv2."
                    }

                    img {
                        class: "mb-4 rounded-lg",
                        loading: "lazy",
                        alt: "Table D-3 HSRPv1 Versus HSRPv2",
                        src: asset!("/assets/static/v2p3c12s1sh5exd3-5.png", AssetOptions::image().with_avif()),
                    }

                    {h4_heading("Identifying MAC Addresses")}
                    p { class: "mb-4",
                        "Of the details in the table, make sure to look at the MAC addresses for both versions 1 and 2."
                        br {}
                        "Cisco reserves the prefixes of 0000.0C07.AC for HSRPv1 and 0000.0C9F.F for HSRPv2."
                        br {}
                        strong {
                            "HSRPv1, with 256 possible HSRP groups per interface, then uses the last two hex digits to
                            identify the HSRP group."
                        }
                        br {}
                        "For example, an HSRP group 1 using version 1 would use a virtual MAC address that ends in hex 01."
                        br {}
                        strong {
                            "Similarly, because HSRPv2 supports 4096 groups per interface, the MAC address reserves three hex digits to 
                            identify the group."
                        }
                        br {}
                        "An HSRP group 1 using version 2 would use a virtual MAC address that ends in hex 001."
                    }

                    GreenNote {
                        p {
                            strong { "NOTE" }
                            " The content under the heading “Gateway Load Balancing Protocol (GLBP)” was
                            most recently published for the 200-105 Exam in 2016, in Appendix K of the Cisco CCNA
                            ICND2 200-105 Official Cert Guide."
                        }
                    }
                }
            }
        }

        {h3_heading("RECAP")}
        ol { class: "list-disc list-inside",
            li {
                "HSRP configuration requires only one command on the two (or more) routers that want to
                share default router responsibilities with HSRP: the "
                {text_command("standby", TextCommandColor::Gold)}
                i { " group " }
                {text_command("ip", TextCommandColor::Gold)}
                i { " virtual-ip" }
                " interface subcommand."
            }
            li {
                "The group number lets one router support multiple HSRP groups at a time on the same interface, and it allows 
                the routers to identify each other based on the group."
            }
            li {
                "With HSRP, if the two routers are brought up at the same time, the router with the higher (better) priority wins 
                the election to become the active router."
            }
            li {
                "Once configured, the two routers negotiate the HSRP settings and choose which router will currently be active and 
                which will be standby."
            }
            li {
                "The HSRP configuration can be verified via the "
                {text_command("show standby brief", TextCommandColor::Gold)}
                " command."
            }
            li {
                "HSRP defines some rules to determine which router acts as the active HSRP router and which acts as standby."
                " Those rules also define details about when a standby router should take over as active."
            }
            li {
                "A router's interface participating in the HSRP has a default "
                {text_command("no standby preempt", TextCommandColor::Gold)}
                " interface subcommand configured and will never preempt ( never attempts to take over even if it has a better
                HSRP priority ) as the active router when there's already an active router."
            }
            li {
                "If an interface is configured with the "
                {text_command("standby preempt", TextCommandColor::Gold)}
                " interface subcommand, it'll try to preempt if it has a higher (better) HSRP priority."
            }
            li {
                "Cisco IOS on routers and Layer 3 switches supports two versions of HSRP: versions 1 and 2."
                " With v2 has many benefits over v1."
            }
            li {
                "If two routers configured to be in the same HSRP group mistakenly configure to use different versions, 
                they will not understand each other and ignore each other for the purposes of HSRP."
            }
        }
    }
}