use dioxus::prelude::*;

#[component]
pub fn SectionIntroductionContent() -> Element {
    rsx! {
        p { class: "mb-4",
            "Cisco switches do not require any configuration to work."
            br {}
            "You can purchase Cisco switches, install devices with the correct cabling, turn on the switches, and they work."
            br {}
            "You would never need to configure the switch, and it would work fine, even if you interconnected
            switches, until you needed more than one VLAN."
            br {}
            "But if you want to use VLANs—and most enterprise networks do—you need to add some configuration."
        }

        p { "This section separates the VLAN configuration details into two major sections." }
        ol { class: "list-disc list-inside",
            li {
                "The first section looks at how to configure static access interfaces: switch interfaces configured to be in
                one VLAN only, therefore not using VLAN trunking."
            }
            li { "The second part shows how to configure interfaces that do use VLAN trunking." }
        }
    }
}