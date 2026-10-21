use dioxus::prelude::*;

use crate::{utils::h3_heading, components::KeyTopic};

#[component]
pub fn Content() -> Element {
    rsx! {
        p { class: "mb-4",
            strong { "The NMS (Network Management Station) typically polls the SNMP agent on each device." }
            br {}
            "The NMS can notify the human user in front of the PC or send emails, texts, and so on to notify the network 
            operations staff of any issues identified by the data found by polling the devices."
            br {}
            strong {
                "You can even reconfigure the device through these SNMP variables in the MIB if you permit this level of
                control."
            }
        }

        {h3_heading("SNMP GET and SET messages")}
        p { class: "mb-4",
            "Specifically, the NMS uses the SNMP Get, GetNext, and GetBulk messages (together referenced simply as Get messages) 
            to ask for information from an agent."
            br {}
            "The NMS sends an SNMP Set message to write variables on the SNMP agent as a means to change the configuration of the 
            device."
            br {}
            strong {
                "These messages come in pairs, with, for instance, a Get Request asking the agent for the contents of a variable, and 
                the Get Response supplying that information."
            }
            br {}
            "Figure 12-9 shows an example of a typical flow, with the NMS using an SNMP Get to ask
            for the MIB variable that describes the status of a particular router interface."
        }

        KeyTopic {}
        img {
            class: "mb-4 rounded-lg",
            loading: "lazy",
            alt: "Figure 12-9 SNMP Get Request and Get Response Message Flow",
            src: asset!("/assets/static/v2p3c12s2sh1f12-9.png", AssetOptions::image().with_avif()),
        }

        p { class: "mb-4",
            "SNMP permits much flexibility in how you monitor variables in the MIB."
            br {}
            strong {
                "Most commonly, a network administrator gathers and stores statistics over time using the NMS."
            }
            br {}
            "The NMS, with the stored data, can then analyze various statistical facts such as averages, minimums,
            and maximums."
            br {}
            strong {
                "To be proactive, administrators can set thresholds for certain key variables, telling the NMS to send a notification 
            (email, text, and so on) when a threshold is passed."
            }
        }

        {h3_heading("RECAP")}
        ol { class: "list-disc list-inside",
            li {
                "The NMS (Network Management Station) typically polls the SNMP agent on each device."
                " The NMS can notify users or network staff of any issues identified by the data found by polling the devices 
                via emails, text, and so on."
            }
            li {
                "The NMS uses SNMP GET and SET messages to obtain information and to reconfigure devices respectively."
            
            }
            li {
                "Most commonly, a network administrator gathers and stores statistics over time using the NMS. 
                To be proactive, administrators can set thresholds for certain key variables, telling the NMS to send a notification 
                (email, text, and so on) when a threshold is passed."
            
            }
        }
    }
}