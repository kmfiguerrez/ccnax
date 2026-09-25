use dioxus::prelude::*;

use crate::{
    components::{KeyTopic, ConfigChecklist}, utils::{TextCommandColor, h3_heading, text_command}
};

#[component]
pub fn Content() -> Element {
    rsx! {
        p { class: "mb-4",
            "The configuration of a Layer 3 switch mostly looks like the Layer 2 switching configuration shown back in 
            Parts II and III of this book, with a small bit of configuration added for the Layer 3 functions."
            br {}
            strong {
                "The Layer 3 switching function needs a virtual interface connected to each VLAN internal to the switch."
            }
            br {}
            "These "
            i { "VLAN interfaces" }
            " act like router interfaces, with an IP address and mask."
            br {}
            "The Layer 3 switch has an IP routing table, with connected routes off each of these VLAN interfaces."
            br {}
            "(These interfaces are also referred to as "
            i { "switched virtual interfaces" }
            " [SVI].)"
        }

        p { class: "mb-4",
            "To show the concept of Layer 3 switching with SVIs, the following example uses the same
            branch office with two VLANs shown in the earlier examples, but now the design will use
            Layer 3 switching in the LAN switch."
            br {}
            "Figure 17-3 shows the design changes and configuration concept for the Layer 3 switch function with a router icon 
            inside the switch, to emphasize that the switch routes the packets."
        }

        KeyTopic {}
        img {
            class: "mb-4 rounded-lg",
            alt: "Figure 17-3 Routing on VLAN Interfaces in a Layer 3 Switch",
            loading: "lazy",
            src: asset!("/assets/static/v1p3c17s2sh1f17-3.png", AssetOptions::image().with_avif()),
        }

        p { class: "mb-4",
            "Note that the figure represents the internals of the Layer 3 switch within the box in the
            middle of the figure."
            br {}
            "The branch still has two user VLANs (10 and 20), so the Layer 3 switch
            needs one VLAN interface for each VLAN."
            br {}
            "The figure shows a router icon inside the gray box to represent the Layer 3 switching function, with two 
            VLAN interfaces on the right side of that icon."
            br {}
            "In addition, the traffic still needs to get to router B1 (a physical router) to access
            the WAN, so the switch uses a third VLAN (VLAN 30 in this case) for the link to Router B1."
            br {}
            strong {
                "The physical link between the Layer 3 switch and router B1 would not be a trunk, but
                instead be an access link."
            }
        }

        {h3_heading("Layer 3 switching configurations")}
        p { class: "mb-1",
            "The following steps show how to configure Layer 3 switching using SVIs."
            br {}
            "Note that on some switches, like the 2960 and 2960-XR switches used for the examples in this book, the ability
            to route IPv4 packets must be enabled first, with a "
            {text_command("reload", TextCommandColor::Gold)}
            " of the switch required to enable the feature."
            br {}
            "The steps that occur after the reload would apply to all models of Cisco switches
            that are capable of doing Layer 3 switching."
        }
        ConfigChecklist {}
        ol { class: "mb-4",
            // Step 1
            li { class: "flex flex-col md:flex-row md:gap-x-4",
                span { class: "text-sky-500 font-semibold shrink-0", "Step 1." }
                div {
                    span { "Enable IP routing on the switch, as needed:" }
                    ol {
                        // Step A
                        li {
                            span { class: "text-sky-500 font-semibold uppercase mr-2",
                                "a."
                            }
                            "Use the "
                            {text_command("sdm prefer lanbase-routing", TextCommandColor::Gold)}
                            " command (or similar) in global configuration mode to change the switch forwarding ASIC settings to 
                            make space for IPv4 routes at the next reload of the switch."
                        }
                        // Step B
                        li {
                            span { class: "text-sky-500 font-semibold uppercase mr-2",
                                "b."
                            }
                            "Use the "
                            {text_command("reload", TextCommandColor::Gold)}
                            " EXEC command in enable mode to reload (reboot) the
                            switch to pick up the new "
                            {text_command("sdm prefer", TextCommandColor::Gold)}
                            " command setting."
                        }
                        // Step C
                        li {
                            span { class: "text-sky-500 font-semibold uppercase mr-2",
                                "c."
                            }
                            "Once reloaded, use the "
                            {text_command("ip routing", TextCommandColor::Gold)}
                            " command in global configuration mode to enable the IPv4 routing function in IOS software and to 
                            enable key commands like "
                            {text_command("show ip route", TextCommandColor::Gold)}
                            "."
                        }
                    }
                }
            
            }
            // Step 2
            li { class: "flex flex-col md:flex-row md:gap-x-4",
                span { class: "text-sky-500 font-semibold shrink-0", "Step 2." }
                div {
                    span {
                        "Configure each SVI interface, one per VLAN for which routing should be done
                        by this Layer 3 switch:"
                    }
                    ol {
                        // Step A
                        li {
                            span { class: "text-sky-500 font-semibold uppercase mr-2",
                                "a."
                            }
                            "Use the "
                            {text_command("interface vlan", TextCommandColor::Gold)}
                            i { " vlan_id" }
                            " command in global configuration mode to create a VLAN interface and to give the switch's routing 
                            logic a Layer 3 interface connected into the VLAN of the same number."
                        }
                        // Step B
                        li {
                            span { class: "text-sky-500 font-semibold uppercase mr-2",
                                "b."
                            }
                            "Use the "
                            {text_command(" ip address", TextCommandColor::Gold)}
                            i { " address mask" }
                            " command in VLAN interface configuration mode to configure an IP address and mask on the 
                            VLAN interface, enabling IPv4 routing on that VLAN interface."
                        }
                        // Step C
                        li {
                            span { class: "text-sky-500 font-semibold uppercase mr-2",
                                "c."
                            }
                            "(As needed) Use the "
                            {text_command("no shutdown", TextCommandColor::Gold)}
                            " command in interface configuration mode to enable the VLAN interface (if it is currently in a 
                            shutdown state)."
                        }
                    }
                }
            
            }
        }

        p { class: "mb-4",
            "Example 17-6 shows the configuration to match Figure 17-3. In this case, switch SW1 has already used the "
            {text_command("sdm prefer", TextCommandColor::Gold)}
            " global command to change to a setting that supports IPv4 routing, and the switch has been reloaded."
            br {}
            "The example shows the related configuration on all three VLAN interfaces."
        }

        img {
            class: "mb-4 rounded-lg",
            alt: "Example 17-6 VLAN Interface Configuration for Layer 3 Switching",
            loading: "lazy",
            src: asset!("/assets/static/v1p3c17s2sh1ex17-6.png", AssetOptions::image().with_avif()),
        }

        {h3_heading("RECAP")}
        ol { class: "list-disc list-inside",
            li {
                "The Layer 3 switching function needs a virtual interface connected to each VLAN internal to the switch."
                " These "
                i { "VLAN interfaces" }
                " act like router interfaces, with an IP address and mask."
            }
            li { "The VLAN interface is a switch's Layer 3 interface connected to the VLAN." }
            li {
                "The Layer 3 switch has an IP routing table, with connected routes off each of these VLAN interfaces 
                (also called switched virtual interfaces [SVI].)"
            }
            li {
                "The "
                {text_command("sdm prefer", TextCommandColor::Gold)}
                " command changes how the switch forwarding chips allocate memory for
                different forwarding tables, and changes to those tables require a reload of the switch."
            }
            li {
                "After configuring the "
                {text_command("interface vlan", TextCommandColor::Gold)}
                i { " vlan_id" }
                " command, the VLAN interface reaches an up/down state only if there's a configured vlan of the same number."
                " Then it reaches an up/up state only if at least there's one physical interface assigned to the vlan."
            }
        }
    }
}