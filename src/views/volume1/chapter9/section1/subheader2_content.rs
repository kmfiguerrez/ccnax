use dioxus::prelude::*;

use crate::{
    components::GreenNote,
    utils::h3_heading
};

#[component]
pub fn Content() -> Element {
    rsx! {
        p { class: "mb-4",
            strong {
                "STP/RSTP prevents loops by placing each switch port in either a forwarding state or a blocking state."
            }
            br {}
            "Interfaces in the forwarding state act as normal, forwarding and receiving frames."
            br {}
            "However, interfaces in a blocking state do not process any frames except STP/RSTP messages 
            (and some other overhead messages)."
            br {}
            "Interfaces that block do not forward user frames, do not learn MAC addresses of received frames, and 
            do not process received user frames."
        }

        p { class: "mb-4",
            "Figure 9-2 shows a simple STP/RSTP tree that solves the problem shown in Figure 9-1 by
            placing one port on SW3 in the blocking state."
        }

        img {
            class: "mb-4 rounded-lg",
            alt: "Figure 9-2 What STP/RSTP Does: Blocks a Port to Break the Loop",
            loading: "lazy",
            src: asset!("/assets/static/v1p3c9s1sh2f9-2.png", AssetOptions::image().with_avif()),
        }

        p {
            "Now when Bob sends a broadcast frame, the frame does not loop. As shown in the steps in the figure:"
        }
        ul { class: "pl-1 mb-4",
            li {
                span { class: "text-blue-500 mr-4", "Step 1." }
                "Bob sends the frame to SW3."
            }
            li {
                span { class: "text-blue-500 mr-4", "Step 2." }
                "SW3 forwards the frame only to SW1, but not out Gi0/2 to SW2, because
                SW3's Gi0/2 interface is in a blocking state."
            }
            li {
                span { class: "text-blue-500 mr-4", "Step 3." }
                "SW1 floods the frame out both Fa0/11 and Gi0/1."
            }
            li {
                span { class: "text-blue-500 mr-4", "Step 4." }
                "SW2 floods the frame out Fa0/12 and Gi0/1."
            }
            li {
                span { class: "text-blue-500 mr-4", "Step 5." }
                "SW3 physically receives the frame, but it ignores the frame received from SW2
                because SW3's Gi0/2 interface is in a blocking state."
            }
        }

        p { class: "mb-4",
            "With the STP/RSTP topology in Figure 9-2, the switches simply do not use the link between
            SW2 and SW3 for traffic in this VLAN, which is the minor negative side effect of STP."
            br {}
            "However, if either of the other two links fails, STP/RSTP converges so that SW3 forwards
            instead of blocks on its Gi0/2 interface."
        }

        GreenNote {
            p {
                strong { "NOTE" }
                " The term "
                i { "STP convergence" }
                " refers to the process by which the switches collectively
                realize that something has changed in the LAN topology and determine whether they need
                to change which ports block and which ports forward."
            }
        }

        p { class: "mb-4",
            strong {
                "That completes the description of what STP/RSTP does, placing each port into either a forwarding or blocking state."
            }
            br {}
            "The more interesting question, and the one that takes a lot more
            work to understand, is how and why STP/RSTP makes its choices."
            br {}
            "How does STP/RSTP manage to make switches block or forward on each interface?"
            br {}
            "And how does it converge to change state from blocking to forwarding to take advantage of redundant links in 
            response to network outages?"
            br {}
            "The following pages answer these questions."
        }

        {h3_heading("RECAP")}
        ol { class: "list-disc list-inside",
            li {
                "STP/RSTP prevents loops by placing each switch port in either a forwarding state or a blocking state."
            }
            li { "Interfaces in the forwarding state act as normal, forwarding and receiving frames." }
            li {
                "Interfaces in a blocking state do not process any frames except STP/RSTP messages 
                (and some other overhead messages). Interfaces that block do not forward user frames, 
                do not learn MAC addresses of received frames, and do not process received user frames."
            }
            li {
                " The term "
                i { "STP convergence" }
                " refers to the process by which the switches collectively
                realize that something has changed in the LAN topology and determine whether they need
                to change which ports block and which ports forward."
            }
        }
    }
}