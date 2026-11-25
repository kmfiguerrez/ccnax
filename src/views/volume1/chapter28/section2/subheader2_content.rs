use dioxus::prelude::*;

use crate::utils::h3_heading;

#[component]
pub fn Content() -> Element {
    rsx! {
        p { class: "mb-4",
            "As you might expect, open authentication offers nothing that can obscure or encrypt the data being sent between a client 
            and an AP."
            br {}
            "As an alternative, "
            strong {
                "The 802.11 standard has traditionally defined Wired Equivalent Privacy (WEP) as a method to make a 
                wireless link more like or equivalent to a wired connection."
            }
        }

        {h3_heading("RC4 cipher algorithm")}
        p { class: "mb-4",
            strong {
                "WEP uses the RC4 cipher algorithm to make every wireless data frame private and hidden from eavesdroppers."
            }
            br {}
            "The same algorithm encrypts data at the sender and decrypts it at the receiver."
            br {}
            strong {
                "The algorithm uses a string of bits as a key, commonly called a WEP key, to derive other encryption keys—one per wireless 
                frame."
            }
            br {}
            "As long as the sender and receiver have an identical key, one can decrypt what the other encrypts."
        }

        {h3_heading("WEP shared-key authentication")}
        p { class: "mb-4",
            strong { "WEP is known as a shared-key security method." }
            br {}
            "The same key must be shared between the sender and receiver ahead of time, so that each can derive other mutually agreeable 
            encryption keys."
            br {}
            "In fact, every potential client and AP must share the same key ahead of time so that any client can associate with the AP."
        }

        p { class: "mb-4",
            strong {
                "The WEP key can also be used as an optional authentication method as well as an encryption tool."
            }
            br {}
            "Unless a client can use the correct WEP key, it cannot associate with an AP."
            br {}
            "The AP tests the client's knowledge of the WEP key by sending it a random challenge phrase."
            br {}
            "The client encrypts the challenge phrase with WEP and returns the result to the AP."
            br {}
            "The AP can compare the client's encryption with its own to see whether the two WEP keys yield identical results."
        }

        p { class: "mb-4",
            strong {
                "WEP keys can be either 40 or 104 bits long, represented by a string of 10 or 26 hex digits."
            }
            br {}
            "As a rule of thumb, longer keys offer more unique bits for the algorithm, resulting in more robust encryption."
            br {}
            "Except in WEP's case, that is."
            br {}
            "Because WEP was defined in the original 802.11 standard in 1999, every wireless adapter was built with encryption hardware 
            specific to WEP."
            br {}
            "In 2001, a number of weaknesses were discovered and revealed, so work began to find better wireless security methods."
            br {}
            strong { "By 2004, the 802.11i amendment was ratified and WEP was officially deprecated." }
            br {}
            "Both WEP encryption and WEP shared-key authentication are widely considered to be weak methods to secure a wireless LAN."
        }

        {h3_heading("RECAP")}
        ol { class: "list-disc list-inside",
            li {
                "The 802.11 standard defined Wired Equivalent Privacy (WEP) in 1999 as a method to make a 
                wireless link more like or equivalent to a wired connection."
            }
            li {
                "WEP uses the RC4 cipher algorithm to make every wireless data frame private and hidden from eavesdroppers."
            }
            li {
                "RC4 uses a string of bits as a key, commonly called a WEP key, to derive other encryption keys—one per wireless frame."
            }
            li { "WEP is known as a shared-key security method." }
            li {
                "The WEP key can also be used as an optional authentication method as well as an encryption tool."
            }
            li {
                "WEP keys can be either 40 or 104 bits long, represented by a string of 10 or 26 hex digits."
            }
            li { "By 2004, the 802.11i amendment was ratified and WEP was officially deprecated." }
            li {
                "Both WEP encryption and WEP shared-key authentication are widely considered to be weak methods to secure a wireless LAN."
            }
        }
    }
}