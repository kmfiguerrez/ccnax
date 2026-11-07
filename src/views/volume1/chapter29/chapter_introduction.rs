use dioxus::prelude::*;

#[component]
pub fn ChapterIntroductionContent() -> Element {
    rsx! {
        p { class: "mb-4",
            "In Chapters 26 through 28, you learned about the fundamentals of wireless networks."
            br {}
            "As a CCNA, you will also need to know how to apply that knowledge toward building a functioning network with APs and a 
            WLC. "
        }

        p { class: "mb-4",
            "In addition, based on the concepts you learned in Chapter 28, “Securing Wireless Networks,”
            you will be able to configure the WLAN to use WPA2-Personal."
        }
    }
}