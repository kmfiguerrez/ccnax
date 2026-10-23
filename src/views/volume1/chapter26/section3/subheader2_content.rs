use dioxus::prelude::*;

use crate::utils::h3_heading;

#[component]
pub fn Content() -> Element {
    rsx! {
        p { class: "mb-4",
            "Suppose you have a device that supports a wired Ethernet link but is not capable of having a wireless connection."
            br {}
            "For example, some mobile medical devices might be designed with only a wired connection."
            br {}
            "While it is possible to plug the device into an Ethernet connection when needed, a wireless connection would be much 
            more practical."
            br {}
            "You can use a workgroup bridge (WGB) to connect the device's wired network adapter to a wireless network."
        }

        p { class: "mb-4",
            "Rather than providing a BSS for wireless service, a WGB becomes a wireless client of a BSS."
            br {}
            strong {
                "In effect, the WGB acts as an external wireless network adapter for a device that has none."
            }
            br {}
            "In Figure 26-11, an AP provides a BSS; Client A is a regular wireless client, while Client B is associated with the AP 
            through a WGB."
        }

        img {
            class: "mb-4 rounded-lg",
            alt: "Figure 26-11 Nonwireless Device Connecting Through a Workgroup Bridge",
            loading: "lazy",
            src: asset!("/assets/static/v1p8c26s3sh2f26-11.png", AssetOptions::image().with_avif()),
        }

        p { class: "mb-1", "You might encounter two types of workgroup bridges:" }
        ol { class: "list-disc list-inside mb-4",
            li {
                span { class: "font-bold mr-1", "Universal workgroup bridge (uWGB):" }
                "A single wired device can be bridged to a wireless network."
            }
            li {
                span { class: "font-bold mr-1", "Workgroup bridge (WGB):" }
                "A Cisco-proprietary implementation that allows multiple wired devices to be bridged to a wireless network."
            }
        }

        {h3_heading("RECAP")}
        ol { class: "list-disc list-inside",
            li {
                "You can use a workgroup bridge (WGB) to connect the device's wired network adapter to a wireless network. 
                In effect, the WGB acts as an external wireless network adapter for a device that has none."
            }
            li {
                "You might encounter two types of workgroup bridges: "
                i { "Universal workgroup bridge" }
                " (uWGB) and "
                i { "Workgroup Bridge" }
                " (WGB)."
            }
        }
    }
}