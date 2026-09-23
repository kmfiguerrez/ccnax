use dioxus::prelude::*;

use crate::{
    components::{
        GreenNote, RedNote, my_accordion::{Accordion, AccordionContent, AccordionItem, AccordionTrigger}
    }, utils::{TextCommandColor, h3_heading, h4_heading, text_command}};

#[component]
pub fn Content() -> Element {
    rsx! {
        p { class: "mb-4",
            strong {
                "When using Layer 2 EtherChannels, a switch's MAC learning process associates MAC
                addresses with the PortChannel interfaces and not the underlying physical ports."
            }
            br {}
            "Later, when a switch makes a forwarding decision to send a frame out a PortChannel interface, 
            the switch must do more work: to decide out which specific physical port to use to forward the frame."
            br {}
            "IOS documentation refers to those rules as "
            i { "EtherChannel load distribution" }
            " or "
            i { "load balancing" }
            "."
            br {}
            "Figure 10-8 shows the main idea."
        }

        img {
            class: "mb-4 rounded-lg",
            alt: "Figure 10-8 Correct EtherChannel Configuration Combinations",
            loading: "lazy",
            src: asset!("/assets/static/v1p3c10s2sh4f10-8.png", AssetOptions::image().with_avif()),
        }

        Accordion { class: "mb-4",
            AccordionItem {
                AccordionTrigger { {h3_heading("Configuration Options for EtherChannel Load Distribution")} }
                AccordionContent {
                    p { class: "mb-4",
                        strong {
                            "EtherChannel load distribution makes the choice for each frame based on various numeric
                            values found in the Layer 2, 3, and 4 headers."
                        }
                        br {}
                        "The process uses one configurable setting as
                        input: the load distribution method as defined with the "
                        {text_command("port-channel load-balance", TextCommandColor::Gold)}
                        i { " method" }
                        " global command."
                        br {}
                        "The process then performs some match against the fields identified by the
                        configured method."
                    }

                    p { class: "mb-4",
                        "Table 10-4 lists the most common methods."
                        br {}
                        "However, note that some switches may support only MAC-based methods, or only MAC- and IP-based methods, 
                        depending on the model and software version."
                    }

                    img {
                        class: "mb-4 rounded-lg",
                        alt: "Table 10-4 EtherChannel Load Distribution Methods",
                        loading: "lazy",
                        src: asset!("/assets/static/v1p3c10s2sh4t10-4.png", AssetOptions::image().with_avif()),
                    }

                    p { class: "mb-1",
                        "To appreciate why you might want to use different methods, you need to consider the results
                        of how switches make their choice."
                        br {}
                        "("
                        strong {
                            "The discussion here focuses on the result, and not the logic, because the logic remains internal to 
                            the switch, and Cisco does not document how each switch model or IOS version works internally"
                        }
                        ".)"
                        br {}
                        "However, the various load distribution algorithms do share some common goals:"
                    }
                    ol { class: "list-disc list-inside mb-4",
                        li {
                            "To cause all messages in a single application flow to use the same link in the channel,
                            rather than being sent over different links. Doing so means that the switch will not inadvertently 
                            reorder the messages sent in that application flow by sending one message over
                            a busy link that has a queue of waiting messages, while immediately sending the next
                            message out an unused link."
                        }
                        li {
                            "To integrate the load distribution algorithm work into the hardware forwarding ASIC so
                            that load distribution works just as quickly as the work to forward any other frame."
                        }
                        li {
                            "To use all the active links in the EtherChannel, adjusting to the addition and removal of
                            active links over time."
                        }
                        li {
                            "Within the constraints of the other goals, balance the traffic across those active links."
                        }
                    }

                    p { class: "mb-4",
                        "In short, the algorithms first intend to avoid message reordering, make use of the switch forwarding 
                        ASICs, and use all the active links."
                        br {}
                        "However, the algorithm does not attempt to send the exact same number of bits over each link over time."
                        br {}
                        "The algorithm does try to balance the traffic, but always within the constraints of the other goals."
                    }

                    p { class: "mb-4",
                        "Whatever load distribution method you choose, the method identifies fields in the message headers."
                        br {}
                        "Any messages in the same application flow will then have the same values in the fields
                        used by the load distribution algorithm and will always be forwarded over the same link."
                        br {}
                        "For example, when a user connects to a website, that web server may return thousands of packets
                        to the client."
                        br {}
                        "Those thousands of packets should flow over the same link in the EtherChannel."
                    }

                    p { class: "mb-4",
                        "For instance, with the load distribution method of "
                        {text_command("src-mac", TextCommandColor::Gold)}
                        " (meaning source MAC address), all frames with the same MAC address flow over one link."
                        br {}
                        "Figure 10-9 shows the idea with pseudo MAC addresses, with the load distribution sending frames with 
                        source MAC 1 over link 1, source MAC 2 over link 2, and source MAC 3 over link 3."
                    }

                    img {
                        class: "mb-4 rounded-lg",
                        alt: "Figure 10-9 Distributing All Frames with Same Mac Out Same Interface",
                        loading: "lazy",
                        src: asset!("/assets/static/v1p3c10s2sh4f10-9.png", AssetOptions::image().with_avif()),
                    }

                    p { class: "mb-4",
                        "Cisco provides a variety of load distribution options so that the engineer can examine the
                        flows in the network with the idea of finding which fields have the most variety in their values: source 
                        and destination MAC, or IP address, or transport layer port numbers."
                        br {}
                        "The more variety in the values in the fields, the better the balancing effects, and the lower the chance
                        of sending disproportionate amounts of traffic over one link. "
                    }

                    GreenNote {
                        p {
                            strong { "NOTE" }
                            " The algorithm focuses on the low-order bits in the fields in the headers because the
                            low-order bits typically differ the most in real networks, while the high-order bits do not
                            differ much. By focusing on the lower-order bits, the algorithm achieves better balancing of
                            traffic over the links."
                        }
                    }
                
                }
            }
            AccordionItem {
                AccordionTrigger { {h3_heading("The Effects of the EtherChannel Load Distribution Algorithm")} }
                AccordionContent {
                    p { class: "mb-4",
                        "Figure 10-10 details a new EtherChannel that will be used in two examples to show the
                        effects of load distribution."
                        br {}
                        "The examples will focus on frames sent by switch SW1 in the figure, showing the use of the "
                        {text_command("test etherchannel load-balance", TextCommandColor::Gold)}
                        " EXEC command."
                        br {}
                        "That command asks the switch to consider some addresses or ports and answer the question: which
                        link would you use when forwarding a message with those address/port values? "
                    }

                    RedNote {
                        p {
                            strong { "NOTE" }
                            " In Packet Tracer version 8.2.2.0400."
                            br {}
                            "The "
                            {text_command("test etherchannel load-balance", TextCommandColor::Black)}
                            " priveledge command is not available!"
                        
                        }
                    }

                    img {
                        class: "mb-4 rounded-lg",
                        alt: "Figure 10-10 Four-Link EtherChannel",
                        loading: "lazy",
                        src: asset!("/assets/static/v1p3c10s2sh4f10-10.png", AssetOptions::image().with_avif()),
                    }

                    {h4_heading("src-mac test with same source mac addresses")}
                    p { class: "mb-4",
                        "Example 10-7 shows how switch SW1 distributes traffic when using "
                        {text_command("src-mac", TextCommandColor::Gold)}
                        " load distribution."
                        br {}
                        "The example lists the output from three of the "
                        {text_command("test etherchannel load-balance", TextCommandColor::Gold)}
                        " commands, but note that  all three commands use the same source MAC address."
                        br {}
                        "As a result, the answer from each command references the same interface (G1/0/22 in this case)."
                    }

                    p { class: "mb-4", "See Example 10-7 in volume 1 on page 256." }

                    p { class: "mb-1", "Example 10-7 makes two important points:" }
                    ol { class: "list-disc list-inside mb-4",
                        li {
                            "All three tests list the same outgoing physical interface because (1) the method uses only
                            the source MAC address, and (2) all three tests use the same MAC addresses."
                        }
                        li {
                            "All three tests use a different destination MAC address, with different low-order bits, but
                            that had no impact on the choice because the method—"
                            {text_command("src-mac", TextCommandColor::Gold)}
                            "—does not consider the destination MAC address."
                        }
                    }

                    {h4_heading("src-mac test with different source mac addresses")}
                    p { class: "mb-4",
                        "In contrast on that first point, Example 10-8 repeats the test commands from Example 10-7."
                        br {}
                        "The switch still uses the "
                        {text_command("src-mac", TextCommandColor::Gold)}
                        " balancing method, but now with different source MAC addresses in each test."
                        br {}
                        "Notice that the source MAC addresses used in the tests differ by just a few bit values in the 
                        low-order bits, so as a result, each test shows a different interface choice by SW1."
                    }

                    p { class: "mb-4", "See Example 10-8 in volume 1 on page 256." }

                    {h4_heading("src-dst-mac test with different source mac addresses")}
                    p { class: "mb-4",
                        "Example 10-9 shows yet a third variation, this time changing the load distribution method to "
                        {text_command("src-dst-mac", TextCommandColor::Gold)}
                        ", which means that the switch will consider both source and destination MAC."
                        br {}
                        "The example repeats the exact same "
                        {text_command("test etherchannel", TextCommandColor::Gold)}
                        " commands as Example 10-7, with the exact same MAC addresses: the source MAC addresses remain the 
                        same in all three tests, but the destination MAC addresses differ in the low-order bits."
                        br {}
                        "With the chosen destination MAC values differing slightly, switch SW1 happens to choose three different 
                        interfaces. "
                    }

                    p { class: "mb-4", "See Example 10-9 in volume 1 on page 257." }
                
                }
            }
        }

        {h3_heading("RECAP")}
        ul { class: "list-disc list-inside",
            li {
                "When using Layer 2 EtherChannels, a switch's MAC learning process associates MAC addresses with the PortChannel 
                interfaces and not the underlying physical ports."
            }
            li {
                "When forwarding frames out PortChannel interface, switches use rules to decide out which specific physical port 
                to use to forward the frame."
                " IOS documentation refers to those rules as "
                i { "EtherChannel load distribution" }
                " or "
                i { "load balancing" }
                "."
            }
            li {
                "EtherChannel load distribution makes the choice for each frame based on various numeric values found in the 
                Layer 2, 3, and 4 headers."
            }
        }
    }
}