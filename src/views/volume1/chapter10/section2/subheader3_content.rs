use dioxus::prelude::*;

use crate::{components::KeyTopic, utils::{h3_heading, text_command, TextCommandColor}};

#[component]
pub fn Content() -> Element {
    rsx! {
        p { class: "mb-4",
            "Even when the "
            {text_command("channel-group", TextCommandColor::Gold)}
            " commands have all been configured correctly, other configuration settings can prevent 
            a switch from using a physical port in an EtherChannel—even physical ports manually configured to be part of 
            the channel."
            br {}
            "The next topic examines those reasons."
        }

        {h3_heading("EtherChannel first check")}
        p { class: "mb-4",
            "First, before using a physical port in an EtherChannel, the switch compares the new physical
            port's configuration to the existing ports in the channel."
            br {}
            "That new physical interface's settings must be the same as the existing ports' settings; otherwise, the switch does 
            not add the new link to the list of approved and working interfaces in the channel."
            br {}
            "That is, the physical interface remains configured as part of the PortChannel, but it is not used as part of
            the channel, often being placed into some nonworking state."
        }

        KeyTopic {}
        p { "The list of items the switch checks includes the following:" }
        ol { class: "list-disc list-inside mb-4",
            li { "Speed" }
            li { "Duplex" }
            li { "Operational access or trunking state (all must be access, or all must be trunks)" }
            li { "If an access port, the access VLAN" }
            li {
                " If a trunk port, the allowed VLAN list (per the "
                {text_command("switchport trunk allowed", TextCommandColor::Gold)}
                " command)"
            }
            li { "If a trunk port, the native VLAN" }
            li { "STP interface settings (i.e. RSTP/STP interface cost and etc)" }
        
        }

        {h3_heading("EtherChannel second check")}
        p { class: "mb-4",
            strong { "In addition, switches check the settings on the neighboring switch." }
            br {}
            "To do so, the switches either use PAgP or LACP (if already in use) or use Cisco Discovery Protocol (CDP) if using
            manual configuration."
            br {}
            strong { "When checking neighbors, all settings except the STP settings must match." }
        }

        {h3_heading("Switch interfaces mismatch settings")}
        p { class: "mb-4",
            "As an example, SW1 and SW2 again use two links in one EtherChannel from Figure 10-7."
            br {}
            "Before configuring the EtherChannel, SW1's G0/2 was given a different RSTP port cost than G0/1."
            br {}
            "Example 10-6 picks up the story just after configuring the correct "
            {text_command("channel-group", TextCommandColor::Gold)}
            " commands, when the switch is deciding whether to use G0/1 and G0/2 in this."
        }

        p { class: "mb-4", "See Example 10-6 in volume 1 on page 252." }

        p { class: "mb-4",
            "The messages at the top of the example specifically state what the switch does when
            determining whether the interface settings match."
            br {}
            "In this case, SW1 detects the different STP costs."
            br {}
            "SW1 does not use G0/1, does not use G0/2, and even places them into an errdisabled state."
            br {}
            "The switch also puts the PortChannel into err-disabled state."
            br {}
            "As a result, the PortChannel is not operational, and the physical interfaces are also not operational."
        }

        p { class: "mb-4",
            "To solve this problem, you must reconfigure the physical interfaces to use the same STP settings."
            br {}
            "In addition, the PortChannel and physical interfaces must be "
            {text_command("shutdown", TextCommandColor::Gold)}
            ", and then "
            {text_command("no shutdown", TextCommandColor::Gold)}
            ", to recover from the  err-disabled state."
            br {}
            "(Note that when a switch applies the "
            {text_command("shutdown", TextCommandColor::Gold)}
            " and "
            {text_command("no shutdown", TextCommandColor::Gold)}
            " commands to a PortChannel, it applies those same commands to the physical interfaces, as well; so, just do the "
            {text_command("shutdown/no shutdown", TextCommandColor::Gold)}
            " on the PortChannel interface.)"
        }

        {h3_heading("RECAP")}
        ul { class: "list-disc list-inside",
            li {
                "Before adding a physical port in an EtherChannel, the switch compares the new physical port's configuration to the 
                existing ports in the channel. If not approved, it remains configured as part of the PortChannel but it will not
                be used and be put in a nonworking state."
            }
            li {
                "In addition, switches check the settings on the neighboring switch. When checking neighbors, all settings except 
                the STP settings must match, using dynamic EtherChannel protocols or CDP if using manual configuration. "
            }
            li {
                "If a switch interfaces have different settings in an EtherChannel, the switch does not use them and places them
                into an err-disabled state as well as the PortChannel interface."
                " As a result, the PortChannel is not operational, and the  physical interfaces are also not operational."
            }
            li {
                "To recover from the err-disable state, just use the "
                {text_command("shutdown/no shutdown", TextCommandColor::Gold)}
                " command on the PortChannel interface and the commands will also be applied automatically on the physical 
                interfaces members of the channel."
            }
        }
    }
}