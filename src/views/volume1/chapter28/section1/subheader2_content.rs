use dioxus::prelude::*;

use crate::utils::h3_heading;

#[component]
pub fn Content() -> Element {
    rsx! {
        p { class: "mb-4",
            "Suppose that the client in Figure 28-3 must authenticate before joining the wireless network."
            br {}
            strong {
                "It might also authenticate the AP and its management frames after it associates but before
                it is itself authenticated."
            }
            br {}
            "The client's relationship with the AP might become much more trusted, but data passing to and from the client is still 
            available to eavesdroppers on the same channel."
        }

        {h3_heading("Encrypting Data for Privacy")}
        p { class: "mb-4",
            strong {
                "To protect data privacy on a wireless network, the data should be encrypted for its journey
                through free space."
            }
            br {}
            "This is accomplished by encrypting the data payload in each wireless
            frame just prior to being transmitted, then decrypting it as it is received."
            br {}
            "The idea is to use an encryption method that the transmitter and receiver share, so the data can be encrypted and
            decrypted successfully."
        }

        {h3_heading("Encryption Keys")}
        p { class: "mb-4",
            strong {
                "In wireless networks, each WLAN may support only one authentication and encryption scheme, so all clients must use the same 
                encryption method when they associate."
            }
            br {}
            "You might think that having one encryption method in common would allow every client to eavesdrop on every other client."
            br {}
            "That is not necessarily the case because the AP should securely negotiate a unique encryption key to use for each associated 
            client."
        }

        p { class: "mb-4",
            strong {
                "Ideally, the AP and a client are the only two devices that have the encryption keys in common so that they can understand 
                each other's data."
            }
            br {}
            "No other device should know about or be able to use the same keys to eavesdrop and decrypt the data."
            br {}
            "In Figure 28-4, the client's confidential password information has been encrypted before being transmitted."
            br {}
            "The AP can decrypt it successfully before forwarding it onto the wired network, but other wireless devices cannot."
        }

        img {
            class: "mb-4 rounded-lg",
            alt: "Figure 28-4 Encrypting Wireless Data to Protect Data Privacy",
            loading: "lazy",
            src: asset!("/assets/static/v1p8c28s1sh2f28-4.png", AssetOptions::image().with_avif()),
        }

        p { class: "mb-4",
            strong {
                "The AP also maintains a “group key” that it uses when it needs to send encrypted data to all clients in its cell at one time."
            }
            br {}
            "Each of the associated clients uses the same group key to decrypt the data."
        }

        {h3_heading("RECAP")}
        ol { class: "list-disc list-inside",
            li {
                "After a client associates itself with the AP, it authenticates the AP and its management frames before the client 
                is authenticated."
            }
            li {
                "To protect data privacy on a wireless network, the data should be encrypted for its journey through free space."
            }
            li {
                "In wireless networks, each WLAN may support only one authentication and encryption scheme, so all clients must use the 
                same encryption method when they associate."
            }
            li {
                "An AP securely negotiates a unique encryption key to use for each associated client, so the AP and a client are the only 
                two devices that have the encryption keys in common."
            }
            li {
                "The AP also maintains a “group key” that it uses when it needs to send encrypted data to all clients in its cell at one 
                time."
            }
        }
    }
}