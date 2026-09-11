use dioxus::prelude::*;

#[component]
pub fn ChapterIntroductionContent() -> Element {
    rsx! {
        p {
            "So far, you have learned that Ethernet switches receive Ethernet frames, make
            decisions, and then forward (switch) those Ethernet frames."
            br {}
            "That core logic revolves around:"
        }
        ol { class: "list-disc list-inside mb-4",
            li { "MAC addresses" }
            li { "the interface in which the frame arrives" }
            li { "and the interfaces out which the switch forwards the frame." }
        }

        p { class: "mb-4",
            "While true, that logic omits any consideration of virtual LANs (VLANs)."
            br {}
            "VLANs impact the switching logic for each frame because each VLAN acts as a subset of the switch ports in an
            Ethernet LAN."
            br {}

            strong {
                "Switches believe each Ethernet frame to be received in an identifiable VLAN, forwarded based on MAC table entries 
                for that VLAN, and forwarded out ports in that VLAN."
            }
            br {}
            "This chapter explores those concepts and others related to VLANs."
        }

        p { "As for the organization of the chapter, " }
        ol { class: "list-disc list-inside mb-4",
            li {
                "the first major section of the chapter explains the core concepts.
                These concepts include how VLANs work on a single switch, how to use VLAN trunking to create VLANs that span 
                across multiple switches, and how to forward traffic between VLANs using a router."
            }
            li {
                "The second major section shows how to configure VLANs and VLAN trunks: how to statically assign interfaces to a VLAN."
            }
            li {
                "The final major section discusses some issues that can arise when using VLANs and trunks and how to avoid those issues"
            }
        }
    }
}