use dioxus::prelude::*;

use crate::utils::{TextCommandColor, h3_heading, text_command};

#[component]
pub fn Content() -> Element {
    rsx! {
        p { class: "mb-4",
            "Unfortunately, it is possible to set the native VLAN ID to different VLANs on either end of
            the trunk, using the "
            {text_command("switchport trunk native vlan", TextCommandColor::Gold)}
            i { " vlan-id" }
            " command."
            br {}
            "If the native VLANs differ according to the two neighboring switches, the switches will cause frames sent in the
            native VLAN to jump from one VLAN to the other."
        }

        p { class: "mb-4",
            "For example, if switch SW1 sends a frame using native VLAN 1 on an 802.1Q trunk, SW1
            does not add a VLAN header, as is normal for the native VLAN."
            br {}
            "When switch SW2 receives the frame, noticing that no 802.1Q header exists, SW2 assumes that the frame is part of
            SW2's configured native VLAN."
            br {}
            "If SW2 has been configured to think VLAN 2 is the native VLAN on that trunk, SW2 will try to forward the received 
            frame into VLAN 2."
            br {}
            "(This effect of a frame being sent in one VLAN but then being believed to be in a different VLAN is
            called "
            i { "VLAN hopping" }
            "."
        }

        {h3_heading("RECAP")}
        ol { class: "list-disc list-inside",
            li {
                "Unfortunately, it is possible to set the native VLAN ID to different VLANs on either end of
                the trunk, using the "
                {text_command("switchport trunk native vlan", TextCommandColor::Gold)}
                i { " vlan-id" }
                " command."
            }
            li {
                "VLAN hopping is the process of sending frame in one VLAN but the receiving end believes the frame 
                is in a different VLAN."
            }
            li { "Watch out for mismatch Native VLAN on a trunk." }
        }
    }
}