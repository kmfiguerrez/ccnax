use dioxus::prelude::*;

use crate::{utils::h3_heading, components::KeyTopic};

#[component]
pub fn Content() -> Element {
    rsx! {
        p { class: "mb-1",
            "In addition to asking for information with Get commands and setting variables on agents
            with the Set command, "
            strong { "SNMP agents can initiate communications to the NMS" }
            "."
            br {}
            "These messages, generally called notifications, use two specific SNMP messages:"
        }
        ol { class: "list-disc list-inside mb-1",
            li { "Trap" }
            li { "Inform" }
        }
        p { class: "mb-4",
            "SNMP agents send a Trap or Inform SNMP message to the NMS to list the state of certain
            MIB variables when those variables reach a certain state."
        }

        p { class: "mb-4",
            "As an example of a Trap, suppose that Router 1's G0/0 interface fails, as shown at step 1 of Figure 12-10."
            br {}
            "With Traps configured, the router would send an SNMP Trap message to the NMS, with that Trap message noting the down 
            state of the G0/0 interface."
            br {}
            "Then, the NMS software can send a text message to the network support staff, pop up a window on the
            NMS screen, change the color of the correct router icon to red on the graphical interface, and so on."
        }

        KeyTopic {}
        img {
            class: "mb-4 rounded-lg",
            loading: "lazy",
            alt: "Figure 12-10 SNMP Trap Notification Process",
            src: asset!("/assets/static/v2p3c12s2sh2f12-10.png", AssetOptions::image().with_avif()),
        }

        {h3_heading("Trap messages")}
        p { class: "mb-4",
            strong {
                "SNMP Traps and Inform messages have the exact same purpose but differ in the protocol mechanisms."
            }
            br {}
            "SNMP Traps, available since the first version of SNMP from the late 1980s
            (SNMP Version 1, or SNMPv1), use a fire-and-forget process."
            br {}
            strong {
                "The SNMP agent sends the Trap to the IP address of the NMS, with UDP as the transport protocol as with all SNMP
                messages, and with no application layer error recovery."
            }
            br {}
            "If the Trap arrives, great; if it is lost in transit, it is lost."
        }

        {h3_heading("Inform messages")}
        p { class: "mb-4",
            strong { "Inform messages are like Trap messages but with reliability added." }
            br {}
            "Added to the protocol with SNMP Version 2  (SNMPv2), "
            strong { "Informs still use UDP but add application layer reliability." }
            br {}
            "The NMS must acknowledge receipt of the Inform with an SNMP Response message, or the
            SNMP agent will time out and resend the Inform."
        }

        {h3_heading("Traps and Informs Role today")}
        p { class: "mb-4",
            "Note that Traps and Informs both have a useful role today, and Traps are still frequently used."
            br {}
            "Both inform the NMS."
            br {}
            "Traps use less overhead on the agent, while Informs improve reliability of the messages but require a little more 
            overhead effort."
        }

        {h3_heading("RECAP")}
        ol { class: "list-disc list-inside",
            li {
                "SNMP agents can initiate communications to the NMS using two messages (called notifications): Trap and Inform."
            }
            li {
                "SNMP Traps and Inform messages have the exact same purpose but differ in the protocol mechanisms"
            }
            li {
                "SNMP Traps, available since the SNMPv1, use a fire-and-forget process-uses UDP as the transport protocol 
                but with no application layer error recovery."
            }
            li {
                "SNMP informs, available since the SNMPv2, still use UDP but with added application layer reliability. 
                The NMS must acknowledge receipt of the Inform with an SNMP Response message, or the SNMP agent will time out and 
                resend the Inform"
            }
        }
    }
}