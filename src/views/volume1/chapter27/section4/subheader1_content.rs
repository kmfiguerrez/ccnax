use dioxus::prelude::*;

use crate::components::{KeyTopic, GreenNote};

#[component]
pub fn Content() -> Element {
    rsx! {
        p { class: "mb-1",
            "Many Cisco APs can operate in either autonomous or lightweight mode, depending on which code image is loaded and run."
            br {}
            "From the WLC, you can also configure a lightweight AP to operate in one of the following special-purpose modes:"
        }
        KeyTopic {}
        ol { class: "list-disc list-inside mb-4",
            li {
                span { class: "font-bold mr-1", "Local:" }
                "The default lightweight mode that offers one or more functioning BSSs on a specific channel."
                " During times that it is not transmitting, the AP will scan the other channels to measure the level of noise, 
                measure interference, discover rogue devices, and match against intrusion detection system (IDS) events."
            }
            li {
                span { class: "font-bold mr-1", "Monitor:" }
                "The AP does not transmit at all, but its receiver is enabled to act as a dedicated
                sensor. The AP checks for IDS events, detects rogue access points, and determines the
                position of stations through location-based services."
            }
            li {
                span { class: "font-bold mr-1", "FlexConnect:" }
                "An AP at a remote site can locally switch traffic between an SSID and a VLAN if its CAPWAP tunnel to the WLC is 
                down and if it is configured to do so."
            }
            li {
                span { class: "font-bold mr-1", "Sniffer:" }
                "An AP dedicates its radios to receiving 802.11 traffic from other sources, much
                like a sniffer or packet capture device. The captured traffic is then forwarded to a PC running network analyzer 
                software such as Wildpackets OmniPeek or WireShark, where it can be analyzed further."
            }
            li {
                span { class: "font-bold mr-1", "Rogue detector:" }
                "An AP dedicates itself to detecting rogue devices by correlating MAC
                addresses heard on the wired network with those heard over the air. Rogue devices are
                those that appear on both networks."
            }
            li {
                span { class: "font-bold mr-1", "Bridge:" }
                "An AP becomes a dedicated bridge (point-to-point or point-to-multipoint)
                between two networks. Two APs in bridge mode can be used to link two locations separated by a distance. Multiple 
                APs in bridge mode can form an indoor or outdoor mesh network."
            }
            li {
                span { class: "font-bold mr-1", "Flex+Bridge:" }
                "FlexConnect operation is enabled on a mesh AP."
            }
            li {
                span { class: "font-bold mr-1", "SE-Connect:" }
                " The AP dedicates its radios to spectrum analysis on all wireless channels.
                You can remotely connect a PC running software such as MetaGeek Chanalyzer or Cisco
                Spectrum Expert to the AP to collect and analyze the spectrum analysis data to discover
                sources of interference."
            }
        }

        GreenNote {
            p {
                strong { "NOTE" }
                " Remember that a lightweight AP is normally in local mode when it is providing
                BSSs and allowing client devices to associate to wireless LANs. When an AP is configured to
                operate in one of the other modes, local mode (and the BSSs) is disabled."
            }
        }
    }
}