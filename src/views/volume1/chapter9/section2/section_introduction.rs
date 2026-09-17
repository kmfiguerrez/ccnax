use dioxus::prelude::*;

#[component]
pub fn SectionIntroductionContent() -> Element {
    rsx! {
        p { class: "mb-4",
            "As promised in the introduction to this chapter, the first section showed features that apply
            to both STP and RSTP."
            br {}
            "This next heading acts as the turning point, with the next several
            pages being about STP only."
            br {}
            "The upcoming section titled “Rapid STP Concepts” then shows
            details specific to RSTP, in contrast to STP."
        }

        p { class: "mb-4",
            "Once the engineer has finished all STP configuration, the STP topology should settle into a
            stable state and not change, at least until the network topology changes."
            br {}
            "This section examines the ongoing operation of STP while the network is stable, and then it covers how STP
            converges to a new topology when something changes."
        }

        p { class: "mb-4",
            strong {
                "Note that almost all the differences between STP and RSTP revolve around the activities
            of waiting for and reacting to changes in the topology."
            }
            br {}
            "STP performed well for the era and circumstances in which it was created."
            br {}
            strong {
                "The “rapid” in RSTP refers to the improvements to how fast RSTP could react when changes occur—so understanding 
                how STP reacts will be useful to understand why RSTP reacts faster."
            }
            br {}
            "These next few topics show the specifics of STP (and not RSTP) and how STP reacts to and manages convergence when 
            changes happen in an Ethernet LAN."
        }
    }
}