use dioxus::prelude::*;

#[component]
pub fn SectionIntroductionContent() -> Element {
    rsx! {
        p {
            "Now that you have seen some of the ways to configure switch interfaces, the rest of the
            chapter takes a closer look at how to verify the interfaces work correctly."
            br {}
            "This section also
            looks at those more unusual cases in which the interface is working but not working well, as
            revealed by different interface status codes and statistics."
        }
    }
}