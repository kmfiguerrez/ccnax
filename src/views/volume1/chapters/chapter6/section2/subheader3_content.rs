use dioxus::prelude::*;

use crate::{
    components::ConfigChecklist, utils::{TextCommandColor, h3_heading, text_command}
};

#[component]
pub fn Content() -> Element {
    rsx! {
        p { class: "mb-4",
            "The switch can also use Dynamic Host Configuration Protocol (DHCP) to dynamically learn
            its IPv4 settings."
            br {}
            "Basically, all you have to do is tell the switch to use DHCP on the interface
            and enable the interface."
            br {}
            "Assuming that DHCP works in this network, the switch will learn all its settings."
            br {}
            "The following list details the steps, again assuming the use of interface VLAN 1,
            with Example 6-8 that follows showing an example:"
        }

        ConfigChecklist {}
        ol { class: "mb-4",
            // Step 1
            li {
                span { class: "text-sky-500 font-semibold mr-4", "Step 1." }
                "Enter VLAN 1 configuration mode using the "
                {text_command("interface vlan 1", TextCommandColor::Gold)}
                " global configuration command, and enable the interface using the "
                {text_command("no shutdown", TextCommandColor::Gold)}
                " command as necessary."
            }
            // Step 2
            li {
                span { class: "text-sky-500 font-semibold mr-4", "Step 2." }
                "Assign an IP address and mask using the "
                {text_command("ip address dhcp", TextCommandColor::Gold)}
                "  interface subcommand."
            }
        }

        img {
            class: "mb-4 rounded-lg",
            alt: "Example 6-8 Switch Dynamic IP Address Configuration with DHCP",
            loading: "lazy",
            src: asset!("/assets/static/v1p2c6s2sh3ex6-8.png", AssetOptions::image().with_avif()),
        }

        {h3_heading("RECAP")}
        ol { class: "list-disc list-inside",
            li {
                "The switch can also use Dynamic Host Configuration Protocol (DHCP) to dynamically learn its IPv4 settings."
            }
        }
    }
}