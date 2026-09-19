use dioxus::prelude::*;

use crate::{
    components::KeyTopic,
    utils::h3_heading
};

#[component]
pub fn Content() -> Element {
    rsx! {
        p { class: "mb-4",
            "The RSTP backup port role acts as yet another new RSTP port role as compared to STP."
            br {}
            "As a reminder, the RSTP alternate port role creates a way for RSTP to quickly replace a switch's root port."
            br {}
            "Similarly, the RSTP backup port role creates a way for RSTP to quickly replace a switch's designated port on 
            some LAN."
        }

        p { class: "mb-4",
            "The need for a backup port can be a bit confusing at first because the need for the backup
            port role only happens in designs that are a little unlikely today."
            br {}
            "The reason is that a design must use hubs, which then allows the possibility that one switch connects more 
            than one port to the same collision domain."
        }

        p { class: "mb-4",
            "Figure 9-10 shows an example."
            br {}
            "SW3 and SW4 both connect to the same hub."
            br {}
            "SW4's port F0/1 happens to win the election as designated port (DP)."
            br {}
            "The other port on SW4 that connects to the same collision domain, F0/2, acts as a backup port."
        }

        img {
            class: "mb-4 rounded-lg",
            alt: "Figure 9-10 RSTP Backup Port Example",
            loading: "lazy",
            src: asset!("/assets/static/v1p3c9s3sh4f9-10.png", AssetOptions::image().with_avif()),
        }

        p { class: "mb-4",
            "With a backup port, if the current designated port fails, SW4 can start using the backup
            port with rapid convergence."
            br {}
            "For instance, if SW4's F0/1 interface were to fail, SW4 could transition F0/2 to the designated port role, 
            without any delay in moving from discarding state to a forwarding state."
        }

        {h3_heading("RECAP")}
        ol { class: "list-disc list-inside",
            li {
                "As a reminder, the RSTP alternate port role creates a way for RSTP to quickly replace a switch's root port."
            }
            li {
                "Similarly, the RSTP backup port role creates a way for RSTP to quickly replace a switch's designated port on 
                some LAN."
            }
            li { "The need for the backup port role only happens in designs that use Hubs." }
        }
    }
}