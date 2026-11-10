use dioxus::prelude::*;

#[component]
pub fn Content() -> Element {
    rsx! {
        p { class: "mb-1",
            "The Counter/CBC-MAC Protocol (CCMP) is considered to be more secure than TKIP."
            br {}
            "CCMP consists of two algorithms:"
        }
        ol { class: "list-disc list-inside mb-4",
            li { "AES counter mode encryption" }
            li {
                "Cipher Block Chaining Message Authentication Code (CBC-MAC) used as a message integrity check (MIC)"
            }
        }

        p { class: "mb-4",
            "The Advanced Encryption Standard (AES) is the current encryption algorithm adopted by U.S. National Institute of Standards 
            and Technology (NIST) and the U.S. government, and widely used around the world."
            br {}
            "In other words, AES is open, publicly accessible, and represents the most secure encryption method available today."
        }

        p { class: "mb-4",
            strong {
                "Before CCMP can be used to secure a wireless network, the client devices and APs must support the AES counter mode and 
                CBC-MAC in hardware."
            }
            br {}
            "CCMP cannot be used on legacy devices that support only WEP or TKIP."
            br {}
            strong {
                "How can you know if a device supports CCMP?"
                "Look for the WPA2 designation, which is described in the following section."
            }
        }

    }
}