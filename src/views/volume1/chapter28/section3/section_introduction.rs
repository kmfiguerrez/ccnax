use dioxus::prelude::*;

#[component]
pub fn SectionIntroductionContent() -> Element {
    rsx! {
        p { class: "mb-4",
            "The original 802.11 standard supported only one method to secure wireless data from eavesdroppers: WEP."
            br {}
            "As you have learned in this chapter, WEP has been compromised, deprecated, and can no longer be recommended."
            br {}
            "What other options are available to encrypt data and protect its integrity as it travels through free space?"
        }
    }
}