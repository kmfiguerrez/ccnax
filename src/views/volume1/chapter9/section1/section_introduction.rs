use dioxus::prelude::*;

use crate::{
    components::GreenNote,
    utils::h3_heading
};

#[component]
pub fn SectionIntroductionContent() -> Element {
    rsx! {
        p {
            strong {
                "Without some mechanism like Spanning Tree Protocol (STP) or Rapid STP (RSTP), a LAN
                with redundant links would cause Ethernet frames to loop for an indefinite period of time."
            }
            br {}
            "With STP or RSTP enabled, some switches block ports so that these ports do not forward
            frames."
            br {}
            "STP and RSTP intelligently choose which ports block, with two goals in mind:"
        }
        ul { class: "list-disc list-inside mb-4",
            li {
                "All devices in a VLAN can send frames to all other devices. In other words, STP or RSTP
                does not block too many ports, cutting off some parts of the LAN from other parts."
            }
            li { "Frames have a short life and do not loop around the network indefinitely." }
        }

        p { class: "mb-4",
            "STP and RSTP strike a balance, allowing frames to be delivered to each device, without causing the problems that occur 
            when frames loop through the network over and over again."
        }

        GreenNote {
            p {
                strong { "NOTE" }
                " This first major section of the chapter explains details of both STP and RSTP, so this
                section uses the term STP/RSTP to refer to these protocols together."
                " Note that this term is just a convenient shorthand."
                " Later in the chapter, the text will point out differences between STP and RSTP and begin using the terms STP 
                and RSTP separately, referring to only the specific protocol."
            }
        }

        {h3_heading("STP/RSTP interfaces checks")}
        p { class: "mb-4",
            strong {
                "STP/RSTP prevents looping frames by adding an additional check on each interface before a
                switch uses it to send or receive user traffic."
            }
            br {}
            "That check: If the port is in STP/RSTP forwarding state in that VLAN, use it as normal; if it is in STP/RSTP blocking 
        state, however, block all user traffic and do not send or receive user traffic on that interface in that VLAN."
        }

        {h3_heading("STP/RSTP states")}
        p { class: "mb-4",
            strong {
                "Note that these STP/RSTP states do not change the other information you already know
                about switch interfaces."
            }
            br {}
            "The interface's state of connected/notconnect does not change."
            br {}
            "The interface's operational state as either an access or trunk port does not change."
            br {}
            "STP/RSTP adds this additional state, with the blocking state basically disabling the interface."
        }

        p { class: "mb-4",
            "In many ways, those last two paragraphs sum up what STP/RSTP does."
            br {}
            "However, the details of how STP/RSTP does its work can take a fair amount of study and practice."
            br {}
            "This first major section of the chapter begins by explaining the need for STP/RSTP and the basic ideas of what
            STP/RSTP does to solve the problem of looping frames."
            br {}
            "The majority of this section then looks at how STP/RSTP goes about choosing which switch ports to block to 
            accomplish its goals."
        }

        {h3_heading("RECAP")}
        ol { class: "list-disc list-inside",
            li { "STP/RSTP prevents looping frames in a LAN with redundant links by block some ports." }
            li {
                "Some STP/RSTP states are forwarding state (normal working interface) and 
                blocking state (disabling an interface)."
            }
            li {
                "Note that these STP/RSTP states do not change the other information about switch interfaces: 
                status codes (up/up or connect), trunking operational states (access or trunk)."
            }
        }

    }
}