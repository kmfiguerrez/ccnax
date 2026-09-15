use dioxus::prelude::*;

use crate::utils::h3_heading;

#[component]
pub fn Content() -> Element {
    rsx! {
        p { class: "mb-4",
            "STP/RSTP prevents three common problems in Ethernet LANs."
            br {}
            "All three problems occur as a side effect of one fact: without STP/RSTP, some Ethernet frames would loop around the 
            network for a long time (hours, days, literally forever if the LAN devices and links never failed)."
        }

        p { class: "mb-4",
            "Just one looping frame causes what is called a "
            i { "broadcast storm" }
            "."
            br {}
            "Broadcast storms happen when any kind of Ethernet frames—broadcast frames, multicast frames, or 
            unknown-destination unicast frames—loop around a LAN indefinitely."
            br {}
            "Broadcast storms can saturate all the links with copies of that one single frame, crowding out good frames, as 
            well as significantly impacting end-user device performance by making the PCs process too many broadcast frames."
        }
    }
}