use dioxus::prelude::*;

#[component]
pub fn SectionIntroductionContent() -> Element {
    rsx! {
        p { class: "mb-4",
            "You can use many different methods to authenticate wireless clients as they try to associate with the network."
            br {}
            "The methods have been introduced over time and have evolved as security weaknesses have been exposed and wireless hardware 
            has advanced."
            br {}
            "This section covers the most common authentication methods you might encounter."
        }
    }
}