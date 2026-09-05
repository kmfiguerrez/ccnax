use dioxus::prelude::*;

use crate::{
    components::GreenNote, 
    utils::h3_heading
};

#[component]
pub fn Content() -> Element {
    rsx! {
        p { class: "mb-4",
            "A switch needs the same kind of IP settings as a PC with a single Ethernet interface."
            br {}
            "For perspective, a PC has a CPU, with the operating system running on the CPU."
            br {}
            "It has an Ethernet network interface card (NIC)."
            br {}
            "The OS configuration includes an IP address associated with the NIC, either configured or learned 
            dynamically with DHCP"
        }

        {h3_heading("Virtual NIC")}
        p { class: "mb-4",
            "A switch uses the same ideas, except that the switch needs to use a virtual NIC inside the
            switch."
            br {}
            "Like a PC, a switch has a real CPU, running an OS (called IOS)."
            br {}
            "The switch obviously has lots of Ethernet ports, but instead of assigning its management IP address to any of
            those ports, the switch then uses a NIC-like concept called a switched virtual interface (SVI),
            or more commonly, a VLAN interface, that acts like the switch's own NIC."
            br {}
            "Then the settings on the switch look something like a host, with the switch configuration assigning IP settings,
            like an IP address, to this VLAN interface, as shown in Figure 6-6"
        }

        img {
            class: "mb-4 rounded-lg",
            alt: "Figure 6-6 Switch Virtual Interface (SVI) Concept Inside a Switch",
            loading: "lazy",
            src: asset!("/assets/static/v1p2c6s2sh1f6-6.png", AssetOptions::image().with_avif()),
        }

        p { class: "mb-4",
            "By using interface VLAN 1 for the IP configuration, the switch can then send and receive
            frames on any of the ports in VLAN 1."
            br {}
            "In a Cisco switch, by default, all ports are assigned to VLAN 1."
        }

        p { class: "mb-4",
            "In most networks, switches configure many VLANs, so the network engineer has a choice of
            where to configure the IP address."
            br {}
            "That is, the management IP address does not have to be configured on the VLAN 1 interface 
            (as configured with the interface vlan 1 command seen in Figure 6-6)."
        }

        {h3_heading("One IP address and one VLAN only for management purposes")}
        p { class: "mb-4",
            "A Layer 2 Cisco LAN switch needs only one IP address for management purposes."
            br {}
            "However, you can choose to use any VLAN to which the switch connects."
            br {}
            "The configuration then includes a VLAN interface for that VLAN number, with an appropriate IP address."
        }

        p {
            "For example, Figure 6-7 shows a Layer 2 switch with some physical ports in two different
            VLANs (VLANs 1 and 2)."
            br {}
            "The figure also shows the subnets used on those VLANs."
            br {}
            "The network engineer could choose to use either"
        }
        ul { class: "list-disc list-inside mb-4",
            li { "Interface VLAN 1, with an IP address in subnet 192.168.1.0" }
            li { "Interface VLAN 2, with an IP address in subnet 192.168.2.0" }
        }

        img {
            class: "mb-4 rounded-lg",
            alt: "Figure 6-7 Choosing One VLAN on Which to Configure a Switch IP Address",
            loading: "lazy",
            src: asset!("/assets/static/v1p2c6s2sh1f6-7.png", AssetOptions::image().with_avif()),
        }

        p { class: "mb-4",
            "Note that you should not try to use a VLAN interface for which there are no physical ports
            assigned to the same VLAN."
            br {}
            "If you do, the VLAN interface will not reach an up/up state, and the switch will not have the physical 
            ability to communicate outside the switch."
        }

        GreenNote {
            p {
                strong { "NOTE" }
                " Some Cisco switches can be configured to act as either a Layer 2 switch or a Layer
                3 switch. When acting as a Layer 2 switch, a switch forwards Ethernet frames as discussed
                in depth in Chapter 5, “Analyzing Ethernet LAN Switching.” Alternatively, a switch can also
                act as a multilayer switch or Layer 3 switch, which means the switch can do both Layer
                2 switching and Layer 3 IP routing of IP packets, using the Layer 3 logic normally used by
                routers. This chapter assumes all switches are Layer 2 switches. Chapter 17, “IP Routing in
                the LAN,” discusses Layer 3 switching in depth along with using multiple VLAN interfaces
                at the same time."
            }
        }

        {h3_heading("Switches also need a default gateway")}
        p {
            "Configuring the IP address (and mask) on one VLAN interface allows the switch to send and
            receive IP packets with other hosts in a subnet that exists on that VLAN; however, the switch
            cannot communicate outside the local subnet without another configuration setting called
            the default gateway."
            br {}
            "The reason a switch needs a default gateway setting is the same reason that hosts need the same 
            setting—because of how hosts think when sending IP packets."
            br {}
            "Specifically:"
        }
        ol { class: "list-disc list-inside mb-4",
            li { "To send IP packets to hosts in the same subnet, send them directly" }
            li {
                "To send IP packets to hosts in a different subnet, send them to the local router; that is, the
                default gateway"
            }
        }

        p { class: "mb-4",
            "Figure 6-8 shows the ideas. In this case, the switch (on the right) will use IP address
            192.168.1.200 as configured on interface VLAN 1."
            br {}
            "However, to communicate with host A, on the far left of the figure, the switch must use Router R1 
            (the default gateway) to forward IP packets to host A."
            br {}
            "To make that work, the switch needs to configure a default gateway setting, pointing to Router R1's 
            IP address (192.168.1.1 in this case)."
            br {}
            "Note that the switch and router both use the same mask, 255.255.255.0, which puts the addresses in the same subnet."
        }

        img {
            class: "mb-4 rounded-lg",
            alt: "Figure 6-8 The Need for a Default Gateway",
            loading: "lazy",
            src: asset!("/assets/static/v1p2c6s2sh1f6-8.png", AssetOptions::image().with_avif()),
        }

        {h3_heading("RECAP")}
        ol { class: "list-disc list-inside",
            li {
                "To allow Telnet or SSH access to the switch, and to allow other IP-based management protocols, 
                the switch needs an IP address, as well as a few other related settings."
            }
            li {
                "The IP address has nothing to do with how switches forward Ethernet frames; it simply exists to support 
                overhead management traffic."
            }
            li {
                "Switches use a NIC-like concept called a switched virtual interface (SVI), or more commonly, a VLAN interface, 
                that acts like the switch's own host NIC."
            }
            li { "In a Cisco switch, by default, all ports are assigned to VLAN 1." }
            li { "A Layer 2 Cisco LAN switch needs only one IP address for management purposes." }
            li {
                "You should not try to use a VLAN interface for which there are no physical ports
                assigned to the same VLAN."
            }
            li {
                "To communicate outsite the local network, switches also need a default gateway setting."
            }
        }
    }
}