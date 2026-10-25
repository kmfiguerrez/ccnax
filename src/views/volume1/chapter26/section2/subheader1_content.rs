use dioxus::prelude::*;

use crate::{components::{KeyTopic, GreenNote}, utils::h3_heading};

#[component]
pub fn Content() -> Element {
    rsx! {
        p { class: "mb-4",
            "Wireless service area is a closed group of mobile devices that forms around a fixed device; 
            before a device can participate, it must advertise its capabilities and then be granted permission to join."
            br {}
            strong { "The 802.11 standard calls this a basic service set (BSS)." }
            br {}
            "At the heart of every BSS is a wireless access point (AP), as shown in Figure 26-4."
            br {}
            strong {
                "The AP operates in infrastructure mode, which means it offers the services that are necessary to
                form the infrastructure of a wireless network."
            }
            br {}
            "The AP also establishes its BSS over a single wireless channel."
            br {}
            "The AP and the members of the BSS must all use the same channel to communicate properly."
        }

        {h3_heading("Basic Service Area (BSA)")}
        p { class: "mb-4",
            "Because the operation of a BSS hinges on the AP, the BSS is bounded by the area where the AP's signal is usable."
            br {}
            strong { "This is known as the basic service area (BSA) or cell." }
            br {}
            "In Figure 26-4, the cell is shown as a simple shaded circular area that centers around the AP itself."
            br {}
            strong {
                "Cells can have other shapes too, depending on the antenna that is connected to the AP and on the
            physical surroundings that might affect the AP's signals."
            }
        }

        {h3_heading("Basic Service Set ID (BSSID)")}
        p { class: "mb-4",
            strong {
                "The AP serves as a single point of contact for every device that wants to use the BSS."
            }
            br {}
            "It advertises the existence of the BSS so that devices can find it and try to join."
            br {}
            "To do that, the AP uses a unique BSS identifier (BSSID) that is based on the AP's own radio MAC address."
        }

        GreenNote {
            p {
                strong { "NOTE" }
                " Recall that wired Ethernet devices each have a unique MAC address to send frames
                from a source to a destination over a Layer 2 network. Wireless devices must also have
                unique MAC addresses to send wireless frames at Layer 2 over the air."
            }
        }

        KeyTopic {}
        img {
            class: "mb-4 rounded-lg",
            alt: "Figure 26-4 802.11 Basic Service Set",
            loading: "lazy",
            src: asset!("/assets/static/v1p8c26s2sh1f26-4.png", AssetOptions::image().with_avif()),
        }

        {h3_heading("Service Set ID (SSID)")}
        p { class: "mb-4",
            "In addition, the AP advertises the wireless network with a Service Set Identifier (SSID),
            which is a text string containing a logical name."
            br {}
            strong {
                "Think of the BSSID as a machine-readable name tag that uniquely identifies the BSS ambassador (the AP), and the SSID 
                as a nonunique, human-readable name tag that identifies the wireless service."
            }
        }

        {h3_heading("Association (Membership)")}
        p { class: "mb-4",
            "Membership with the BSS is called an "
            i { "association" }
            "."
            br {}
            "A wireless device must send an association request to the AP and the AP must either grant or deny the request."
            br {}
            strong { "Once associated, a device becomes a client, or an 802.11 station (STA), of the BSS." }
            br {}
            "What then?"
            br {}
            "As long as a wireless client remains associated with a BSS, most communications to and from the client
            must pass through the AP, as indicated in Figure 26-5."
            br {}
            strong {
                "By using the BSSID as a source or destination address, data frames can be relayed to or from the AP."
            }
        }

        {h3_heading("Why the use of an AP")}
        p { class: "mb-4",
            "You might be wondering why all client traffic has to traverse the AP at all."
            br {}
            "Why can two clients not simply transmit data frames directly to each other and bypass the middleman?"
            br {}
            "If clients are allowed to communicate directly, then the whole idea of organizing and managing a BSS is moot."
            br {}
            "By sending data through the AP first, the BSS remains stable and under control."
        }

        GreenNote {
            p {
                strong { "NOTE" }
                " Even though data frames are meant to pass through an AP, keep in mind that other devices in the same general area 
                that are listening on the same channel can overhear the transmissions. After all, wireless frames are not contained 
                within a wire that connects a device to an AP. Instead, the frames are freely available over the air to anyone that 
                is within range to receive them. If the frames are unencrypted, then anyone may inspect their contents. Only the 
                BSSID value contained within the frames indicates that the intended sender or recipient is the AP."
            }
        }

        img {
            class: "mb-4 rounded-lg",
            alt: "Figure 26-5 Traffic Flows Within a BSS",
            loading: "lazy",
            src: asset!("/assets/static/v1p8c26s2sh1f26-5.png", AssetOptions::image().with_avif()),
        }

        {h3_heading("RECAP")}
        ol { class: "list-disc list-inside",
            li {
                "The 802.11 standard defines basic service set (BSS) which is a wireless service provided by one AP to one or more 
                mobile devices to create a wireless network."
            }
            li {
                "The AP operates in infrastructure mode, which means it offers the services that are necessary to form the 
                infrastructure of a wireless network."
            }
            li {
                "At the heart of every BSS is a wireless access point (AP). The AP also establishes its BSS over a single wireless 
                channel. The AP and the members of the BSS must all use the same channel to communicate properly."
            }
            li { "Basic Service Area (BSA) or cell is the area where the AP's signal is usable." }
            li {
                "The AP serves as a single point of contact for every device that wants to use the BSS."
            }
            li {
                "An AP uses a unique BSS identifier (BSSID) that is based on the AP's own radio MAC address to advertise 
                the existence of the BSS so that devices can find it and try to join."
            }
            li {
                "In addition, the AP advertises the wireless network with a Service Set Identifier (SSID), which is a text string 
                containing a logical name."
            }
            li {
                "Think of the BSSID as a machine-readable name tag that uniquely identifies the BSS ambassador (the AP), and the 
                SSID as a nonunique, human-readable name tag that identifies the wireless service."
            }
        }

    }
}