use dioxus::prelude::*;

use crate::{
    components::{KeyTopic, GreenNote, ConfigChecklist}, utils::{TextCommandColor, h3_heading, text_command}
};

#[component]
pub fn Content() -> Element {
    rsx! {
        p { class: "mb-4",
            strong {
                "So far, this section has stated that routed interfaces can be used with a single point-to-point
                link between pairs of Layer 3 switches, or between a Layer 3 switch and a router."
            }
            br {}
            "However, in most designs, the network engineers use at least two links between each pair of distribution and core 
            switches, as shown in Figure 17-6."
        }

        img {
            class: "mb-4 rounded-lg",
            alt: "Figure 17-6 Two Links Between Each Distribution and Core Switch",
            loading: "lazy",
            src: asset!("/assets/static/v1p3c17s3sh2f17-6.png", AssetOptions::image().with_avif()),
        }

        {h3_heading("Reasons to use layer 3 EtherChannel")}
        p { class: "mb-4",
            strong {
                "While each individual port in the distribution and core could be treated as a separate routed
                port, it is better to combine each pair of parallel links into a Layer 3 EtherChannel."
            }
            br {}
            "Without using EtherChannel, you can still make each port on each switch in the center of the figure be a routed port."
            " It works."
            br {}
            strong {
                "However, once you enable a routing protocol but don't use EtherChannels, each Layer 3 switch will now learn 
                two IP routes with the same neighboring switch as the next hop—one route over one link, another route over the other 
                link."
            }
        }

        p { class: "mb-4",
            "Using a Layer 3 EtherChannel makes more sense with multiple parallel links between two switches."
            br {}
            "By doing so, each pair of links acts as one Layer 3 link."
            br {}
            "So, each pair of switches has one routing protocol neighbor relationship with the neighbor, and not two."
            br {}
            "Each switch learns one route per destination per pair of links, and not two."
            br {}
            strong {
                "IOS then balances the traffic, often with better balancing than the balancing that occurs with the use of multiple 
                IP routes to the same subnet."
            }
            br {}
            "Overall, the Layer 3 EtherChannel approach works much better than leaving each link as a separate routed port 
            and using Layer 3 balancing."
        }

        {h3_heading("Layer 3 EtherChannel configurations")}
        p { class: "mb-1",
            "Compared to what you have already learned, configuring a Layer 3 EtherChannel takes only a little more work."
            br {}
            "Chapter 10 already showed you how to configure an EtherChannel."
            br {}
            "This chapter has already shown how to make a port a Layer 3 routed port."
            br {}
            "Next, you have to combine the two ideas by combining both the EtherChannel and routed port configuration.
            The following checklist shows the steps, assuming a static definition."
        }
        ConfigChecklist {}
        ol { class: "mb-4",
            // Step 1
            li { class: "flex flex-col md:flex-row md:gap-x-4",
                span { class: "text-sky-500 font-semibold shrink-0", "Step 1." }
                div {
                    span {
                        "Configure the physical interfaces as follows, in interface configuration mode:"
                    }
                    ol {
                        // Step A
                        li {
                            span { class: "text-sky-500 font-semibold uppercase mr-2",
                                "a."
                            }
                            "Add the "
                            {text_command("channel-group", TextCommandColor::Gold)}
                            i { " number " }
                            {text_command("mode on", TextCommandColor::Gold)}
                            " command to add it to the channel. Use the same number for all physical interfaces on the same switch, 
                            but "
                            strong {
                                "the number used (the channel-group number) can differ on the two neighboring switches"
                            }
                            "."
                        }
                        // Step B
                        li {
                            span { class: "text-sky-500 font-semibold uppercase mr-2",
                                "b."
                            }
                            "Add the "
                            {text_command("no switchport", TextCommandColor::Gold)}
                            " command to make each physical port a routed port."
                        }
                    }
                }
            
            }
            // Step 2
            li { class: "flex flex-col md:flex-row md:gap-x-4",
                span { class: "text-sky-500 font-semibold shrink-0", "Step 2." }
                div {
                    span { "Configure the PortChannel interface:" }
                    ol {
                        // Step A
                        li {
                            span { class: "text-sky-500 font-semibold uppercase mr-2",
                                "a."
                            }
                            "Use the "
                            {text_command("interface port-channel", TextCommandColor::Gold)}
                            i { " number " }
                            "command to move to port-channel configuration mode for the same channel number configured on the 
                            physical interfaces."
                        }
                        // Step B
                        li {
                            span { class: "text-sky-500 font-semibold uppercase mr-2",
                                "b."
                            }
                            "Add the "
                            {text_command("no switchport", TextCommandColor::Gold)}
                            " command to make sure that the port-channel interface acts as a routed port. 
                            (IOS may have already added this command.)"
                        }
                        // Step C
                        li {
                            span { class: "text-sky-500 font-semibold uppercase mr-2",
                                "c."
                            }
                            "Use the "
                            {text_command("ip address", TextCommandColor::Gold)}
                            i { " address mask " }
                            "command to configure the address and mask."
                        }
                    }
                }
            
            }
        }

        GreenNote {
            p {
                strong { "NOTE" }
                " Cisco uses the term EtherChannel in concepts discussed in this section and then
                uses the term "
                i { "PortChannel" }
                ", with command keyword "
                {text_command("port-channel", TextCommandColor::Black)}
                ", when verifying and configuring EtherChannels."
                " For the purposes of understanding the technology, you may treat these terms as synonyms."
                " However, it helps to pay close attention to the use of the terms "
                i { "PortChannel" }
                " and "
                i { "EtherChannel" }
                " as you work through the examples in this section because IOS uses both."
            }
        }

        KeyTopic {}
        img {
            class: "mb-4 rounded-lg",
            alt: "Figure 17-7 Design Used in EtherChannel Configuration Examples",
            loading: "lazy",
            src: asset!("/assets/static/v1p3c17s3sh2f17-7.png", AssetOptions::image().with_avif()),
        }

        img {
            class: "mb-4 rounded-lg",
            alt: "Example 17-12 Layer 3 EtherChannel Configuration on Switch SW1",
            loading: "lazy",
            src: asset!("/assets/static/v1p3c17s3sh2ex17-12.png", AssetOptions::image().with_avif()),
        }

        p { class: "mb-4",
            strong {
                "Of particular importance, note that although the physical interfaces and PortChannel interface
                are all routed ports, the IP address should be placed on the PortChannel interface only."
            }
            br {}
            "In fact, when the "
            {text_command("no switchport", TextCommandColor::Gold)}
            " command is configured on an interface, IOS adds the "
            {text_command("no ip address", TextCommandColor::Gold)}
            " command to the interface."
            br {}
            "Then configure the IP address on the PortChannel interface only."
        }

        {h3_heading("Routed ports in the show commands")}
        p { class: "mb-4",
            "Once configured, the PortChannel interface appears in several commands, as shown in Example 17-13."
            br {}
            "The commands that list IP addresses and routes refer to the PortChannel interface."
            br {}
            "Also, note that the "
            {text_command("show interfaces status", TextCommandColor::Gold)}
            " command lists the fact that the physical ports and the port-channel 12 interface are all routed ports."
        }

        p { class: "mb-4", "See Example 17-13 in volume 1 on page 412." }

        p { class: "mb-4",
            "For a final bit of verification, you can examine the EtherChannel directly with the "
            {text_command("show etherchannel summary", TextCommandColor::Gold)}
            " command as listed in Example 17-14."
            br {}
            "Note in particular that it lists a flag legend for characters that identify key operational states, such as whether 
            a port is bundled (included) in the PortChannel (P) and whether it is acting as a routed (R) or switched
            (S) port."
        }

        p { class: "mb-4", "See Example 17-14 in volume 1 on page 413." }

        {h3_heading("RECAP")}
        ol { class: "list-disc list-inside",
            li {
                "In most designs, the network engineers use at least two links between each pair of distribution and core switches."
            }
            li {
                "While each individual port in the distribution and core could be treated as a separate routed port, it is better 
                to combine each pair of parallel links into a Layer 3 EtherChannel."
            }
            li {
                "Without using layer 3 EtherChannel, layer 3 switches that do routing, with parallel links, will learn multiple IP 
                routes with the same neighboring switch as the next hop—one route over one link, another route over the other link."
            }
            li {
                "With Layer 3 EtherChannel, IOS then balances the traffic, often with better balancing than the balancing that 
                occurs with the use of multiple IP routes to the same subnet."
            }
            li {
                "Note that although the physical interfaces and PortChannel interface
                are all routed ports, the IP address should be placed on the PortChannel interface only."
            }
            li {
                "In fact, when the "
                {text_command("no switchport", TextCommandColor::Gold)}
                " command is configured on an interface, IOS adds the "
                {text_command("no ip address", TextCommandColor::Gold)}
                " command to the interface."
            }
            li {
                "Note that layer 3 EtherChannel configurations can be both static and dynamic. if static, using the "
                {text_command("channel-group", TextCommandColor::Gold)}
                i { " number " }
                {text_command("mode on", TextCommandColor::Gold)}
                " interface subcommand."
                " if dynamic, using the protocols used to create layer 2 EtherChannel: LACP and PaGp."
            
            }
        }
    }
}