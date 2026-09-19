use dioxus::prelude::*;

use crate::utils::h3_heading;

#[component]
pub fn Content() -> Element {
    rsx! {
        p { class: "mb-4",
            "The final RSTP concept included here relates to some terms RSTP uses to refer to different
            types of ports and the links that connect to those ports."
        }

        {h3_heading("Point-to-point links/ports")}
        p { class: "mb-4",
            "To begin, consider the basic image in Figure 9-11."
            br {}
            "It shows several links between two switches."
            br {}
            strong {
                "RSTP considers these links to be point-to-point links and the ports connected to them to
                be point-to-point ports because the link connects exactly two devices (points)."
            }
        }

        {h3_heading("Two categories of p2p ports")}
        p { class: "mb-4",
            "RSTP further classifies point-to-point ports into two categories."
            br {}
            "Point-to-point ports that connect two switches are not at the edge of the network and are simply called "
            i { "point-to-point ports." }
            br {}
            "Ports that instead connect to a single endpoint device at the edge of the network, like a PC or server, are 
            called "
            i { "point-to-point edge ports" }
            ", or simply "
            i { "edge ports" }
            "."
            br {}
            "In Figure 9-11, SW3's switch port connected to a PC is an edge port."
        }

        img {
            class: "mb-4 rounded-lg",
            alt: "Figure 9-11 RSTP Link Types",
            loading: "lazy",
            src: asset!("/assets/static/v1p3c9s3sh5f9-11.png", AssetOptions::image().with_avif()),
        }

        {h3_heading("Share port")}
        p { class: "mb-4",
            strong { "Finally, RSTP defines the term shared to describe ports connected to a hub." }
            br {}
            strong {
                "The term shared comes from the fact that hubs create a shared Ethernet; hubs also force the attached
            switch port to use half-duplex logic."
            }
            br {}
            "RSTP assumes that all half-duplex ports may be connected to hubs, treating ports that use half duplex as shared ports."
            br {}
            "RSTP converges more slowly on shared ports as compared to all point-to-point ports."
        }

        {h3_heading("RECAP")}
        ol { class: "list-disc list-inside",
            li {
                "RSTP considers links between two switches as "
                i { "point-to-point links" }
                " and the ports connected to them to be "
                i { "point-to-point ports" }
                " because the link connects exactly two devices (points)."
            }
            li {
                "Point-to-point ports that connect two switches are not at the edge of the network and are simply called "
                i { "point-to-point ports." }
            }
            li {
                "Ports that instead connect to a single endpoint device at the edge of the network, like a PC or server, are 
                called "
                i { "point-to-point edge ports" }
                ", or simply "
                i { "edge ports" }
                "."
            }
            li { "RSTP defines the term shared to describe ports connected to a hub." }
        }
    }
}