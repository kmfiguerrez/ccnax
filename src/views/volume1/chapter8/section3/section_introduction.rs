use dioxus::prelude::*;

#[component]
pub fn SectionIntroductionContent() -> Element {
    rsx! {
        p {
            "A switch's data plane forwarding processes depend in part on VLANs and VLAN trunking."
            br {}
            "This final section of the chapter focuses on issues related to VLANs and VLAN trunks that
            could prevent LAN switching from working properly, focusing on a few items not yet discussed in the chapter."
            br {}
            "In particular, this section examines these steps an engineer can take to avoid issues:"
        }
        ol { class: "list-disc list-inside",
            li {
                span { class: "text-sky-500 font-semibold mr-1", "Step 1." }
                " Confirm that all VLANs are both defined and active."
            }
            li {
                span { class: "text-sky-500 font-semibold mr-1", "Step 2." }
                " Check the allowed VLAN lists on both ends of each trunk to ensure that all
                VLANs intended to be used are included."
            }
            li {
                span { class: "text-sky-500 font-semibold mr-1", "Step 3." }
                " Check for incorrect trunk configuration settings that result in one switch operating as a trunk, with the 
                neighboring switch not operating as a trunk."
            }
            li {
                span { class: "text-sky-500 font-semibold mr-1", "Step 4." }
                " Check the native VLAN settings on both ends of the trunk to ensure the settings match."
            }
        }
    }
}