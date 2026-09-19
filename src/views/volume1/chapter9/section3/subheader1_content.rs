use dioxus::prelude::*;

use crate::{
    components::{KeyTopic, GreenNote},
    utils::h3_heading
};

#[component]
pub fn Content() -> Element {
    rsx! {
        p {
            "RSTP works just like STP in several ways, as discussed in the first major section of the chapter."
            br {}
            "To review:"
        }
        KeyTopic {}
        ul { class: "list-disc list-inside mb-4",
            li { "RSTP and STP elect the root switch using the same rules and tiebreakers." }
            li { "RSTP and STP switches select their root ports with the same rules." }
            li {
                "RSTP and STP elect designated ports on each LAN segment with the same rules and tiebreakers."
            }
            li {
                "RSTP and STP place each port in either forwarding or blocking state, although RSTP calls
                the blocking state the "
                i { "discarding state" }
                "."
            }
        }

        {h3_heading("Both RSTP and STP can be used in the same network")}
        p { class: "mb-4",
            "In fact, RSTP works so much like STP that they can both be used in the same network."
            br {}
            "RSTP and STP switches can be deployed in the same network, with RSTP features working
            in switches that support it and traditional STP features working in the switches that support
            only STP."
        }

        {h3_heading("Reason for the creation of RSTP")}
        p { class: "mb-4",
            "With all these similarities, you might be wondering why the IEEE bothered to create RSTP
            in the first place."
            br {}
            strong { "The overriding reason is convergence." }
            br {}
            "STP takes a relatively long time to converge (50 seconds with the default settings when all the wait times must be 
            followed)."
            br {}
            "RSTP improves network convergence when topology changes occur, usually converging within a few seconds 
            (or in slow conditions, in about 10 seconds)"
        }

        {h3_heading("RSTP's workaround against the timers")}
        p {
            "RSTP changes and adds to STP in ways that avoid waiting on STP timers, resulting in quick
            transitions from forwarding to discarding (blocking) state and vice versa."
            br {}
            "Specifically, RSTP, compared to STP, defines more cases in which the switch can avoid waiting for a timer to
            expire, such as the following:"
        }
        KeyTopic {}
        ul { class: "list-disc list-inside mb-4",
            li {
                "RSTP adds a mechanism by which a switch can replace its root port, without any waiting
                to reach a forwarding state (in some conditions)."
            }
            li {
                "RSTP adds a new mechanism to replace a designated port, without any waiting to reach a
                forwarding state (in some conditions)."
            }
            li {
                "RSTP and STP elect designated ports on each LAN segment with the same rules and tiebreakers."
            }
            li { "RSTP lowers waiting times for cases in which RSTP must wait for a timer." }
        }

        p { class: "mb-4",
            "For instance, imagine a failure case in which a link remains up, but for some reason, a nonroot switch stops 
            hearing the Hello BPDUs it had been hearing in the past."
            br {}
            "STP requires a switch to wait for MaxAge seconds, which STP defines based on 10 times the Hello timer, or
            20 seconds, by default."
            br {}
            "RSTP shortens this timer, defining MaxAge as three times the Hello timer."
            br {}
            "Additionally, "
            strong {
                "RSTP can send messages to the neighboring switch to inquire whether a
                problem has occurred rather than wait for timers."
            }
        }

        {h3_heading("Alternate and backup ports")}
        p { class: "mb-4",
            "The best way to get a sense for these mechanisms is to see how the RSTP alternate port and
            the backup port both work."
            br {}
            "RSTP uses the term alternate port to refer to a switch's other ports that could be used as the root port if the root 
            port ever fails."
            br {}
            "The backup port concept provides a backup port on the local switch for a designated port."
            br {}
            strong {
                "(Note that backup ports apply only to designs that use hubs, so they are unlikely to be useful today.)"
            }
            br {}
            "However, both are instructive about how RSTP works."
            br {}
            "Table 9-9 lists these RSTP port roles."
        }

        KeyTopic {}
        img {
            class: "mb-4 rounded-lg",
            alt: "Table 9-9 Port Roles in RSTP",
            loading: "lazy",
            src: asset!("/assets/static/v1p3c9s3sh1t9-9.png", AssetOptions::image().with_avif()),
        }

        {h3_heading("Other RSTP and STP difference")}
        p { class: "mb-4",
            "RSTP differs from STP in a few other ways as well."
            br {}
            "For instance, with STP, the root switch creates a Hello with all other switches, updating and forwarding the Hello."
            br {}
            strong { "With RSTP, each switch independently generates its own Hellos." }
            br {}
            strong {
                "Additionally, RSTP allows for queries between neighbors, rather than waiting on timers to expire, as a means to 
                avoid waiting to learn information."
            }
            br {}
            "These types of protocol changes help RSTP-based switches isolate what
            has changed in a network and react quickly to choose a net RSTP topology. "
        }

        {h3_heading("RECAP")}
        ol { class: "list-disc list-inside",
            li { "RSTP calls blocking state the discarding state." }
            li { "RSTP works so much like STP that they can both be used in the same network." }
            li { "The reason RSTP was created to replace STP is to improve convergence." }
            li {
                "RSTP improves network convergence when topology changes occur, usually converging within a few seconds 
                (or in slow conditions, in about 10 seconds)"
            }
            li {
                "RSTP can send messages to the neighboring switch to inquire whether a problem has occurred rather than wait 
                for timers."
            }
            li {
                "RSTP uses the term alternate port to refer to a switch's other ports that could be used as the root port if 
                the root port ever fails."
            }
            li {
                "The backup port concept provides a backup port on the local switch for a designated port."
                " Note that backup ports apply only to designs that use hubs, so they are unlikely to be useful today."
            }
            li { "With RSTP, each switch independently generates its own Hellos." }
        }
    }
}