use dioxus::prelude::*;

use crate::{components::KeyTopic, utils::h3_heading};

#[component]
pub fn Content() -> Element {
    rsx! {
        p { class: "mb-4",
            "Notice that a BSS involves a single AP and no explicit connection into a regular Ethernet network."
            br {}
            "In that setting, the AP and its associated clients make up a standalone network."
            br {}
            strong {
                "But the AP's role at the center of the BSS does not just stop with managing the BSS; sooner or later, wireless clients 
                will need to communicate with other devices that are not members of the BSS."
            }
            br {}
            "Fortunately, an AP can also uplink into an Ethernet network because it has both wireless and wired capabilities."
            br {}
            strong {
                "The 802.11 standard refers to the upstream wired Ethernet as the distribution system (DS) for the wireless BSS, as 
                shown in Figure 26-6."
            }
        }

        {h3_heading("AP maps a VLAN to an SSID")}
        p { class: "mb-4",
            strong {
                "You can think of an AP as a translational bridge, where frames from two dissimilar media
                (wireless and wired) are translated and then bridged at Layer 2."
            }
            br {}
            "In simple terms, the AP is in charge of mapping a virtual local-area network (VLAN) to an SSID."
            br {}
            "In Figure 26-6, the AP maps VLAN 10 to the wireless LAN using SSID “MyNetwork.”"
            br {}
            "Clients associated with the “MyNetwork” SSID will appear to be connected to VLAN 10."
        }

        img {
            class: "mb-4 rounded-lg",
            alt: "Figure 26-6 Distribution System Supporting a BSS",
            loading: "lazy",
            src: asset!("/assets/static/v1p8c26s2sh2f26-6.png", AssetOptions::image().with_avif()),
        }

        {h3_heading("Mapping VLANs to multiple SSIDs")}
        p { class: "mb-4",
            "This concept can be extended so that multiple VLANs are mapped to multiple SSIDs."
            br {}
            strong {
                "To do this, the AP must be connected to the switch by a trunk link that carries the VLANs."
            }
            br {}
            "In Figure 26-7, VLANs 10, 20, and 30 are trunked to the AP over the DS."
            br {}
            strong { "The AP uses the 802.1Q tag to map the VLAN numbers to the appropriate SSIDs." }
            br {}
            "For example, VLAN 10 is mapped to SSID “MyNetwork,” VLAN 20 is mapped to SSID “YourNetwork,” and VLAN 30 to 
            SSID “Guest.”"
        }

        p { class: "mb-4",
            strong {
                "In effect, when an AP uses multiple SSIDs, it is trunking VLANs over the air, and over the
                
                same channel, to wireless clients."
            }
            br {}
            "The clients must use the appropriate SSID that has been mapped to the respective VLAN when the AP was configured."
            br {}
            strong {
                "The AP then appears as multiple logical APs—one per BSS—with a unique BSSID for each."
            }
            br {}
            "With Cisco APs, this is usually accomplished by incrementing the last digit of the radio's MAC address for each SSID."
        }

        {h3_heading("Note on VLAN Trunking via APs")}
        p { class: "mb-4",
            "Even though an AP can advertise and support multiple logical wireless networks, each of the
            SSIDs covers the same geographic area."
            br {}
            strong {
                "The reason is that the AP uses the same transmitter, receiver, antennas, and channel for every SSID that it supports."
            }
            br {}
            "Beware of one misconception though: multiple SSIDs can give an illusion of scale."
            br {}
            "Even though wireless clients can be distributed across many SSIDs, all of those clients must share the same AP's 
            hardware and must contend for airtime on the same channel."
        }

        KeyTopic {}
        img {
            class: "mb-4 rounded-lg",
            alt: "Figure 26-7 Supporting Multiple SSIDs on One AP",
            loading: "lazy",
            src: asset!("/assets/static/v1p8c26s2sh2f26-7.png", AssetOptions::image().with_avif()),
        }

        {h3_heading("RECAP")}
        ol { class: "list-disc list-inside",
            li { "BSS with a single AP and its associated clients make up a standalone network." }
            li {
                "AP's role at the center of the BSS does not just stop with managing the BSS; sooner or later, wireless clients 
                will need to communicate with other devices that are not members of the BSS."
            }
            li {
                "An AP can also uplink into an Ethernet network because it has both wireless and wired capabilities."
            }
            li {
                "The 802.11 standard refers to the upstream wired Ethernet as the distribution system (DS) for the wireless BSS."
            }
            li {
                "You can think of an AP as a translational bridge, where frames from two dissimilar media (wireless and wired) are 
                translated and then bridged at Layer 2. In simple terms, the AP is in charge of mapping a virtual local-area network 
                (VLAN) to an SSID."
            }
            li {
                "AP also supports 802.1Q VLAN trunking to map multiple VLANs to multiple SSIDs. The AP must be connected to the switch by a 
                trunk link that carries the VLANs."
            }
            li {
                "When an AP uses multiple SSIDs, it is trunking VLANs over the air, and over the same channel, to wireless clients. 
                The AP then appears as multiple logical APs—one per BSS—with a unique BSSID for each. With Cisco APs, this is 
                usually accomplished by incrementing the last digit of the radio's MAC address for each SSID."
            }
        }

    }
}