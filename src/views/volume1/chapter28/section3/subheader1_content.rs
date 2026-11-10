use dioxus::prelude::*;

#[component]
pub fn Content() -> Element {
    rsx! {
        p { class: "mb-4",
            "During the time when WEP was embedded in wireless client and AP hardware, yet was known to be vulnerable, the Temporal Key 
            Integrity Protocol (TKIP) was developed."
        }

        p { class: "mb-1",
            "TKIP adds the following security features using legacy hardware and the underlying WEP encryption:"
        }
        ol { class: "list-disc list-inside mb-4",
            li {
                span { class: "font-bold mr-1", "MIC:" }
                "This efficient algorithm adds a hash value to each frame as a message integrity check to prevent tampering; commonly 
                called “Michael” as an informal reference to MIC."
            }
            li {
                span { class: "font-bold mr-1", "Time stamp:" }
                "A time stamp is added into the MIC to prevent replay attacks that attempt to reuse or replay frames that have already 
                been sent."
            }
            li {
                span { class: "font-bold mr-1", "Sender's MAC address:" }
                "The MIC also includes the sender's MAC address as evidence of the frame source."
            }
            li {
                span { class: "font-bold mr-1", "TKIP sequence counter:" }
                "This feature provides a record of frames sent by a unique MAC address, to prevent frames from being replayed as an 
                attack."
            }
            li {
                span { class: "font-bold mr-1", "Key mixing algorithm:" }
                "This algorithm computes a unique 128-bit WEP key for each frame."
            }
            li {
                span { class: "font-bold mr-1", "Longer initialization vector (IV):" }
                "The IV size is doubled from 24 to 48 bits, making it virtually impossible to exhaust all WEP keys by brute-force calculation."
            }
        }

        p { class: "mb-4",
            "TKIP became a reasonably secure stopgap security method, buying time until the 802.11i standard could be ratified."
            br {}
            strong {
                "Some attacks have been created against TKIP, so it, too, should be avoided if a better method is available."
            }
            br {}
            strong { "In fact, TKIP was deprecated in the 802.11-2012 standard." }
        }
    }
}