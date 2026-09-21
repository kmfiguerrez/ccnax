// Note that this section is not part of the book.
// I just added it to focus on the verification commands for RSTP/STP.

use dioxus::prelude::*;

use crate::{
    components::{RedNote, GreenNote}, utils::{TextCommandColor, text_command}
};

#[component]
pub fn SectionIntroductionContent() -> Element {
    rsx! {
        p { class: "mb-4",
            "First of all, this content is not part of the book."
            br {}
            "I just added it to focus on the verification commands for RSTP/STP once the topology
            has settled into stable state."
            br {}
            "The commands shown here are based on the current version of packet tracer version (8.2.2.0400)."
        
        }

        p { class: "mb-4",
            "Let's begin with stuff we should keep in mind."
            br {}
            "Note that the switches do not support STP or RSTP with the single tree (CST)."
            br {}
            "Meaning they only support types of RSTP/STP that supports 1 CST per vlan."
            br {}
            "They can use either the Cisco-proprietary and STP-based PVST+, Cisco-proprietary and RSTP-based RPVST+, 
            or the IEEE standard MSTP."
        }

        RedNote {
            p {
                strong { "NOTE" }
                " In packet tracer version (8.2.2.0400)."
                br {}
                "The IEEE MSTP protocol is not available on switches."
            }
        }

        p { "The list of verification commands we're going to examine are:" }
        ul { class: "list-disc list-inside mb-4",
            li {
                {text_command("show spanning-tree", TextCommandColor::Gold)}
                ": General RSTP/STP information on VLANs."
            }
            li {
                {text_command("show spanning-tree active", TextCommandColor::Gold)}
                ": General RSTP/STP information on active VLANs."
            }
            li {
                {text_command("show spanning-tree detail", TextCommandColor::Gold)}
                ": Detail RSTP/STP information on VLANs."
            }
            li {
                {text_command("show spanning-tree interface", TextCommandColor::Gold)}
                ": Spanning Tree interface status and configuration."
            }
            li {
                {text_command("show spanning-tree summary", TextCommandColor::Gold)}
                ": RSTP/STP summary information."
            }
            li {
                {text_command("show spanning-tree vlan", TextCommandColor::Gold)}
                ": General RSTP/STP information for a particular VLAN."
            }
        }

        p { class: "mb-4",
            "The following headings talk about the commands."
            br {}
            "The first heading talks about the "
            {text_command("show spanning-tree", TextCommandColor::Gold)}
            ", "
            {text_command("show spanning-tree active", TextCommandColor::Gold)}
            " and the "
            {text_command("show spanning-tree vlan", TextCommandColor::Gold)}
            i { " x" }
            " commands."
            br {}
            "The second heading talks about the "
            {text_command("show spanning-tree detail", TextCommandColor::Gold)}
            "."
            br {}
            "The third heading talks about the "
            {text_command("show spanning-tree interface", TextCommandColor::Gold)}
            "."
            br {}
            "The fourth heading talks about the "
            {text_command("show spanning-tree summary", TextCommandColor::Gold)}
            "."
            br {}
        }

        GreenNote {
            p {
                strong { "NOTE" }
                " The "
                {text_command("show spanning-tree", TextCommandColor::Black)}
                " enable mode command is already suffice for the CCNA exam."
            
            }
        }

    }
}