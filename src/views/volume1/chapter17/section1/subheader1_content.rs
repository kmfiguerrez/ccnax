use dioxus::prelude::*;

use crate::{
    components::{KeyTopic, GreenNote, ConfigChecklist}, utils::{TextCommandColor, h3_heading, text_command}
};

#[component]
pub fn Content() -> Element {
    rsx! {
        p { class: "mb-4",
            strong {
                "This next topic discusses how routers route packets to subnets associated with VLANs connected to a 
                router 802.1Q trunk."
            }
            br {}
            "That long description can be a bit of a chore to repeat each time someone wants to discuss this feature, so over 
            time, the networking world has instead settled on a shorter and more interesting name for this 
            feature: router-on-a-stick (ROAS)."
        }

        p { class: "mb-4",
            strong {
                "ROAS uses router VLAN trunking configuration to give the router a logical router interface connected to each VLAN."
            }
            br {}
            "Because the router then has an interface connected to each VLAN, the router can also be configured with an 
            IP address in the subnet that exists on each VLAN."
        }

        {h3_heading("Routers subinterfaces")}
        p { class: "mb-4",
            strong { "Routers use subinterfaces as the means to have an interface connected to a VLAN." }
            br {}
            "The router needs to have an IP address/mask associated with each VLAN on the trunk."
            br {}
            "However, the router has only one physical interface for the link connected to the trunk."
            br {}
            strong {
                "Cisco solves this problem by creating multiple virtual router interfaces, one associated with each VLAN
                on that trunk (at least for each VLAN that you want the trunk to support)."
            }
            br {}
            "Cisco calls these virtual interfaces "
            i { "subinterfaces" }
            "."
            br {}
            "The configuration can then include an ip address command for each subinterface."
        }

        p { class: "mb-4",
            "Figure 17-2 shows the concept with Router B1, one of the branch routers from Figure 17-1."
            br {}
            "Because this router needs to route between only two VLANs, "
            strong {
                "the figure also shows two subinterfaces, 
                named G0/0.10 and G0/0.20, which create a new place in the configuration where
                the per-VLAN configuration settings can be made"
            }
            "."
            br {}
            "The router treats frames tagged with VLAN 10 as if they came in or out of G0/0.10 and frames tagged with 
            VLAN 20 as if they came in or out G0/0.20."
        }

        KeyTopic {}
        img {
            class: "mb-4 rounded-lg",
            alt: "Figure 17-2 Subinterfaces on Router B1",
            loading: "lazy",
            src: asset!("/assets/static/v1p3c17s1sh1f17-2.png", AssetOptions::image().with_avif()),
        }

        p { class: "mb-4",
            "In addition, "
            strong {
                "note that most Cisco routers do not attempt to negotiate trunking, so both the
                router and switch need to manually configure trunking"
            }
            "."
            br {}
            "This chapter discusses the router side of that trunking configuration; the matching switch interface would need to be 
            configured with the "
            {text_command("switchport mode trunk", TextCommandColor::Gold)}
            " command."
        }

        ConfigChecklist {}
        p { class: "mb-1",
            "Example 17-1 shows a full example of the 802.1Q trunking configuration required on Router B1 in Figure 17-2."
            br {}
            "More generally, these steps detail how to configure 802.1Q trunking on a router:"
        }
        ol { class: "pl-1 mb-4",
            li {
                span { class: "text-blue-500 font-bold mr-4", "Step 1." }
                " Use the "
                {text_command("interface", TextCommandColor::Gold)}
                i { " type number.subint" }
                " command in global configuration mode
                to create a unique subinterface for each VLAN that needs to be routed."
            }
            li {
                span { class: "text-blue-500 font-bold mr-4", "Step 2." }
                " Use the "
                {text_command("encapsulation dot1q", TextCommandColor::Gold)}
                i { " vlan_id" }
                " command in subinterface configuration mode to enable 802.1Q and associate one specific VLAN with 
                the subinterface."
            }
            li {
                span { class: "text-blue-500 font-bold mr-4", "Step 2." }
                " Use the "
                {text_command("ip address", TextCommandColor::Gold)}
                i { " address mask" }
                " command in subinterface configuration mode to configure IP settings (address and mask)."
            }
        }

        img {
            class: "mb-4 rounded-lg",
            alt: "Example 17-1 Router Configuration for the 802.1Q Encapsulation Shown in Figure 17-2",
            loading: "lazy",
            src: asset!("/assets/static/v1p3c17s1sh1ex17-1.png", AssetOptions::image().with_avif()),
        }

        {h3_heading("The encapsulation subcommand")}
        p { class: "mb-4",
            "First, look at the subinterface numbers. The subinterface number begins with the period, like .10 and .20 in this case."
            br {}
            "These numbers can be any number from 1 up through a very large number (over 4 billion)."
            br {}
            "The number just needs to be unique among all subinterfaces associated with this one physical interface."
            br {}
            strong { "In fact, the subinterface number does not even have to match the associated VLAN ID." }
            br {}
            "(The "
            {text_command("encapsulation", TextCommandColor::Gold)}
            " command, and not the subinterface number, defines the VLAN ID associated with the subinterface."
        }

        GreenNote {
            p {
                strong { "NOTE" }
                " Although not required, most sites do choose to make the subinterface number
                match the VLAN ID, as shown in Example 17-1, just to avoid confusion."
            }
        }

        p { class: "mb-4",
            "Each subinterface configuration lists two subcommands."
            br {}
            "One command ("
            {text_command("encapsulation", TextCommandColor::Gold)}
            ") enables trunking and defines  the VLAN whose frames are considered to be coming in and out 
            of the subinterface."
            br {}
            "The "
            {text_command("ip address", TextCommandColor::Gold)}
            " command works the same way it does on any other interface."
            br {}
            strong {
                "Note that if the physical Ethernet interface reaches an up/up state, the subinterface should as well, which would 
                then let the router add the connected routes shown at the bottom of the example."
            }
        }

        p { class: "mb-4",
            "Now that the router has a working interface, with IPv4 addresses configured, the router can
            route IPv4 packets on these subinterfaces."
            br {}
            "That is, "
            strong {
                "the router treats these subinterfaces like  any physical interface in terms of adding connected routes, 
                matching those routes, and forwarding packets to/from those connected subnets"
            }
            "."
        }

        {h3_heading("The Native VLAN")}
        p { class: "mb-1",
            "The configuration and use of the native VLAN on the trunk require a little extra thought."
            br {}
            strong {
                "The native VLAN can be configured on a subinterface, or on the physical interface, or ignored"
            }
            " as in Example 17-1."
            br {}
            strong {
                "Each 802.1Q trunk has one native VLAN, and if the router needs to route packets for a subnet that exists in the 
                native VLAN, then the router needs some configuration to support that subnet."
            }
            br {}
            "The two options to define a router interface for the native VLAN are"
        }
        KeyTopic {}
        ul { class: "list-disc list-inside mb-4",
            li {
                "Configure the "
                {text_command("ip address", TextCommandColor::Gold)}
                " command on the physical interface, but without an "
                {text_command("encapsulation", TextCommandColor::Gold)}
                " command; the router considers this physical interface to be using the native VLAN."
            }
            li {
                "Configure the "
                {text_command("ip address", TextCommandColor::Gold)}
                " command on a subinterface and use the "
                {text_command("encapsulation dot1q", TextCommandColor::Gold)}
                i { " vlan-id " }
                {text_command("native", TextCommandColor::Gold)}
                " subcommand to tell the router both the VLAN ID and the fact that it is the native VLAN."
            }
        }

        p { class: "mb-4",
            "Example 17-2 shows both native VLAN configuration options with a small change to the
            same configuration in Example 17-1."
            br {}
            "In this case, VLAN 10 becomes the native VLAN."
            br {}
            "The top part of the example shows the option to configure the router physical interface to use native VLAN 10."
            br {}
            "The second half of the example shows how to configure that same native VLAN on a subinterface."
            br {}
            "In both cases, the switch configuration also needs to be changed to make VLAN 10 the native VLAN."
        }

        p { class: "mb-4", "See Example 17-2 on volume 1 on page 398." }

        {h3_heading("RECAP")}
        ol { class: "list-disc list-inside",
            li {
                "Routers routing packets to subnets associated with VLANs connected to a router 802.1Q trunk is known as 
                router-on-a-stick (ROAS)."
            }
            li {
                "ROAS uses router VLAN trunking configuration to give the router a logical router interface (virtual interface) 
                connected to each VLAN."
            
            }
            li {
                "Cisco calls these virtual interfaces "
                i { "subinterfaces" }
                " and used by routers as the means to have an interface connected to a VLAN."
            }
            li {
                "Note that most Cisco routers do not attempt to negotiate trunking, so both the router and switch need to 
                manually configure trunking."
            }
            li { "The subinterface number does not even have to match the associated VLAN ID." }
            li {
                "The "
                {text_command("encapsulation", TextCommandColor::Gold)}
                " command, and not the subinterface number, defines the VLAN ID associated with the subinterface 
                and enables trunking."
            }
            li {
                "Note that if the physical Ethernet interface reaches an up/up state, the subinterface should as well, which would 
                then let the router add the connected routes."
            }
            li {
                "Routers treat subinterfaces like any physical interface in terms of adding connected routes, matching those 
                routes, and forwarding packets to/from those connected subnets."
            }
            li {
                "The native VLAN can be configured on a subinterface, or on the physical interface, or ignored."
            }
            li {
                "Each 802.1Q trunk has one native VLAN, and if the router needs to route packets for a subnet that exists in the 
                native VLAN, then the router needs some configuration to support that subnet."
            }
        }
    }
}