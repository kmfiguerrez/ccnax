use dioxus::prelude::*;

#[component]
pub fn ChapterIntroductionContent() -> Element {
    rsx! {
        p { class: "mb-4",
            "In Chapter 26, “Fundamentals of Wireless Networks,” you learned about how a single access
            point (AP) can provide a basic service set (BSS) for a cell area and how multiple APs can
            be connected to form an extended service set (ESS) for a larger network."
            br {}
            "In this chapter, you learn more about different approaches or architectures that allow APs to be networked
             together for an enterprise."
            br {}
            "You also learn how some architectures are more scalable than others and how to manage each type of wireless network 
            architecture."
        }

        p { class: "mb-4",
            "As you work through this chapter, think about how each architecture can be applied to specific environments—how easy 
            it would be to manage, deploy, and troubleshoot the network, how the APs can be controlled, and how data would move 
            through the network."
        }
    }
}