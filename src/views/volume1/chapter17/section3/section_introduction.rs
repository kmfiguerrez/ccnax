use dioxus::prelude::*;

use crate::utils::h3_heading;

#[component]
pub fn SectionIntroductionContent() -> Element {
    rsx! {
        p { class: "mb-4",
            "When Layer 3 switches use SVIs, the physical interfaces on the switches act like they always
            have: as Layer 2 interfaces."
            br {}
            "That is, the physical interfaces receive Ethernet frames."
            br {}
            "The switch learns the source MAC address of the frame, and the switch forwards the frame based
            on the destination MAC address."
            br {}
            "To perform routing, any Ethernet frames destined for any of the SVI interface MAC addresses trigger the processing of 
            the Layer 2 switching logic, resulting in normal routing actions like stripping data-link headers, making a routing 
            decision, and so on."
        }

        {h3_heading("Routed ports")}
        p { class: "mb-1",
            "Alternately, the Layer 3 switch configuration can make a physical port act like a router interface instead of a 
            switch interface."
            br {}
            "To do so, the switch configuration makes that port a routed port."
            br {}
            strong { "On a routed port, the switch does not perform Layer 2 switching logic on that frame." }
            br {}
            "Instead, frames arriving in a routed port trigger the Layer 3 routing logic, including"
        }
        ol { class: "list-decimal list-inside pl-1 mb-4",
            li { "Stripping off the incoming frame's Ethernet data-link header/trailer" }
            li {
                "Making a Layer 3 forwarding decision by comparing the destination IP address to the
                IP routing table"
            }
            li { "Adding a new Ethernet data-link header/trailer to the packet" }
            li { "Forwarding the packet, encapsulated in a new frame" }
        }

        {h3_heading("Layer 3 EtherChannel")}
        p { class: "mb-4",
            "This third major section of the chapter examines routed interfaces as configured on Cisco
            Layer 3 switches, but with a particular goal in mind: to also discuss Layer 3 EtherChannels."
            br {}
            strong {
                "The exam topics do not mention routed interfaces specifically, but the exam topics do mention L3 EtherChannels, 
                meaning Layer 3 EtherChannels."
            }
        }

        p { class: "mb-4",
            "You might recall that Chapter 10, “RSTP and EtherChannel Configuration,” discussed Layer 2 EtherChannels."
            br {}
            "Like Layer 2 EtherChannels, Layer 3 EtherChannels also treat multiple links as one link."
            br {}
            "Unlike Layer 2 EtherChannels, however, Layer 3 EtherChannels treat the channel as a "
            i { "routed port" }
            " instead of "
            i { "switched port" }
            "."
            br {}
            "So this section first looks at routed ports on Cisco Layer 3 switches and then discusses Layer 3 EtherChannels."
        }

        {h3_heading("RECAP")}
        ol { class: "list-disc list-inside",
            li {
                "The Layer 3 switch configuration can make a physical port act like a router interface instead of a switch interface 
                and it's called routed port."
            }
            li {
                "On a routed port, the switch does not perform Layer 2 switching logic on that frame."
                " Instead, frames arriving in a routed port trigger the Layer 3 routing logic."
            }
            li {
                "Like Layer 2 EtherChannels, Layer 3 EtherChannels also treat multiple links as one link."
            }

            li {
                "Unlike Layer 2 EtherChannels, however, Layer 3 EtherChannels treat the channel as a "
                i { "routed port" }
                " instead of "
                i { "switched port" }
                "."
            }
        }

    }
}