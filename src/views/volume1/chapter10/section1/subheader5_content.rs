use dioxus::prelude::*;

use crate::{components::{KeyTopic, GreenNote}, utils::h3_heading};

#[component]
pub fn Content() -> Element {
    rsx! {
        p { class: "mb-4",
            "Although the history and configuration might make the BID priority idea seem a bit convoluted, having an extra 
            12-bit field in the BID works well in practice because it can be used to identify the VLAN ID."
            br {}
            "VLAN IDs range from 1 to 4094, requiring 12 bits."
        }

        p {
            "For the purposes of discussion, focus on the standard RSTP and its Cisco-proprietary cousin RPVST+."
            br {}
            "Both use the RSTP mechanisms as discussed in Chapter 9, “Spanning Tree Protocol Concepts,” 
            but RPVST+ uses the mechanisms for every VLAN, while standard RSTP does not."
            br {}
            "So how do their methods differ?"
        }
        KeyTopic {}
        ul { class: "list-disc list-inside mb-4",
            li {
                "RSTP creates one tree—the Common Spanning Tree (CST)—while RPVST+ creates one
                tree for each and every VLAN."
            }
            li {
                "RSTP sends one set of RSTP messages (BPDUs) in the network, no matter the number of
                VLANs, while RPVST+ sends one set of messages per VLAN."
            }
            li {
                "RSTP and RPVST+ use different destination MAC addresses: RSTP with multicast address
                0180.C200.0000 (an address defined in the IEEE standard), and RPVST+ with multicast
                address 0100.0CCC.CCCD (an address chosen by Cisco)."
            }
            li {
                "When transmitting messages on VLAN trunks, RSTP sends the messages in the native
                VLAN with no VLAN header/tag. RPVST+ sends each VLAN's messages inside that
                VLAN—for instance, BPDUs about VLAN 9 have an 802.1Q header that lists VLAN 9."
            }
            li {
                "RPVST+ adds an extra type-length value (TLV) to the BPDU that identifies the VLAN ID,
                while RSTP does not (because it does not need to, as RSTP ignores VLANs.)"
            }
            li {
                "Both view the 16-bit priority as having a 12-bit System ID Extension, with RSTP setting
                the value to 0000.0000.0000, meaning “no VLAN,” while RPVST+ uses the VLAN ID. "
            }
        }

        p { class: "mb-4",
            "In other words, "
            strong { "standard RSTP behaves as if VLANs do not exist" }
            ", while Cisco's RPVST+ integrates VLAN information 
            into the entire process. "
        }

        GreenNote {
            p {
                strong { "NOTE" }
                " Some documents refer to the feature of sending BPDUs over trunks with VLAN
                tags matching the same VLAN as BPDU tunneling."
            }
        }

        {h3_heading("RECAP")}
        ol { class: "list-disc list-inside",
            li { "RSTP/STP ignores VLANs! Standard RSTP/STP behaves as if VLANs do not exist" }
            li {
                "When transmitting messages on VLAN trunks, RSTP/STP sends the messages in the native VLAN with no VLAN header/tag."
            }
            li {
                "RSTP/STP view the 16-bit priority as having a 12-bit System ID Extension, with RSTP/STP setting the value to 
                0000.0000.0000, meaning “no VLAN,”."
            }
        }

    }
}