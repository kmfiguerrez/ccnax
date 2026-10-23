use dioxus::prelude::*;

#[component]
pub fn SectionIntroductionContent() -> Element {
    rsx! {
        p { class: "mb-4",
            "Wireless APs can be configured to operate in noninfrastructure modes when a normal BSS cannot provide the functionality 
            that is needed."
            br {}
            "The following sections cover the most common modes."
        }
    }
}