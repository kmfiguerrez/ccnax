use dioxus::prelude::*;

use crate::{
    components::{KeyTopic, GreenNote},
    utils::h3_heading
};

#[component]
pub fn Content() -> Element {
    rsx! {
        p { class: "mb-4",
            "Through its distribution system ports, a controller can connect to multiple VLANs on the switched network."
            br {}
            "Internally, the controller must somehow map those wired VLANs to equivalent logical wireless networks."
            br {}
            "For example, suppose that VLAN 10 is set aside for wireless users in the Engineering division of a company."
            br {}
            "That VLAN must be connected to a unique wireless LAN that exists on a controller and its associated APs."
            br {}
            "The wireless LAN must then be extended to every client that associates with the Service Set Identifier (SSID)
            “Engineering.”"
        }

        p { class: "mb-4",
            "Cisco wireless controllers provide the necessary connectivity through internal logical interfaces, which must be 
            configured with an IP address, subnet mask, default gateway, and a Dynamic Host Configuration Protocol (DHCP) server."
            br {}
            "Each interface is then assigned to a physical port and a VLAN ID."
            br {}
            strong { "You can think of an interface as a Layer 3 termination on a VLAN." }
        }

        p { class: "mb-1",
            "Cisco controllers support the following interface types, also shown in Figure 29-6."
        }
        ol { class: "list-disc list-inside mb-4",
            li {
                span { class: "font-bold mr-1", "Management interface:" }
                "Used for normal management traffic, such as RADIUS user authentication, WLC-to-WLC communication, web-based and 
                SSH sessions, SNMP, Network Time Protocol (NTP), syslog, and so on. The management interface is also used
                to terminate CAPWAP tunnels between the controller and its APs."
            }
            li {
                span { class: "font-bold mr-1", "Redundancy management:" }
                "The management IP address of a redundant WLC that is part of a high availability pair of controllers. 
                The active WLC uses the management interface address, while the standby WLC uses the redundancy management address."
            }
            li {
                span { class: "font-bold mr-1", "Virtual interface:" }
                "IP address facing wireless clients when the controller is relaying client DHCP requests, performing client web 
                authentication, and supporting client mobility"
            }
            li {
                span { class: "font-bold mr-1", "Service port interface:" }
                "Bound to the service port and used for out-of-band management."
            }
            li {
                span { class: "font-bold mr-1", "Dynamic interface:" }
                "Used to connect a VLAN to a WLAN."
            }
        }

        KeyTopic {}
        img {
            class: "mb-4 rounded-lg",
            alt: "Figure 29-6 Cisco Wireless LAN Controller Interfaces",
            loading: "lazy",
            src: asset!("/assets/static/v1p8c29s3sh2f29-6.png", AssetOptions::image().with_avif()),
        }

        {h3_heading("Management Interface")}
        p { class: "mb-4",
            "The management interface faces the switched network, where management users and APs are located."
            br {}
            "Management traffic will usually consist of protocols like HTTPS, SSH, SNMP, NTP, TFTP, and so on."
            br {}
            "In addition, management interface traffic consists of CAPWAP packets that carry control and data tunnels to and from 
            the APs."
        }

        {h3_heading("Virtual Interface")}
        p { class: "mb-4",
            "The virtual interface is used only for certain client-facing operations."
            br {}
            "For example, when a wireless client issues a request to obtain an IP address, the controller can relay the request
            on to an actual DHCP server that can provide the appropriate IP address."
            br {}
            "From the client's perspective, the DHCP server appears to be the controller's virtual interface address."
            br {}
            "Clients may see the virtual interface's address, but that address is never used when the controller
            communicates with other devices on the switched network."
        }

        p { class: "mb-4",
            "Because the virtual interface is used only for some client management functions, you should configure it with a 
            unique, nonroutable address."
            br {}
            "For example, you might use 10.1.1.1 because it is within a private address space defined in RFC 1918."
        }

        GreenNote {
            p {
                strong { "NOTE" }
                " Traditionally, many people have assigned IP address 1.1.1.1 to the virtual interface.
                Although it is a unique address, it is routable and already in use elsewhere on the Internet.
                A better practice is to use an IP address from the RFC 1918 private address space that is
                unused or reserved, such as 192.168.1.1. You could also use a reserved address from RFC
                5737 (192.0.2.0/24) that is set aside for documentation purposes and is never used."
            }
        }

        p { class: "mb-4",
            "The virtual interface address is also used to support client mobility."
            br {}
            "For that reason, every controller that exists in the same mobility group should be configured with a virtual address
            that is identical to the others."
            br {}
            "By using one common virtual address, all the controllers will appear to operate as a cluster as clients roam from 
            controller to controller."
        }

        {h3_heading("Dynamic Interface")}
        p { class: "mb-4",
            strong { "Dynamic interfaces map WLANs to VLANs" }
            ", making the logical connections between wireless and wired networks."
            br {}
            "You will configure one dynamic interface for each wireless LAN that is offered by the controller's APs and then map 
            the interface to the WLAN."
            br {}
            strong {
                "Each dynamic interface must also be configured with its own IP address and can act as a DHCP
                relay for wireless clients."
            }
            br {}
            "To filter traffic passing through a dynamic interface, you can configure an optional access list. "
        }

        {h3_heading("RECAP")}
        ol { class: "list-disc list-inside",
            li { "Management interface is used for management and CAPWAP traffic." }
            li {
                "The virtual interface is used only for certain client-facing operations and should be used with private 
                addresses only."
            }
            li {
                "Dynamic interfaces map WLANs to VLANs, and each dynamic interface must also be configured with its own IP address 
                and can act as a DHCP relay for wireless clients."
            }
        }
    }
}