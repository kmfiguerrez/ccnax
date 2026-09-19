use dioxus::prelude::*;

use crate::{
    components::KeyTopic,
    utils::h3_heading
};

#[component]
pub fn Content() -> Element {
    rsx! {
        p { class: "mb-4",
            "The depth of the previous example does not point out all details of RSTP, of course; however, the example does show 
            enough details to discuss RSTP states and internal processes."
        }

        p { class: "mb-4",
            "Both STP and RSTP use port states, but with some differences."
            br {}
            "First, RSTP keeps both the learning and forwarding states as compared with STP, for the same purposes."
            br {}
            strong { "However, RSTP does not even define a listening state, finding it unnecessary." }
            br {}
            "Finally, RSTP renames the blocking state to the discarding state and redefines its use slightly."
        }

        {h3_heading("RSTP discarding state")}
        p { class: "mb-4",
            strong {
                "RSTP uses the discarding state for what STP defines as two states: disabled state and
                blocking state."
            }
            br {}
            "Blocking should be somewhat obvious by now: the interface can work physically, but STP/RSTP chooses to not 
            forward traffic to avoid loops."
            br {}
            strong { "STP's disabled state simply meant that the interface was administratively disabled." }
            br {}
            "RSTP just combines those into a single discarding state."
            br {}
            "Table 9-10 shows the list of STP and RSTP states for comparison purposes."
        }

        KeyTopic {}
        img {
            class: "mb-4 rounded-lg",
            alt: "Table 9-10 Port States Compared: STP and RSTP",
            loading: "lazy",
            src: asset!("/assets/static/v1p3c9s3sh3t9-10.png", AssetOptions::image().with_avif()),
        }

        {h3_heading("STP processes")}
        p { class: "mb-4",
            "RSTP also changes some processes and message content (compared to STP) to speed convergence."
            br {}
            "For example, STP waits for a time (forward delay) in both listening and learning states."
            br {}
            strong {
                "The reason for this delay in STP is that, at the same time, the switches have all been told
                to time out their MAC table entries."
            }
            br {}
            strong { "When the topology changes, the existing MAC table entries may actually cause a loop." }
            br {}
            strong {
                "With STP, the switches all tell each other (with BPDU messages) that the topology has changed and to time out any 
                MAC table entries using the forward delay timer."
            }
            br {}
            "This removes the entries, which is good, but it causes the need to wait
            in both listening and learning state for forward delay time (default 15 seconds each)."
        }

        {h3_heading("RSTP processes")}
        p { class: "mb-4",
            strong { "RSTP, to converge more quickly, avoids relying on timers." }
            br {}
            "RSTP switches tell each other (using messages) that the topology has changed."
            br {}
            "Those messages also direct neighboring switches to flush the contents of their MAC tables in a way that removes 
            all the potentially loop-causing entries, without a wait."
            br {}
            "As a result, RSTP creates more scenarios in which a formerly discarding port can immediately transition to a 
            forwarding state, without waiting, and without using the learning state, as shown in the example in Figure 9-9."
        }

        {h3_heading("RECAP")}
        ol { class: "list-disc list-inside",
            li { "RSTP does not define a listening state, finding it unnecessary." }
            li {
                "RSTP uses the discarding state for what STP defines as two states: disabled state and
                blocking state."
            }
            li {
                "The reason for the "
                i { "forward delay time" }
                " in STP is that, at the same time, the switches have all been told
                to time out their MAC table entries."
            }
            li { "When the topology changes, the existing MAC table entries may actually cause a loop." }
            li {
                "With STP, the switches all tell each other (with BPDU messages) that the topology has changed and to time out 
                any MAC table entries using the forward delay timer."
            }
            li { "RSTP, to converge more quickly, avoids relying on timers." }
        }
    }
}