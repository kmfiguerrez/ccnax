use dioxus::prelude::*;

use crate::{
    components::{KeyTopic, GreenNote},
    utils::h3_heading
};

#[component]
pub fn SectionIntroductionContent() -> Element {
    rsx! {
        p { class: "mb-4",
            "A Cisco wireless network can consist of autonomous APs or lightweight APs that are coupled with one or more wireless 
            LAN controllers."
            br {}
            "Both types of APs are covered in Chapter 27, “Analyzing Cisco Wireless Architectures,” from a functional perspective."
            br {}
            "You should also understand how to connect the wired side of each type of AP so that it can pass traffic
            between the appropriate VLANs and WLANs."
        }

        p { class: "mb-4",
            strong {
                "Recall that an autonomous AP is a standalone device; nothing else is needed to forward Ethernet frames from a wired 
                VLAN to a wireless LAN, and vice versa."
            }
            br {}
            "In effect, the AP maps each VLAN to a WLAN and BSS."
            br {}
            "The autonomous AP has a single wired Ethernet interface, as shown in the left portion of Figure 29-1, which means that 
            multiple VLANs must be brought to it over a trunk link."
        }

        GreenNote {
            p {
                strong { "NOTE" }
                " A switch port providing a wired connection to an AP must be configured to support either access or trunk mode. In 
                trunk mode, 802.1Q encapsulation tags each frame according to the VLAN number it came from. The wireless side of an 
                AP inherently trunks 802.11 frames by marking them with the BSSID of the WLAN where they belong."
            }
        }

        p { class: "mb-4",
            "A lightweight AP also has a single wired Ethernet interface; however, it must be paired with a WLC to be fully 
            functional."
            br {}
            "Wired VLANs that terminate at the WLC can be mapped to WLANs that emerge at the AP."
            br {}
            strong {
                "Even though multiple VLANs are being extended from the WLC to the AP, they are all carried over the CAPWAP tunnel 
                between the two."
            }
            br {}
            "That means the AP needs only an access link to connect to the network infrastructure and terminate its
            end of the tunnel, as shown in the right portion of Figure 29-1."
        }

        KeyTopic {}
        img {
            class: "mb-4 rounded-lg",
            alt: "Figure 29-1 Comparing Connections to Autonomous and Lightweight APs",
            loading: "lazy",
            src: asset!("/assets/static/v1p8c29s1f29-1.png", AssetOptions::image().with_avif()),
        }

        p { class: "mb-4",
            "To configure and manage Cisco APs, you can connect a serial console cable from your PC to the console port on the AP."
            br {}
            "Once the AP is operational and has an IP address, you can also use Telnet or SSH to connect to its CLI over the wired 
            network."
            br {}
            "Autonomous APs support browser-based management sessions via HTTP and HTTPS."
            br {}
            strong { "You can manage lightweight APs from a browser session to the WLC." }
        }

        {h3_heading("RECAP")}
        ol { class: "list-disc list-inside",
            li {
                "Recall that an autonomous AP is a standalone device; nothing else is needed to forward Ethernet frames from a 
                wired VLAN to a wireless LAN, and vice versa."
            }
            li {
                "With Split-MAc architecture, APs don't need trunk links to the wired network; they can use an access link to 
                connect to the WLC, which then maps wired VLANs to WLANs."
            }
        }

    }
}