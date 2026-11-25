use dioxus::prelude::*;

#[component]
pub fn ChapterIntroductionContent() -> Element {
    rsx! {
        p { class: "mb-4",
            "As you know by now, wireless networks are complex."
            br {}
            "Many technologies and protocols work behind the scenes to give end users a stable, yet mobile, connection to a wired 
            network infrastructure."
            br {}
            "From the user's perspective, a wireless connection should seem no different than a wired connection."
            br {}
            "A wired connection can give users a sense of security; data traveling over a wire is probably not going to be overheard by 
            others."
            br {}
            strong {
                "A wireless connection is inherently different; data traveling over the air can be overheard by anyone within range."
            }
        }

        p { class: "mb-1",
            "Therefore, securing a wireless network becomes just as important as any other aspect."
            br {}
            "A comprehensive approach to wireless security focuses on the following areas:"
        }
        ol { class: "list-disc list-inside mb-4",
            li { "Identifying the endpoints of a wireless connection" }
            li { "Identifying the end user" }
            li { "Protecting the wireless data from eavesdroppers" }
            li { "Protecting the wireless data from tampering" }
        }

        p { class: "mb-4",
            "The identification process is performed through various authentication schemes."
            br {}
            "Protecting wireless data involves security functions like encryption and frame authentication."
        }

        p { class: "mb-4",
            "This chapter covers many of the methods you can use to secure a wireless network."
            br {}
            strong {
                "Be warned: wireless security can be a confusing topic because it is filled with many acronyms."
            }
            br {}
            "Some of the acronyms rhyme like words from a children's book."
            br {}
            "In fact, this chapter is a story about WEP, PSK, TKIP, MIC, AES, EAP, EAP-FAST, EAP-TLS, LEAP, PEAP, WPA,
            WPA2, WPA3, CCMP, GCMP, and on and on it goes."
            br {}
            "When you finish with this chapter, though, you will come away with a clearer view of what these terms mean and how they all
            fit together."
            br {}
            "You might even be ready to configure a wireless LAN with effective security."
        }
    }
}