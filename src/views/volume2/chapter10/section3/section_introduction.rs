use dioxus::prelude::*;

#[component]
pub fn SectionIntroductionContent() -> Element {
    rsx! {
        p {
            "The following sections describe how to configure the three most common variations of
            NAT: static NAT, dynamic NAT, and PAT, along with the show and debug commands used
            to troubleshoot NAT."
        }
    }
}