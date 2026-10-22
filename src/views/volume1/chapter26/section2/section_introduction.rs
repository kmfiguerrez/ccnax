use dioxus::prelude::*;

use crate::{
    utils::h3_heading,
    components::GreenNote
};

#[component]
pub fn SectionIntroductionContent() -> Element {
    rsx! {
        p { class: "mb-4",
            strong {
                "Wireless communication takes place over free space through the use of radio frequency (RF) signals."
            }
            br {}
            "The theory behind RF signals can be complex, and is described further in the “RF Overview” section in this chapter."
            br {}
            "For now, just assume that one device, the transmitter, sends RF signals to another device, the receiver."
            br {}
            strong {
                "As Figure 26-1 shows, the transmitter can contact the receiver at any and all times, as long as both devices are tuned 
                to the same frequency (or channel) and use the same scheme to carry the data between them."
            }
            br {}
            "That all sounds simple, except that it is not really practical."
        }

        img {
            class: "mb-4 rounded-lg",
            alt: "Figure 26-1 Unidirectional Communication",
            loading: "lazy",
            src: asset!("/assets/static/v1p8c26s2f26-1.png", AssetOptions::image().with_avif()),
        }

        p { class: "mb-4",
            strong {
                "To fully leverage wireless communication, data should travel in both directions, as shown in Figure 26-2."
            }
            br {}
            "Sometimes Device A needs to send data to Device B, while Device B would like to take a turn to send at other times."
        }

        img {
            class: "mb-4 rounded-lg",
            alt: "Figure 26-2 Bidirectional Communication",
            loading: "lazy",
            src: asset!("/assets/static/v1p8c26s2f26-2.png", AssetOptions::image().with_avif()),
        }

        p { class: "mb-4",
            "Because the two devices are using the same channel, two phrases in the preceding sentence become vitally 
            important: "
            i { "take a turn and send at other times" }
            "."
            br {}
            "With wireless communication, if multiple signals are received at the same time, they can interfere with each other."
            br {}
            "The likelihood of interference increases as the number of wireless devices grows."
            br {}
            "For example, Figure 26-3 shows four devices tuned to the same channel and what might happen if some or all of them 
            transmit at the same time."
        }

        img {
            class: "mb-4 rounded-lg",
            alt: "Figure 26-3 Interference from Simultaneous Transmissions",
            loading: "lazy",
            src: asset!("/assets/static/v1p8c26s2f26-3.png", AssetOptions::image().with_avif()),
        }

        {h3_heading("Hubs network uses Half-Duplex")}
        p { class: "mb-4",
            "All this talk about waiting turns and avoiding interference might remind you of a traditional (nonswitched) Ethernet 
            LAN, where multiple hosts can connect to a shared media and share a common bandwidth."
            br {}
            strong {
                "To use the media effectively, all the hosts must operate in half-duplex mode so that they try to avoid colliding with 
                other transmissions already in progress."
            }
            br {}
            "The side effect is that no host can transmit and receive at the same time on a shared medium."
        }

        {h3_heading("Wireless LANs use Half-Duplex")}
        p { class: "mb-4",
            "A wireless LAN is similar."
            br {}
            strong {
                "Because multiple hosts can share the same channel, they also share the “airtime” or access to that channel at any 
                given time."
            }
            br {}
            "Therefore, to keep everything clean, only one device should transmit at any given time."
            br {}
            "To contend for use of the channel, devices based on the 802.11 standard have to determine whether the channel is 
            clear and available before transmitting anything."
        }

        GreenNote {
            p {
                strong { "NOTE" }
                " IEEE 802.11 WLANs are always half duplex because transmissions between stations use the same frequency or channel. 
                Only one station can transmit at any time; otherwise, collisions occur. To achieve full-duplex mode, one station's 
                transmission would have to occur on one frequency while it receives over a different frequency—much like full-duplex
                Ethernet links work. Although this is certainly possible and practical, the 802.11 standard
                does not permit full-duplex operation. Some amendments to the standard do provide a
                means for multiple devices to transmit on the same channel at the same time, but this is
                beyond the scope of this book."
            }
        }

        p { class: "mb-4",
            "At the most basic level, there is no inherent organization to a wireless medium or any inherent control over the 
            number of devices that can transmit and receive frames."
            br {}
            strong {
                "Any device that has a wireless network adapter can power up at  any time and try to communicate."
            }
            br {}
            "At a minimum, a wireless network should have a way to make sure that every device using a channel can support a common 
            set of parameters."
            br {}
            "Beyond that, there should be a way to control which devices (and users) are allowed to use the wireless medium and the 
            methods that are used to secure the wireless transmissions."
        }

        {h3_heading("RECAP")}
        ol { class: "list-disc list-inside",
            li {
                "Wireless communication takes place over free space through the use of radio frequency (RF) signals."
            }
            li {
                "The transmitter can contact the receiver at any and all times, as long as both devices are tuned to the same frequency 
                (or channel) and use the same scheme to carry the data between them."
            }
            li { "To fully leverage wireless communication, data should travel in both directions." }
            li {
                "Because multiple hosts can share the same channel, they also share the “airtime” or access to that channel at any 
                given time. Therefore, to keep everything clean, only one device should transmit at any given time."
            }
        }
    }
}