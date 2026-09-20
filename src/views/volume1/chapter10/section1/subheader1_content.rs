use dioxus::prelude::*;

use crate::{
    components::KeyTopic,
    utils::h3_heading
};

#[component]
pub fn Content() -> Element {
    rsx! {
        {h3_heading("Before Switches and VLANs")}
        p { class: "mb-4",
            "The IEEE first standardized STP as the IEEE 802.1D standard, first published back in 1990."
            br {}
            "To put some perspective on that date, Cisco did not have a LAN switch product line at the
            time, and virtual LANs did not exist yet."
            br {}
            "Instead of multiple VLANs in a physical Ethernet LAN, the physical Ethernet LAN existed as one single 
            broadcast domain, with one instance of STP"
        }

        {h3_heading("Existence of Switches and VLANs")}
        p { class: "mb-4",
            "By the mid 1990s, VLANs had appeared on the scene, along with LAN switches."
            br {}
            "The emergence of VLANs posed a challenge for STP—the only type of STP available at the time—
            because STP defined a single common spanning tree (CST) topology for the entire LAN."
            br {}
            strong {
                "The IEEE needed an option to create multiple spanning trees so that traffic could be balanced
            across the available links, as shown in Figure 10-2."
            }
            br {}
            "With two different STP instances, SW3 could block on a different interface in each VLAN, as shown in the figure."
        }

        KeyTopic {}
        img {
            class: "mb-4 rounded-lg",
            alt: "Figure 10-2 Load Balancing with One Tree for VLAN 1 and Another for VLAN 2",
            loading: "lazy",
            src: asset!("/assets/static/v1p3c10s1sh1f10-2.png", AssetOptions::image().with_avif()),
        }

        {h3_heading("RECAP")}
        ol { class: "list-disc list-inside",
            li {
                "The IEEE first standardized STP as the IEEE 802.1D standard, first published back in 1990."
            }
            li { "STP defines a single common spanning tree (CST) topology for a LAN." }
        }

    }
}