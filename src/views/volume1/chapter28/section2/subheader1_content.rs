use dioxus::prelude::*;

use crate::utils::h3_heading;

#[component]
pub fn Content() -> Element {
    rsx! {
        p { class: "mb-1",
            "The original 802.11 standard offered only two choices to authenticate a client:"
        }
        ol { class: "list-disc list-inside mb-4",
            li { "open authentication" }
            li { "and WEP." }
        }

        p { class: "mb-4",
            "Open authentication is true to its name; it offers open access to a WLAN."
            br {}
            "The only requirement is that a client must use an 802.11 authentication request before it attempts to associate with an AP."
            br {}
            "No other credentials are needed."
        }

        p { class: "mb-4",
            "When would you want to use open authentication?"
            br {}
            "After all, it does not sound very secure because it is not."
            br {}
            "With no challenge, any 802.11 client may authenticate to access the network."
            br {}
            "That is, in fact, "
            strong {
                "the whole purpose of open authentication—to validate that a client is a valid 802.11 device by authenticating 
                the wireless hardware and the protocol."
            }
            br {}
            "Authenticating the user's identity is handled as a true security process through other means."
        }

        p { class: "mb-4",
            "You have probably seen a WLAN with open authentication when you have visited a public location."
            br {}
            "If any client screening is used at all, it comes in the form of web authentication."
            br {}
            "A client can associate right away but must open a web browser to see and accept the terms for
            use and enter basic credentials."
            br {}
            "From that point, network access is opened up for the client."
            br {}
            "Most client operating systems flag such networks to warn you that your wireless data will not be secured in any way if you 
            join."
        }

        {h3_heading("RECAP")}
        ol { class: "list-disc list-inside",
            li {
                "The original 802.11 standard offered only two choices to authenticate a client: open authentication and WEP."
            }
            li { "Open authentication is true to its name; it offers open access to a WLAN." }
            li {
                "The whole purpose of open authentication—to validate that a client is a valid 802.11 device by authenticating the 
                wireless hardware and the protocol."
            }
            li {
                "Most client operating systems flag WLANs with open authenticaiton to warn you that your wireless data will not be 
                secured in any way if you join."
            }
        }
    }
}