use dioxus::prelude::*;

#[component]
pub fn ChapterIntroductionContent() -> Element {
    rsx! {
        p { class: "mb-4",
            "Wireless communication usually involves a data exchange between two devices."
            br {}
            "A wireless LAN goes even further; many  devices can participate in sharing the medium for data exchanges."
            br {}
            strong {
                "Wireless LANs must transmit a signal over radio frequencies (RF) to move data from one device to another."
            }
            br {}
            "Transmitters and receivers can be fixed in consistent locations, or they can be mobile and free to move around."
            br {}
            "This chapter explains the topologies that can be used to control access to the wireless medium and provide data 
            exchange between devices."
        }
    }
}