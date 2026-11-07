use dioxus::prelude::*;

use crate::{
    components::{KeyTopic, GreenNote},
    utils::h3_heading
};

#[component]
pub fn Content() -> Element {
    rsx! {
        p { class: "mb-1",
            "You can connect several different types of controller ports to your network, as shown in
            Figure 29-5 and discussed in the following list:"
        }
        ol { class: "list-disc list-inside mb-4",
            li {
                span { class: "font-bold mr-1", "Service port:" }
                "Used for out-of-band management, system recovery, and initial boot functions; always connects to a switch port in 
                access mode"
            }
            li {
                span { class: "font-bold mr-1", "Distribution system port:" }
                "Used for all normal AP and management traffic; usually connects to a switch port in 802.1Q trunk mode"
            }
            li {
                span { class: "font-bold mr-1", "Console port:" }
                "Used for out-of-band management, system recovery, and initial boot functions; asynchronous connection to a 
                terminal emulator (9600 baud, 8 data bits, 1 stop bit, by default)"
            }
            li {
                span { class: "font-bold mr-1", "Redundancy port:" }
                "Used to connect to a peer controller for high availability (HA) operation"
            }
        }

        KeyTopic {}
        img {
            class: "mb-4 rounded-lg",
            alt: "Figure 29-5 Cisco Wireless LAN Controller Ports",
            loading: "lazy",
            src: asset!("/assets/static/v1p8c29s3sh1f29-5.png", AssetOptions::image().with_avif()),
        }

        {h3_heading("Service Port")}
        p { class: "mb-4",
            "Controllers can have a single service port that must be connected to a switched network."
            br {}
            "Usually, the service port is assigned to a management VLAN so that you can access the controller with SSH or a web 
            browser to perform initial configuration or for maintenance."
            br {}
            strong {
                "Notice that the service port supports only a single VLAN, so the corresponding switch port must be configured for 
                access mode only."
            }
        }

        {h3_heading("Distribution System Port")}
        p { class: "mb-4",
            "Controllers also have multiple distribution system ports that you must connect to the network."
            br {}
            "These ports carry most of the data coming to and going from the controller."
            br {}
            "For example, the CAPWAP tunnels (control and data) that extend to each of a controller's APs pass across the 
            distribution system ports."
            br {}
            "Client data also passes from wireless LANs to wired VLANs over the ports."
            br {}
            "In addition, any management traffic using a web browser, SSH, Simple Network Management Protocol (SNMP), Trivial File 
            Transfer Protocol (TFTP), and so on, normally reaches the controller in-band through the ports."
        }

        GreenNote {
            p {
                strong { "NOTE" }
                " You might be thinking that distribution system ports is an odd name for what
                appear to be regular data ports. Recall from the section titled “Wireless LAN Topologies”
                in Chapter 26, “Fundamentals of Wireless Networks,” that the wired network that connects
                APs together is called the distribution system (DS). With the split MAC architecture, the
                point where APs touch the DS is moved upstream to the WLC instead."
            }
        }

        {h3_heading("Trunking with DS ports")}
        p { class: "mb-4",
            "Because the distribution system ports must carry data that is associated with many different
            VLANs, VLAN tags and numbers become very important."
            br {}
            strong {
                "For that reason, the distribution system ports always operate in 802.1Q trunking mode."
            }
            br {}
            "When you connect the ports to a switch, you should also configure the switch ports for unconditional 802.1Q trunk mode."
        }

        {h3_heading("Backup port with DS ports")}
        p { class: "mb-4",
            strong {
                "The distribution system ports can operate independently, each one transporting multiple VLANs to a unique group of 
                internal controller interfaces."
            }
            br {}
            "For resiliency, you can configure distribution system ports in redundant pairs."
            br {}
            "One port is primarily used; if it fails, a backup port is used instead."
        }

        {h3_heading("Link Aggregation Group (LAG) with DS ports")}
        p { class: "mb-4",
            "To get the most use out of each distribution system port, you can configure all of them to operate as a single logical 
            group, much like an EtherChannel or port-channel on a switch."
            br {}
            strong {
                "Controller distribution system ports can be configured as a link aggregation group (LAG) such that they are bundled 
                together to act as one larger link."
            }
            br {}
            "In Figure 29-5, the four distribution system ports are configured as a LAG."
            br {}
            strong {
                "With a LAG configuration, traffic can be load-balanced across the individual ports that make up the LAG."
            }
            br {}
            "In addition, LAG offers resiliency; if one individual port fails, traffic will be redirected to the remaining 
            working ports instead."
        }

        GreenNote {
            p {
                strong { "NOTE" }
                " Be aware that even though the LAG acts as a traditional EtherChannel, Cisco
                WLCs do not support any link aggregation negotiation protocol, like LACP or PaGP,
                at all. Therefore, you must configure the switch ports as an unconditional or always-on
                EtherChannel."
            }
        }

        {h3_heading("RECAP")}
        ol { class: "list-disc list-inside",
            li {
                "Notice that the service port supports only a single VLAN, so the corresponding switch port must be configured 
                for access mode only."
            }
            li {
                "Distribution system ports carry data that is associated with many different VLANs, for that reason, the 
                distribution system ports always operate in 802.1Q trunking mode."
            }
            li {
                "The distribution system ports can also be configured as a Link Aggregation Group (LAG) for load balancing and 
                resiliency. But LAG does not support any link aggregation negotiation protocol, like LACP or PaGP, at all."
            }
        }
    }
}