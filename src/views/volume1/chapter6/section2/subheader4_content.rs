use dioxus::prelude::*;

use crate::utils::{TextCommandColor, h3_heading, text_command};

#[component]
pub fn Content() -> Element {
    rsx! {
        p { "The switch IPv4 configuration can be checked in several places." }
        ol { class: "list-disc list-inside",
            li {
                "First, you can always look at the current configuration using the "
                {text_command("show running-config", TextCommandColor::Gold)}
                " command."
            }
            li {
                "Second, you can look at the IP address and mask information using the "
                {text_command("show interfaces vlan", TextCommandColor::Gold)}
                i { " x" }
                " command, which
                shows detailed status information about the VLAN interface in VLAN x."
            }
            li {
                "Finally, if using DHCP, use the "
                {text_command("show dhcp lease", TextCommandColor::Gold)}
                " command to see the (temporarily) leased IP address and
                other parameters."
            }
        }
        p { class: "mb-4",
            "(Note that the switch does not store the DHCP-learned IP configuration in the running-config file.)"
            br {}
            "Example 6-9 shows sample output from these commands to match the configuration in Example 6-8."
        }

        img {
            class: "mb-4 rounded-lg",
            alt: "Example 6-9 Verifying DHCP-Learned Information on a Switch",
            loading: "lazy",
            src: asset!("/assets/static/v1p2c6s2sh4ex6-9.png", AssetOptions::image().with_avif()),
        }

        {h3_heading("Virtual Interface Status")}
        p { class: "mb-4",
            "The output of the "
            {text_command("show interfaces vlan 1", TextCommandColor::Gold)}
            " command lists two very important details related
            to switch IP addressing."
            br {}
            "First, this "
            {text_command("show", TextCommandColor::Gold)}
            " command lists the interface status of the VLAN 1 interface—in this case, “up and up.”"
            br {}
            "If the VLAN 1 interface is not up, the switch cannot use its IP address to send and receive management traffic."
            br {}
            "Notably, if you forget to issue the "
            {text_command("no shutdown", TextCommandColor::Gold)}
            " command, the VLAN 1 interface remains in its default shutdown 
            state and is listed as “administratively down” in the show command output."
        }

        p { class: "mb-4",
            "Second, note that the output lists the interface's IP address on the third line."
            br {}
            "If you statically configure the IP address, as in Example 6-7, the IP address will always be listed; however,
            if you use DHCP and DHCP fails, the "
            {text_command("show interfaces vlan x", TextCommandColor::Gold)}
            " command will not list an IP
            address here."
            br {}
            "When DHCP works, you can see the IP address with the "
            {text_command("show interfaces vlan 1", TextCommandColor::Gold)}
            " command, but that output does not remind you whether the address is either statically
            configured or DHCP leased."
            br {}
            "So it does take a little extra effort to make sure you know whether the address is statically configured or 
            DHCP-learned on the VLAN interface."
        }

        {h3_heading("RECAP")}
        ol { class: "list-disc list-inside",
            li {
                "The switch IPv4 configuration can be checked using the commands: "
                {text_command("show running-config", TextCommandColor::Gold)}
                ", "
                {text_command("show interfaces vlan", TextCommandColor::Gold)}
                i { " x" }
                ", and "
                {text_command("show dhcp lease", TextCommandColor::Gold)}
            
            }
            li {
                "The output of "
                {text_command("show interfaces vlan", TextCommandColor::Gold)}
                i { " x" }
                " does not remind you whether the address is either statically configured or DHCP leased."
            }
        }
    }
}