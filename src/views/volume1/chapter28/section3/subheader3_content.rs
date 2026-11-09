use dioxus::prelude::*;

#[component]
pub fn Content() -> Element {
    rsx! {
        p { class: "mb-1",
            "The Galois/Counter Mode Protocol (GCMP) is a robust authenticated encryption suite that is more secure and more efficient 
            than CCMP."
            br {}
            "GCMP consists of two algorithms:"
        }
        ol { class: "list-disc list-inside mb-4",
            li { "AES counter mode encryption" }
            li { "Galois Message Authentication Code (GMAC) used as a message integrity check (MIC)" }
        }

        p { class: "mb-4", "GCMP is used in WPA3, which is described in the following section." }
    }
}