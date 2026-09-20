use dioxus::prelude::*;

use crate::{
    components::KeyTopic, utils::{h3_heading, text_command}
};

#[component]
pub fn Content() -> Element {
    rsx! {
        p {
            "Because of the sequence of events over the history of the various STP family of protocols,
            vendors like Cisco needed to create their own proprietary features to create the per-VLAN
            spanning tree concept shown in Figure 10-2."
            br {}
            "That sequence resulted in the following:"
        }
        ol { class: "list-disc list-inside mb-4",
            li {
                "When STP was the only STP standard back in the 1990s with 802.1D, Cisco created the
                STP-based Per VLAN Spanning Tree Plus (PVST+) protocol, which creates one spanning
                tree instance per VLAN."
            }
            li {
                "When the IEEE introduced RSTP (in 802.1D amendment 802.1w, in the year 2001), Cisco
                also created the Rapid PVST+ (RPVST+) protocol. RPVST+ provided more features than
                standardized RSTP, including one tree per VLAN."
            }
            li {
                "The IEEE did not adopt Cisco's PVST+ or RPVST+ into their standards to create multiple
                spanning trees. Instead, the IEEE created a different method: Multiple Spanning Tree
                Protocol (MSTP), originally defined in 802.1Q amendment 802.1s."
            }
        }

        p { "Figure 10-3 shows the features as a timeline for perspective." }

        img {
            class: "mb-4 rounded-lg",
            alt: "Figure 10-3 Timeline of Per-VLAN and Multiple STP Features",
            loading: "lazy",
            src: asset!("/assets/static/v1p3c10s1sh2f10-3.png", AssetOptions::image().with_avif()),
        }

        p { class: "mb-4",
            "Today, Cisco Catalyst switches give us three options to configure on the "
            {text_command("spanning-tree mode", text_command::TextCommandColor::Gold)}
            " command, which tells the switch which type of STP to use."
            br {}
            strong { "Note that the switches do not support STP or RSTP with the single tree (CST)." }
            br {}
            "They can use either the Cisco-proprietary and STP-based PVST+, Cisco-proprietary and RSTP-based RPVST+, or the IEEE
            standard MSTP."
            br {}
            "Table 10-2 summarizes some of the facts about these standards and options, along with the keywords used on 
            the spanning-tree mode global configuration command."
            br {}
            "Example 10-1, which follows, shows the command options in global configuration mode."
        }

        KeyTopic {}
        img {
            class: "rounded-lg",
            alt: "Table 10-2 STP Standards and Configuration Options",
            loading: "lazy",
            src: asset!("/assets/static/v1p3c10s1sh2t10-2.png", AssetOptions::image().with_avif()),
        }
        p { class: "text-sm mb-4",
            "* MSTP allows the definition of as many instances (multiple spanning tree instances, or MSTIs) as chosen by
            the network designer but does not require one per VLAN."
        }

        img {
            class: "mb-4 rounded-lg",
            alt: "Example 10-1 STP Status with Default STP Parameters on SW1 and SW2",
            loading: "lazy",
            src: asset!("/assets/static/v1p3c10s1sh2ex10-1.png", AssetOptions::image().with_avif()),
        }

        {h3_heading("RECAP")}
        ol { class: "list-disc list-inside",
            li {
                "STP-based Per VLAN Spanning Tree Plus (PVST+) and RSTP-based Rapid PVST Plus (RPVST+) protocols are 
                Cisco proprietary that create one spanning tree instance per VLAN."
            }
            li {
                "IEEE created their own protocol called Multiple Spanning Tree Protocol (MSTP) instead of adopting
                Cisco's PVST+ or RPVST+ into their standards to create multiple spanning trees."
            }
            li {
                "MSTP was originally defined in IEEE 802.1s, which was later incorporated into the 
                broader IEEE 802.1Q standard series (beginning with IEEE 802.1Q-2005)."
            }
            li {
                "Note that the switches do not support STP or RSTP with the single tree (CST). They only support 
                protocols that can make 1 tree per vlan."
            }
        }

    }
}