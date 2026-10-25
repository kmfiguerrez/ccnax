use dioxus::prelude::*;

use crate::{
    components::KeyTopic,
    utils::h3_heading
};

#[component]
pub fn SectionIntroductionContent() -> Element {
    rsx! {
        p { class: "mb-4",
            "An access point's primary function is to bridge wireless data from the air to a normal wired network."
            br {}
            strong {
                "An AP can accept “connections” from a number of wireless clients so that they become members of the LAN, as if the 
                same clients were using wired connections."
            }
        }

        p { class: "mb-4",
            strong {
                "APs act as the central point of access (hence the AP name), controlling client access to the wireless LAN."
            }
            br {}
            "An "
            i { "autonomous AP" }
            " is self-contained; it is equipped with both wired and wireless hardware so that the wireless client associations can 
            be terminated onto a wired connection locally at the AP."
            br {}
            "The APs and their data connections must be distributed across the coverage area and across the network."
        }

        {h3_heading("Autonomous APs as an extension of DS")}
        p { class: "mb-4",
            "Autonomous APs offer one or more fully functional, standalone basic service sets (BSSs)."
            br {}
            strong {
                "They are also a natural extension of a switched network, connecting wireless service set
            identifiers (SSIDs) to wired virtual LANs (VLANs) at the access layer."
            }
            br {}
            "Figure 27-1 shows the basic architecture; even though only four APs are shown across the bottom, a typical enterprise 
            network could consist of hundreds or thousands of APs."
        }

        KeyTopic {}
        img {
            class: "mb-4 rounded-lg",
            alt: "Figure 27-1 Wireless Network Architecture with Autonomous APs",
            loading: "lazy",
            src: asset!("/assets/static/v1p8c27s1f27-1.png", AssetOptions::image().with_avif()),
        }

        {h3_heading("Connecting autonomous APs to DS")}
        p { class: "mb-4",
            "What exactly does an autonomous AP need to become a part of the network?"
            br {}
            "The wireless network in Figure 27-1 consists of two SSIDs: wlan100 and wlan200."
            br {}
            "These correspond to wired VLANs 100 and 200, respectively."
            br {}
            strong {
                "As shown by the shaded links, the VLANs must be trunked from the distribution layer switch (where routing commonly 
                takes place) to the access layer, where they are extended further over a trunk link to the AP."
            }
        }

        {h3_heading("Autonomous APs data path")}
        p { class: "mb-4",
            strong {
                "An autonomous AP offers a short and simple path for data to travel between the wireless and wired networks."
            }
            br {}
            "Data has to travel only through the AP to reach the network on the other side."
            br {}
            "Two wireless users that are associated to the same autonomous AP can reach each other
            through the AP without having to pass up into the wired network."
            br {}
            strong {
                "As you work through the wireless architectures discussed in the rest of the chapter, notice the data path that is
                required for each."
            }
        }

        {h3_heading("Autonomous APs need management IP address")}
        p { class: "mb-4",
            strong {
                "An autonomous AP must also be configured with a management IP address (10.10.10.10 in Figure 27-1) so that you can 
                remotely manage it."
            }
            br {}
            "After all, you will want to configure SSIDs, VLANs, and many RF parameters like the channel and transmit power to be 
            used."
            br {}
            "The management address is not normally part of any of the data VLANs, so a dedicated management VLAN (i.e., VLAN 10) 
            must be added to the trunk links to reach the AP."
            br {}
            strong {
                "Each AP must be configured and maintained individually unless you leverage a management platform such as
                Cisco Prime Infrastructure or Cisco DNA Center."
            }
        }

        {h3_heading("Cons with Autonomous AP Architecture")}
        p { class: "mb-4",
            strong {
                "Because the data and management VLANs may need to reach every autonomous AP, the network configuration and efficiency 
                can become cumbersome as the network scales."
            }
            br {}
            "For example, you will likely want to offer the same SSID on many APs so that wireless clients can associate with that 
            SSID in most any location or while roaming between any two APs."
            br {}
            "You might also want to extend the corresponding VLAN (and IP subnet) to each and every AP so that clients do not have 
            to request a new IP address for each new association."
        }

        p { class: "mb-4",
            "Because SSIDs and their VLANs must be extended at Layer 2, you should consider how they
            are extended throughout the switched network."
            br {}
            "The shaded links in Figure 27-2 show an example of a single VLAN's extent in the data plane."
            br {}
            "Working top to bottom, follow VLAN 100 as it reaches through the network."
            br {}
            "VLAN 100 is routed within the distribution layer and must be carried over trunk links to the access layer switches 
            and then to each autonomous AP."
            br {}
            strong {
                "In effect, VLAN 100 must extend end to end across the whole infrastructure—something that is usually considered to be 
                a bad practice."
            }
        }

        p { class: "mb-4",
            "That might sound straightforward until you have to add a new VLAN and configure every switch and AP in your network to 
            carry and support it."
            br {}
            "Even worse, suppose your network has redundant links between each layer of switches."
            br {}
            "The Spanning Tree Protocol (STP) running on each switch becomes a vital ingredient to prevent bridging loops from 
            forming and corrupting the network."
            br {}
            "For these reasons, client roaming across autonomous APs is typically limited to the Layer 2 domain, or the extent of a 
            single VLAN."
            br {}
            "As the wireless network expands, the infrastructure becomes more difficult to configure correctly and becomes less
            efficient."
        }

        img {
            class: "mb-4 rounded-lg",
            alt: "Figure 27-2 Extent of a Data VLAN in a Network of Autonomous APs",
            loading: "lazy",
            src: asset!("/assets/static/v1p8c27s1f27-2.png", AssetOptions::image().with_avif()),
        }

        {h3_heading("RECAP")}
        ol { class: "list-disc list-inside",
            li {
                "An access point's primary function is to bridge wireless data from the air to a normal wired network."
            }
            li {
                "An "
                i { "autonomous AP" }
                " is self-contained; it is equipped with both wired and wireless hardware so that the wireless client associations can 
                be terminated onto a wired connection locally at the AP."
            }
            li {
                "Autonomous APs offer one or more fully functional, standalone basic service sets (BSSs)."
            }
            li {
                "Autonomous APs are also a natural extension of a switched network, connecting wireless service set identifiers 
                (SSIDs) to wired virtual LANs (VLANs) at the access layer."
            }
            li {
                "An autonomous AP offers a short and simple path for data to travel between the wireless and wired networks."
            }
            li {
                "An autonomous AP must also be configured with a management IP address, better with management VLAN."
            }
        }
    }
}