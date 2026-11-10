use dioxus::prelude::*;

use crate::utils::h3_heading;

#[component]
pub fn Content() -> Element {
    rsx! {
        p { class: "mb-4",
            strong {
                "Encrypting data obscures it from view while it is traveling over a public or untrusted network."
            }
            br {}
            "The intended recipient should be able to decrypt the message and recover the original contents, but what if someone managed 
            to alter the contents along the way?"
            br {}
            "The recipient would have a very difficult time discovering that the original data had been modified."
        }

        p { class: "mb-4",
            strong {
                "A message integrity check (MIC) is a security tool that can protect against data tampering."
            }
            br {}
            "You can think of a MIC as a way for the sender to add a secret stamp inside the encrypted data frame."
            br {}
            "The stamp is based on the contents of the data bits to be transmitted."
            br {}
            "Once the recipient decrypts the frame, it can compare the secret stamp to its own idea of what the stamp should be, based on 
            the data bits that were received."
            br {}
            strong {
                "If the two stamps are identical, the recipient can safely assume that the data has not been tampered with."
            }
            br {}
            "Figure 28-5 shows the MIC process."
        }

        img {
            class: "mb-4 rounded-lg",
            alt: "Figure 28-5 Checking Message Integrity over a Wireless Network",
            loading: "lazy",
            src: asset!("/assets/static/v1p8c28s1sh3f28-5.png", AssetOptions::image().with_avif()),
        }

        {h3_heading("RECAP")}
        ol { class: "list-disc list-inside",
            li {
                "Encrypting data obscures it from view while it is traveling over a public or untrusted network."
            }
            li {
                "A message integrity check (MIC) is a security tool that can protect 802.11 frames against data tampering. 
                Similar to 802.3 Cyclic Redundancy Check (CRC) used to detect transmission errors."
            }
        }
    }
}