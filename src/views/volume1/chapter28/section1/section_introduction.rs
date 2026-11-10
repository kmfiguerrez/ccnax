use dioxus::prelude::*;

use crate::utils::h3_heading;

#[component]
pub fn SectionIntroductionContent() -> Element {
    rsx! {
        p { class: "mb-4",
            "In the previous chapters of this book, you learned about wireless clients forming associations with wireless access points 
            (APs) and passing data back and forth across the air."
        }

        p { class: "mb-4",
            "As long as all clients and APs conform to the 802.11 standard, they can all coexist—even on the same channel."
            br {}
            "Not every 802.11 device is friendly and trustworthy, however."
            br {}
            "Sometimes it is easy to forget that transmitted frames do not just go directly from the sender to the
            receiver, as in a wired or switched connection."
            br {}
            strong {
                "Instead, they travel according to the transmitter's antenna pattern, potentially reaching any receiver that is within range."
            }
        }

        p { class: "mb-4",
            "Consider the scenario in Figure 28-1."
            br {}
            "The wireless client opens a session with some remote entity and shares a confidential password."
            br {}
            "Because two untrusted users are also located within range of the client's signal, they may also learn the password by 
            capturing frames that have been sent on the channel."
            br {}
            strong {
                "The convenience of wireless communication also makes it easy for transmissions to be overheard and exploited by malicious 
                users."
            }
        }

        p { class: "mb-4",
            "If data is sent through open space, how can it be secured so that it stays private and intact?"
            br {}
            "The 802.11 standard offers a framework of wireless security mechanisms that can be used to add trust, privacy, and 
            integrity to a wireless network."
            br {}
            "The following sections give an overview of the wireless security framework."
        }

        img {
            class: "mb-4 rounded-lg",
            alt: "Figure 28-1 Wireless Transmissions Reaching Unintended Recipients",
            loading: "lazy",
            src: asset!("/assets/static/v1p8c28s1f28-1.png", AssetOptions::image().with_avif()),
        }

        {h3_heading("RECAP")}
        ol { class: "list-disc list-inside",
            li {
                "It is easy to forget that transmitted 802.11 frames do not just go directly from the sender to the receiver, as in a wired or 
                switched connection."
            }
            li {
                "802.11 frames travel according to the transmitter's antenna pattern, potentially reaching any receiver that is within 
                range."
            }
            li {
                "The convenience of wireless communication also makes it easy for transmissions to be overheard and exploited by 
                malicious users."
            }
        }
    }
}