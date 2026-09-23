use dioxus::prelude::*;

use crate::{
    components::KeyTopic,
    utils::h3_heading
};

#[component]
pub fn Content() -> Element {
    rsx! {
        p { class: "mb-4",
            "An STP root switch sends a new Hello BPDU every 2 seconds by default."
            br {}
            strong {
                "Each nonroot switch forwards the Hello on all DPs, but only after changing items listed in the Hello."
            }
            br {}
            "(As a result, the Hello flows once over every working link in the LAN.) "
        }

        {h3_heading("Nonroot switch forwards Hello BPU")}
        p { class: "mb-4",
            "When forwarding the Hello BPDU, each switch sets the root cost to that local switch's calculated root cost."
            br {}
            "The switch also sets the “sender's bridge ID” field to its own bridge ID."
            br {}
            "(The root's bridge ID field is not changed.)"
        }

        {h3_heading("Steady-state operations")}
        p {
            "Assuming a default Hello timer of 2 seconds on the root switch, each switch will forward
            the received (and changed) Hellos out all DPs so that all switches continue to receive Hellos
            every 2 seconds."
            br {}
            "The following steps summarize the steady-state operation when nothing is
            currently changing in the STP topology:"
        }
        KeyTopic {}
        ol { class: "pl-1 mb-4",
            li {
                span { class: "text-blue-500 mr-4", "Step 1." }
                " The root creates and sends a Hello BPDU, with a root cost of 0, out all its working interfaces 
                (those in a forwarding state)."
            }
            li {
                span { class: "text-blue-500 mr-4", "Step 2." }
                " The nonroot switches receive the Hello on their root ports. After changing the
                Hello to list their own BID as the sender's BID and listing that switch's root
                cost, the switch forwards the Hello out all designated ports."
            }
            li {
                span { class: "text-blue-500 mr-4", "Step 3." }
                " Steps 1 and 2 repeat until something changes."
            }
        }

        {h3_heading("Events that triggers STP convergence")}
        p { class: "mb-4",
            "When a switch fails to receive a Hello, it knows a problem might be occurring in the network."
            br {}
            strong {
                "Each switch relies on these periodically received Hellos from the root as a way to
                know that its path to the root is still working."
            }
            br {}
            "When a switch ceases to receive the Hellos, or receives a Hello that lists different details, something has failed, 
            so the switch reacts and starts the process of changing the spanning-tree topology."
        }

        {h3_heading("RECAP")}
        ol { class: "list-disc list-inside",
            li { "An STP root switch sends a new Hello BPDU every 2 seconds by default." }
            li {
                "Each nonroot switch forwards the Hello on all DPs, but only after changing items listed in the Hello."
            }
            li { "The root's bridge ID field is not changed when forwarding Hello BPDU." }
            li {
                "When forwarding the Hello BPDU, each switch sets the root cost to that local switch's calculated root cost."
            }
            li { "The switch also sets the “sender's bridge ID” field to its own bridge ID." }
            li { "The designated ports are the ports out which Hello BPDUs are forwarded." }
            li {
                "When a switch ceases to receive the Hellos, or receives a Hello that lists different details, something has 
                failed, so the switch reacts and starts the process of changing the spanning-tree topology."
            }
        }
    }
}