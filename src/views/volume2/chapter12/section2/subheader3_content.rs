use dioxus::prelude::*;

use crate::utils::h3_heading;

#[component]
pub fn Content() -> Element {
    rsx! {
        p { class: "mb-4",
            strong { "Every SNMP agent has its own Management Information Base." }
            br {}
            "The MIB defines variables whose values are set and updated by the agent."
            br {}
            "The MIB variables on the devices in the network enable the management software to monitor/control the network device."
        }

        {h3_heading("Object ID (OID)")}
        p { class: "mb-4",
            strong { "More formally, the MIB defines each variable as an object ID (OID)." }
            br {}
            "On most devices, the MIB then organizes the OIDs based in part on RFC standards, and in part with vendorproprietary 
            variables."
            br {}
            strong {
                "The MIB organizes all the variables into a hierarchy of OIDs, usually shown as a tree."
            }
            br {}
            "Each node in the tree can be described based on the tree structure sequence, either by name or by number."
            br {}
            "Figure 12-11 shows a small part of the tree structure of an MIB that happens to be part of the Cisco-proprietary part 
            of the MIB."
        }

        img {
            class: "mb-4 rounded-lg",
            loading: "lazy",
            alt: "Figure 12-11 Management Information Base (MIB)",
            src: asset!("/assets/static/v2p3c12s2sh3f12-11.png", AssetOptions::image().with_avif()),
        }

        {h3_heading("Working with MIB")}
        p { class: "mb-4",
            strong {
                "Working directly with an MIB, with long variable names and numbers, can be a bit of a
                challenge, so NMS software typically hides the complexity of the MIB variable numbering and names."
            }
            br {}
            "However, to get a sense for the variable names, Figure 12-11 shows the tree structure for two variables, with the 
            variable names being the long string of numbers shown at the bottom of the figure."
            br {}
            strong { "Working with those numbers and the tree structure can be difficult at best." }
            br {}
            "As a result, most people manage their networks using an NMS such as Cisco Prime."
            br {}
            strong {
                "For perspective, you could use an SNMP manager and type MIB variable 1.3.6.1.4.1.9.2.1.58.0 and click a button to get 
                that variable, to see the current CPU usage percentage from a Cisco router."
            }
            br {}
            "However, most users of an NMS would much prefer to ignore those details and have a simple graphical interface to ask 
            for the same information, never having to know that 1.3.6.1.4.9.2.1.58.0 represents the router CPU utilization MIB
            variable."
        }

        {h3_heading("RECAP")}
        ol { class: "list-disc list-inside",
            li {
                "Every SNMP agent has its own Management Information Base. 
                The MIB defines variables whose values are set and updated by the agent."
            }
            li { "More formally, the MIB defines each variable as an object ID (OID)." }
            li {
                "On most devices, the MIB then organizes the OIDs based in part on RFC standards, and in part with 
                vendorproprietary variables.  The MIB organizes all the variables into a hierarchy of OIDs, usually shown as a tree."
            }
            li {
                "Each node in the tree can be described based on the tree structure sequence, either by name or by number."
            }
            li {
                "Working directly with an MIB, with long variable names and numbers, can be difficult at best. 
                As a result, most people manage their networks using an NMS such as Cisco Prime."
            }
        }
    }
}