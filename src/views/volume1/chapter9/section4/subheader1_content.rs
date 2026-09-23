use dioxus::prelude::*;

use crate::{
    components::GreenNote, utils::{h3_heading, TextCommandColor , text_command}
};

#[component]
pub fn Content() -> Element {
    rsx! {
        p { class: "mb-1",
            "The "
            {text_command("show spanning-tree", TextCommandColor::Gold)}
            " lists general RSTP/STP information for all configured VLANs on a switch."
            br {}
            "The "
            {text_command("show spanning-tree active", TextCommandColor::Gold)}
            " lists general RSTP/STP information for only active VLANs on a switch."
            br {}
            "The "
            {text_command("show spanning-tree vlan", TextCommandColor::Gold)}
            i { " x " }
            "lists general RSTP/STP information for a particular VLAN on a switch."
            br {}
            "The three commands list three major groups of messages:"
        }
        ol { class: "list-disc list-inside",
            li { "one group of messages about the root switch," }
            li { "followed by another group about the local switch," }
            li { "and ending with interface role and status information." }
        }
        p { class: "mb-4",
            "The commands identify the root switch and lists settings on the local switch."
        }

        p { class: "mb-4", "The examples in this section use the network shown in Figure O-4." }

        img {
            class: "mb-4 rounded-lg",
            alt: "Figure O-4 Sample LAN for STP Configuration and Verification Examples",
            loading: "lazy",
            src: asset!("/assets/static/v1p3c9s4sh1f0-4.png", AssetOptions::image().with_avif()),
        }

        p { class: "mb-4",
            "Example O-1 begins the discussion with a useful command for STP: the "
            {text_command("show spanning-tree vlan 10", TextCommandColor::Gold)}
            " command."
            br {}
            strong { "This command identifies the root switch and lists settings on the local switch." }
            br {}
            "Example O-1 lists the output of this command on both SW1 and SW2 for VLAN 10, as explained following the example."
        }

        img {
            class: "mb-4 rounded-lg",
            alt: "Figure O-4 Sample LAN for STP Configuration and Verification Examples",
            loading: "lazy",
            src: asset!("/assets/static/v1p3c9s4sh1exo-1.png", AssetOptions::image().with_avif()),
        }

        GreenNote {
            p {
                strong { "NOTE" }
                " In Example 0-1, if you were to type "
                {text_command("show spanning-tree", TextCommandColor::Black)}
                " or "
                {text_command("show spanning-tree active", TextCommandColor::Black)}
                ", it will list the same information for vlan 10. But both commands default to list information 
                beginning with vlan 1."
            
            }
        }

        p { class: "mb-4",
            "Example O-1 begins with the output of the show spanning-tree vlan 10 command on SW1."
            br {}
            "This command first lists three major groups of messages: one group of messages
            about the root switch, followed by another group about the local switch, and ending with
            interface role and status information."
            br {}
            strong {
                "In this case, SW1 lists its own BID as the root, with even a specific statement that “This bridge is the root,” 
            confirming that SW1 is now the root of the VLAN 10 STP topology."
            }
        }

        p { class: "mb-4",
            "Next, compare the highlighted lines of the same command on SW2 in the lower half of the example."
            br {}
            "SW2 lists SW1's BID details as the root; in other words, SW2 agrees that SW1 has
            won the root election."
            br {}
            strong { "SW2 does not list the phrase “This bridge is the root.”" }
            br {}
            "SW2 then lists its own (different) BID details in the lines after the details about the root's BID."
        }

        {h3_heading("The priority field value")}
        p { class: "mb-4",
            "The output also confirms a few default values."
            br {}
            "First, each switch lists the priority part of
            the BID as a separate number: 32778."
            br {}
            "This value comes from the default priority of 32768, plus VLAN 10, for a total of 32778."
            br {}
            "The output also shows the interface cost for some Fast Ethernet and Gigabit Ethernet interfaces, defaulting to 
            19 and 4, respectively."
        }

        {h3_heading("Interface role and status information")}
        p {
            "Finally, the bottom of the output from the "
            {text_command("show spanning-tree", TextCommandColor::Gold)}
            " command lists each
            interface in the VLAN, including trunks, with the STP port role and port state listed."
            br {}
            "For instance, on switch SW1, the output lists three interfaces, with a role of Desg for designated
            port (DP) and a state of FWD for forwarding."
            br {}
            "SW2 lists three interfaces, two DPs, and one root port, so all three are in an FWD or forwarding state."
        }

    }
}