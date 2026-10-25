use dioxus::prelude::*;

use crate::utils::h3_heading;

#[component]
pub fn Content() -> Element {
    rsx! {
        p { class: "mb-4",
            "An AP can be configured to act as a bridge to form a single wireless link from one LAN to another over a long distance."
            br {}
            strong {
                "Outdoor bridged links are commonly used for connectivity between buildings or between cities."
            }
        }

        {h3_heading("Point-to-point bridged link")}
        p { class: "mb-4",
            strong {
                "If the LANs at two locations need to be bridged, a point-to-point bridged link can be used."
            }
            br {}
            "One AP configured in bridge mode is needed on each end of the wireless link."
            br {}
            strong {
                "Special purpose antennas are normally used with the bridges to focus their signals in one direction—toward the antenna 
                of the AP at the far end of the link."
            }
            br {}
            "This maximizes the link distance, as shown in Figure 26-12."
        }

        img {
            class: "mb-4 rounded-lg",
            alt: "Figure 26-12 Point-to-Point Outdoor Bridge",
            loading: "lazy",
            src: asset!("/assets/static/v1p8c26s3sh3f26-12.png", AssetOptions::image().with_avif()),
        }

        {h3_heading("Point-to-multipoint bridged link")}
        p { class: "mb-4",
            "Sometimes the LANs at multiple sites need to be bridged together."
            br {}
            "A point-to-multipoint bridged link allows a central site to be bridged to several other sites."
            br {}
            strong {
                "The central site bridge is connected to an omnidirectional antenna, such that its signal is transmitted equally in 
            all directions so that it can reach the other sites simultaneously."
            }
            br {}
            "The bridges at each of the other sites can be connected to a directional antenna aimed at the central site."
            br {}
            "Figure 26-13 shows the point-to-multipoint scenario."
        }

        img {
            class: "mb-4 rounded-lg",
            alt: "Figure 26-13 Point-to-Multipoint Outdoor Bridge",
            loading: "lazy",
            src: asset!("/assets/static/v1p8c26s3sh3f26-13.png", AssetOptions::image().with_avif()),
        }

        {h3_heading("RECAP")}
        ol { class: "list-disc list-inside",
            li {
                "An AP can be configured to act as a bridge to form a single wireless link from one LAN to another over a long distance."
            }
            li {
                "Outdoor bridged links are commonly used for connectivity between buildings or between cities."
            }
            li {
                "If the LANs at two locations need to be bridged, a point-to-point bridged link can be used."
            }
            li {
                "A point-to-multipoint bridged link allows a central site to be bridged to several other sites."
            }
        }
    }
}