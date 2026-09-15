use dioxus::prelude::*;

use crate::utils::h3_heading;

#[component]
pub fn ChapterIntroductionContent() -> Element {
    rsx! {
        p { class: "mb-4",
            strong {
                "Spanning Tree Protocol (STP) allows Ethernet LANs to have the added benefits of installing
                redundant links in a LAN, while overcoming the known problems that occur when adding
                those extra links."
            }
            br {}
            "Using redundant links in a LAN design allows the LAN to keep working
            even when some links fail or even when some entire switches fail."
            br {}
            "Proper LAN design should add enough redundancy so that no single point of failure crashes the LAN; STP allows the
            design to use redundancy without causing some other problems."
        }

        {h3_heading("IEEE 802.1D")}
        p { class: "mb-4",
            "Historically, "
            strong { "the IEEE first standardized STP as part of the IEEE 802.1D" }
            " standard back in
            1990, with pre-standard versions working even before that time."
            br {}
            "Over time, the industry and IEEE improved STP, with the eventual replacement of STP with an improved 
            protocol: Rapid Spanning Tree Protocol (RSTP)."
            br {}
            strong {
                "The IEEE first released RSTP as amendment 802.1w and, in 2004, integrated RSTP into the 802.1D standard."
            }
        }

        {h3_heading("RSTP")}
        p { class: "mb-4",
            "An argument could be made to ignore STP today and instead focus solely on RSTP."
            br {}
            strong { "Most modern networks use RSTP instead of STP." }
            br {}
            "The most recent models and IOS versions of Cisco switches default to use RSTP instead of STP."
            br {}
            "Plus, the CCNA 200-301 exam topics mention RSTP by name, but not STP."
            br {}
            "However, STP and RSTP share many of the same mechanisms, and RSTP's improvements can be best understood in comparison 
            to STP."
            br {}
            "For that reason, this chapter presents some details that apply only to STP, as a learning tool to
            help you understand RSTP."
        }

        p { "This chapter organizes the material into three sections." }
        ol { class: "list-disc list-inside mb-4",
            li {
                "The first section presents some core concepts about how both STP and RSTP discover a tree made of nodes 
                (switches) and links so that no loops exist in a network."
            }
            li {
                "The second section then takes a brief look at the area for
                which STP differs the most from RSTP: in how STP reacts to changes in the network."
            }
            li {
                "This chapter ends with a third major section that details RSTP, including how RSTP works much
                better that STP when reacting to changes."
            }
        }

    }
}