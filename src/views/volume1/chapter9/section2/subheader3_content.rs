use dioxus::prelude::*;

use crate::{
    components::KeyTopic,
    utils::h3_heading
};

#[component]
pub fn Content() -> Element {
    rsx! {
        p { class: "mb-4",
            strong { "STP uses the idea of roles and states." }
            br {}
            "Roles, like root port and designated port, relate to how STP analyzes the LAN topology."
            br {}
            "States, like forwarding and blocking, tell a switch whether to send or receive frames."
            br {}
            strong {
                "When STP converges, a switch chooses new port roles, and the port roles determine the state (forwarding or blocking)."
            }
        }

        p { class: "mb-4",
            strong {
                "Switches using STP can simply move immediately from forwarding to blocking state, but
                they must take extra time to transition from blocking state to forwarding state."
            }
            br {}
            "For instance, when switch SW3 in Figure 9-7 formerly used port G0/1 as its RP (a role), that port was in
            a forwarding state."
            br {}
            "After convergence, G0/1 might be neither an RP nor DP; the switch can immediately move that port to a blocking state."
        }

        {h3_heading("Two STP intermediate interface states")}
        p {
            "However, when a port that formerly blocked needs to transition to forwarding, the switch
            first puts the port through two intermediate interface states."
            br {}
            "These temporary STP states help prevent temporary loops:"
        }
        KeyTopic {}
        ol { class: "list-disc list-inside mb-4",
            li {
                span { class: "font-bold", "Listening:" }
                "  Like the blocking state, the interface does not forward frames. The switch
                removes old stale (unused) MAC table entries for which no frames are received from each
                MAC address during this period. These stale MAC table entries could be the cause of the
                temporary loops."
            }
            li {
                span { class: "font-bold", "Learning:" }
                "  Interfaces in this state still do not forward frames, but the switch begins to
                learn the MAC addresses of frames received on the interface."
            }
        }

        {h3_heading("Time length from blocking to forwading transition")}
        p { class: "mb-4",
            "STP moves an interface from blocking to listening, then to learning, and then to forwarding state."
            br {}
            "STP leaves the interface in each interim state for a time equal to the "
            i { "forward delay timer" }
            ", which defaults to 15 seconds."
            br {}
            "As a result, a convergence event that causes an interface
            to change from blocking to forwarding requires 30 seconds to transition from blocking to
            forwarding."
            br {}
            "In addition, a switch might have to wait MaxAge seconds (default 20 seconds)
            before even choosing to move an interface from blocking to forwarding state."
        }

        p { class: "mb-4",
            "For example, follow what happens with an initial STP topology as shown in Figures 9-3
            through 9-6, with the SW1-to-SW3 link failing as shown in Figure 9-7."
            br {}
            "If SW1 simply quit sending Hello messages to SW3, but the link between the two did not fail, SW3 would wait
            MaxAge seconds before reacting (20 seconds is the default)."
            br {}
            "SW3 would actually quickly choose its ports' STP roles, but then wait 15 seconds each in listening and learning 
            states on interface Gi0/2, resulting in a 50-second convergence delay."
        }

        KeyTopic {}
        p { "Table 9-8 summarizes spanning tree's various interface states for easier review." }
        img {
            class: "mb-4 rounded-lg",
            alt: "Table 9-8 IEEE STP (Not RSTP) States",
            loading: "lazy",
            src: asset!("/assets/static/v1p3c9s2sh3t9-8.png", AssetOptions::image().with_avif()),
        }

        {h3_heading("RECAP")}
        ol { class: "list-disc list-inside",
            li { "STP uses the idea of roles and states." }
            li {
                "Roles, like "
                i { "root port" }
                " and "
                i { "designated port" }
                ", relate to how STP analyzes the LAN topology."
            }
            li {
                "States, like "
                i { "forwarding" }
                " and "
                i { "blocking" }
                ", tell a switch whether to send or receive frames."
            }
            li {
                "When STP converges, a switch chooses new port roles, and the port roles determine the state (forwarding or blocking)."
            }
            li {
                "When a port is neither and RP nor DP, the switch can immediately move that port to a blocking state."
            }
            li {
                "Switches using STP can simply move immediately from forwarding to blocking state, but they must take extra time 
                to transition from blocking state to forwarding state."
            }
            li {
                "When a port that formerly blocked needs to transition to forwarding, the switch first puts the port through two 
                intermediate interface states (listening and learning). STP moves an interface from blocking to listening, then to 
                learning, and then to forwarding state."
            }
            li {
                "STP leaves the interface in each interim state for a time equal to the "
                i { "forward delay timer" }
                ", which defaults to 15 seconds."
                " As a result, a convergence event that causes an interface to change from blocking to forwarding requires 
                30 seconds to transition from blocking to forwarding."
            }
            li {
                "In addition, a switch might have to wait MaxAge seconds (default 20 seconds) before even choosing to move an 
                interface from blocking to forwarding state, resulting in a 50-second convergence delay."
            }
        }
    }
}