use dioxus::prelude::*;

use crate::{
    components::KeyTopic, utils::{TextCommandColor, h3_heading, text_command}
};

#[component]
pub fn Content() -> Element {
    rsx! {
        p { class: "mb-4",
            "When a Layer 3 switch needs a Layer 3 interface connected to a subnet, and only one physical interface connects to 
            that subnet, the network engineer can choose to use a routed port instead of an SVI."
            br {}
            "Conversely, when the Layer 3 switch needs a Layer 3 interface connected to a subnet, and many physical interfaces on 
            the switch connect to that subnet, an SVI needs to be used."
            br {}
            "(SVIs forward traffic internally into the VLAN, so that then the Layer 2 logic can
            forward the frame out any of the ports in the VLAN. Routed ports cannot.)"
        }

        {h3_heading("Situation to use routed port")}
        p { class: "mb-4",
            "To see why, consider the design in Figure 17-4, which repeats the same design from Figure
            17-3 (used in the SVI examples)."
            br {}
            "In that design, the gray rectangle on the right represents the switch and its internals."
            br {}
            "On the right of the switch, at least two access ports sit in both VLAN 10 and VLAN 20."
            br {}
            "However, that figure shows a single link from the switch to Router B1."
            br {}
            "The switch could configure the port as an access port in a separate VLAN, as shown with VLAN 30 in 
            Examples 17-6 and 17-7."
            br {}
            "However, with only one switch port needed, the switch could configure that link as a routed port, 
            as shown in the figure."
        }

        KeyTopic {}
        img {
            class: "mb-4 rounded-lg",
            alt: "Figure 17-4 Routing on a Routed Interface on a Switch",
            loading: "lazy",
            src: asset!("/assets/static/v1p3c17s3sh1f17-4.png", AssetOptions::image().with_avif()),
        }

        {h3_heading("Enabling a routed interface")}
        p { class: "mb-4",
            "Enabling a switch interface to be a routed interface instead of a switched interface is simple:
            just use the "
            {text_command("no switchport", TextCommandColor::Gold)}
            " subcommand on the physical interface."
            br {}
            "Cisco switches capable of being a Layer 3 switch use a default of the "
            {text_command("switchport", TextCommandColor::Gold)}
            " command to each switch physical interface."
        }

        {h3_heading("The term switchport")}
        p { class: "mb-4",
            "Think about the word "
            i { "switchport" }
            " for a moment."
            br {}
            "With that term, Cisco tells the layer 3 switch to treat the port like it is a port on a switch—that is, a Layer 2 
            port on a switch."
            br {}
            "To make the port stop acting like a switch port and instead act like a router port, use the "
            {text_command("no switchport", TextCommandColor::Gold)}
            " command  on the interface."
        }

        {h3_heading("Routed port as router interface")}
        p { class: "mb-4",
            strong { "Once the port is acting as a routed port, think of it like a router interface." }
            br {}
            "That is, configure the IP address on the physical port, as implied in Figure 17-4."
            br {}
            "Example 17-10 shows a completed configuration for the interfaces configured on the switch in Figure 17-4."
            br {}
            "Note that the design uses the exact same IP subnets as the example that showed SVI configuration
            in Example 17-6, but now, the port connected to subnet 10.1.30.0 has been converted to a
            routed port."
            br {}
            "All you have to do is add the "
            {text_command("no switchport", TextCommandColor::Gold)}
            " command to the physical interface and configure the IP address on 
            the physical interface."
        }

        img {
            class: "mb-4 rounded-lg",
            alt: "Example 17-10 Configuring Interface G0/1 on Switch SW1 as a Routed Port",
            loading: "lazy",
            src: asset!("/assets/static/v1p3c17s3sh1ex17-10.png", AssetOptions::image().with_avif()),
        }

        KeyTopic {}
        p { class: "mb-1",
            strong {
                "Once configured, the routed interface will show up differently in command output in the switch."
            }
            br {}
            "In particular, for an interface configured as a routed port with an IP address, like
            interface GigabitEthernet0/1 in the previous example:"
        }
        ul { class: "list-disc list-inside mb-4",
            li {
                {text_command("show interfaces", TextCommandColor::Gold)}
                ": Similar to the same command on a router, the output will display the IP
                address of the interface. (Conversely, for switch ports, this command does not list an IP address."
            }
            li {
                {text_command("show interfaces status", TextCommandColor::Gold)}
                ": Under the “VLAN” heading, instead of listing the access VLAN or the word"
                i { " trunk" }
                ", the output lists the word routed, meaning that it is a routed port."
            }
            li {
                {text_command("show ip route", TextCommandColor::Gold)}
                ": Lists the routed port as an outgoing interface in routes."
            }
            li {
                {text_command("show interfaces", TextCommandColor::Gold)}
                i { " type number " }
                {text_command("switchport", TextCommandColor::Gold)}
                ": If a routed port, the output is short and confirms that the port is not a switch port. 
                (If the port is a Layer 2 port, this command lists many configuration and status details.)"
            }
        }

        p { class: "mb-4",
            "Example 17-11 shows samples of all four of these commands as taken from the switch as
            configured in Example 17-10."
        }

        p { class: "mb-4", "See Example 17-11 in volume 1 on page 409." }

        {h3_heading("Routed ports vs SVIs")}
        p { class: "mb-4", "So, with two options—SVI and routed ports—where should you use each?" }

        p { class: "pl-4 mb-4",
            strong {
                "For any topologies with a point-to-point link between two devices that do routing, a
                routed interface works well."
            }
        }

        p { class: "mb-4",
            "Figure 17-5 shows an example of where to use SVIs and where to use routed ports in a typical 
            core/distribution/access design."
            br {}
            "In this design, the core (Core1, Core2) and distribution (D11 through D22) switches perform Layer 3 switching."
            br {}
            "All the ports that are links directly between the Layer 3 switches can be routed interfaces."
            br {}
            "For VLANs for which many interfaces (access and trunk) connect to the VLAN, SVIs make sense because the SVIs can 
            send and receive traffic out multiple ports on the same switch."
            br {}
            "In this design, all the ports on Core1 and Core2 will be routed ports, while the four distribution switches will 
            use some routed ports and some SVIs."
        }

        img {
            class: "mb-4 rounded-lg",
            alt: "Figure 17-5 Using Routed Interfaces for Core and Distribution Layer 3 Links",
            loading: "lazy",
            src: asset!("/assets/static/v1p3c17s3sh1f17-5.png", AssetOptions::image().with_avif()),
        }

        {h3_heading("RECAP")}
        ol { class: "list-disc list-inside",
            li {
                "When a Layer 3 switch needs a Layer 3 interface connected to a subnet, and only one physical interface connects 
                to that subnet, the network engineer can choose to use a routed port instead of an SVI."
            }
            li {
                "Conversely, when the Layer 3 switch needs a Layer 3 interface connected to a subnet, and many physical interfaces 
                on the switch connect to that subnet, an SVI needs to be used."
            }
            li {
                "SVIs forward traffic internally into the VLAN, so that then the Layer 2 logic can forward the frame out any of 
                the ports in the VLAN. Routed ports cannot."
            }
            li { "Once the port is acting as a routed port, think of it like a router interface." }
            li {
                "Once configured, the routed interface will show up differently in command output in the switch."
            }
            li {
                "For any topologies with a point-to-point link between two devices that do routing, a routed interface works well."
            }
        
        }
    }
}