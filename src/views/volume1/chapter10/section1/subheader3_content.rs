use dioxus::prelude::*;

use crate::{
    components::KeyTopic, utils::{h3_heading, text_command}
};

#[component]
pub fn Content() -> Element {
    rsx! {
        p { class: "mb-4",
            strong {
                "To support the idea of multiple spanning trees, whether one per VLAN or simply multiple as
                created with MSTP, the protocols must consider the VLANs and VLAN trunking."
            }
            br {}
            "(That's one reason why RSTP and MSTP now exist as part of the 802.1Q standard, which defines VLANs
            and VLAN trunking.)"
            br {}
            "To help make that work, the IEEE redefined the format of the original BID value to help make per-VLAN instances 
            of STP/RSTP become a reality."
        }

        {h3_heading("Redefining the BID")}
        p { class: "mb-4",
            "Originally, a switch's BID was formed by combining the switch's 2-byte priority and its 6-byte MAC address."
            br {}
            "The revised rules divide the original priority field into two separate
            fields, as shown in Figure 10-4: a 4-bit priority field and a 12-bit subfield called the "
            i { "system ID extension" }
            " (which represents the VLAN ID)."
        }

        KeyTopic {}
        img {
            class: "mb-4 rounded-lg",
            alt: "Figure 10-4 STP System ID Extension",
            loading: "lazy",
            src: asset!("/assets/static/v1p3c10s1sh3f10-4.png", AssetOptions::image().with_avif()),
        }

        {h3_heading("BID configurable part")}
        p { class: "mb-4",
            strong { "Cisco switches let you configure the BID, but only the priority part." }
            br {}
            "The switch fills in its universal (burned-in) MAC address as the system ID."
            br {}
            "It also plugs in the VLAN ID of a VLAN in the 12-bit system ID extension field; you cannot 
            change that behavior either."
            br {}
            strong { "The only part configurable by the network engineer is the 4-bit priority field." }
        }

        {h3_heading("Priority field value")}
        p { class: "mb-4",
            "However, configuring the number to put in the priority field may be one of the strangest
            things to configure on a Cisco router or switch."
            br {}
            "As shown at the top of Figure 10-4, the priority field was originally a 16-bit number, which represented a 
            decimal number from 0 to 65,535."
            br {}
            "Because of that history, the configuration command ("
            {text_command("spanning-tree vlan", text_command::TextCommandColor::Gold)}
            i { " vlan-id " }
            {text_command("priority", text_command::TextCommandColor::Gold)}
            i { " x" }
            ") requires a decimal 
            number between 0 and 65,535."
            br {}
            "But not just any number in that range will suffice; it must be a multiple of 4096, as emphasized in the help 
            text shown in Example 10-2."
        }

        img {
            class: "mb-4 rounded-lg",
            alt: "Example 10-2 Help Shows Requirements for Using Increments of 4096 for Priority",
            loading: "lazy",
            src: asset!("/assets/static/v1p3c10s1sh3ex10-2.png", AssetOptions::image().with_avif()),
        }

        p { class: "mb-4",
            "Table 10-3 lists all the configurable values for the STP/RSTP priority."
            br {}
            "However, do not worry about memorizing the values."
            br {}
            "Instead, the table lists the values to emphasize two points about the binary values: the first 4 bits in each value 
            differ, but the last 12 bits remain as 12 binary zeros."
        }

        img {
            class: "mb-4 rounded-lg",
            alt: "Table 10-3 STP/RSTP Configurable Priority Values",
            loading: "lazy",
            src: asset!("/assets/static/v1p3c10s1sh3t10-3.png", AssetOptions::image().with_avif()),
        }

        {h3_heading("Cisco primary and secondary root switches")}
        p { class: "mb-4",
            "Note that while you can set the priority to any of the 16 decimal values in Table 10-3, Cisco
            provides a convenient means to create a primary and secondary root switch concept without configuring 
            an actual number."
            br {}
            "In most LAN designs, only a small number of switches would be good candidates to ever be the root switch based on 
            where the switches sit within the topology."
            br {}
            "Think of the preferred switch as the primary switch and the next-best option
            as the secondary switch."
            br {}
            "Then, to configure those two switches to be the two most likely
            switches to be the root switch, simply configure"
        }

        ul { class: "pl-4 mb-4",
            li {
                {text_command("spanning-tree vlan", text_command::TextCommandColor::Gold)}
                i { " x " }
                {text_command("root primary", text_command::TextCommandColor::Gold)}
                " (on the switch that should be primary)"
            }
            li {
                {text_command("spanning-tree vlan", text_command::TextCommandColor::Gold)}
                i { " x " }
                {text_command("root secondary", text_command::TextCommandColor::Gold)}
                " (on the switch that should be secondary)"
            }
        }

        p { class: "mb-4",
            "These two commands cause the switch to make a choice of priority value but then store the
            chosen priority value in the "
            {text_command("spanning-tree vlan", text_command::TextCommandColor::Gold)}
            i { " x " }
            {text_command("priority", text_command::TextCommandColor::Gold)}
            i { " value" }
            " command."
            br {}
            strong { "The command with root primary or root secondary does not appear in the configuration." }
            br {}
            "When configuring root primary, the switch looks at the priority of the current root switch and chooses
            either (a) 24,576 or (b) 4096 less than the current root's priority (if the current root's priority
            is 24,576 or less) to the configuration instead."
            br {}
            "When configuring, "
            {text_command("root secondary", text_command::TextCommandColor::Gold)}
            " always results in that switch using a priority of 28,672, with the assumption 
            that the value will be less than other switches that use the default of 32,768, and higher than any switch configured 
            as "
            {text_command("root primary", text_command::TextCommandColor::Gold)}
            "."
        
        }

        {h3_heading("RECAP")}
        ol { class: "list-disc list-inside",
            li {
                "To support the idea of multiple spanning trees, whether one per VLAN or simply multiple as created with MSTP, 
                the protocols must consider the VLANs and VLAN trunking. That's one reason why RSTP and MSTP now exist as part 
                of the 802.1Q standard, which defines VLANs and VLAN trunking."
            }
            li {
                "To account for VLANs and VLAN trunking, the IEEE redefined the format of the original BID value to help make 
                per-VLAN instances of STP/RSTP become a reality."
            }
            li {
                "The revised rules for BID divide the original priority field into two 
                separate fields: priority field and a 12-bit subfield called the "
                i { "system ID extension" }
                " (which represents the VLAN ID)."
            }
            li { "Cisco switches let you configure the BID, but only the 4-bit priority field." }
            li {
                "The Cisco command with root primary or root secondary does not appear in the configuration."
            }
        }
    }
}