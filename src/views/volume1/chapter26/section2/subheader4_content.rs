use dioxus::prelude::*;

use crate::utils::h3_heading;

#[component]
pub fn Content() -> Element {
    rsx! {
        p { class: "mb-4",
            "Usually a wireless network leverages APs for organization, control, and scalability."
            br {}
            "Sometimes that is not possible or convenient in an impromptu situation."
            br {}
            strong {
                "For example, two people who want to exchange electronic documents at a meeting might not be able to find
                a BSS available or might want to avoid having to authenticate to a production network."
            }
            br {}
            "In addition, "
            strong {
                "many personal printers have the capability to print documents wirelessly, without relying on a regular 
                BSS or AP"
            }
            "."
        }

        h3 { {h3_heading("Ad hoc network")} }
        p { class: "mb-4",
            "The 802.11 standard allows two or more wireless clients to communicate directly with each
            other, with no other means of network connectivity."
            br {}
            "This is known as an "
            i { "ad hoc" }
            " wireless network, or an "
            i { "independent basic service set" }
            " (IBSS), as shown in Figure 26-9."
            br {}
            "For this to work, one of the devices must take the lead and begin advertising a network name and the necessary radio 
            parameters, much like an AP would do."
            br {}
            "Any other device can then join as needed."
            br {}
            "IBSSs are meant to be organized in an impromptu, distributed fashion; therefore, they do not
            scale well beyond eight to ten devices."
        }

        img {
            class: "mb-4 rounded-lg",
            alt: "Figure 26-9 802.11 Independent Basic Service Set",
            loading: "lazy",
            src: asset!("/assets/static/v1p8c26s2sh4f26-9.png", AssetOptions::image().with_avif()),
        }

        {h3_heading("RECAP")}
        ol { class: "list-disc list-inside",
            li {
                "The 802.11 standard defines "
                i { "ad hoc" }
                " wireless network, or an "
                i { "independent basic service set" }
                " (IBSS) where  two or more wireless clients to communicate directly with each other, with no other means of 
                network connectivity."
            }
            li {
                "For ad hoc network to work, one of the devices must take the lead and begin advertising a network name and the 
                necessary radio parameters, much like an AP would do."
            }
            li {
                "IBSSs are meant to be organized in an impromptu, distributed fashion; therefore, they do not scale well beyond 
                eight to ten devices."
            }
        
        }
    }
}