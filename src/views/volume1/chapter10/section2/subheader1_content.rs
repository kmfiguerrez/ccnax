use dioxus::prelude::*;

use crate::{
    components::{KeyTopic, ConfigChecklist, RedNote}, utils::{h3_heading, text_command, TextCommandColor}
};

#[component]
pub fn Content() -> Element {
    rsx! {
        p { class: "mb-4",
            "To configure a Layer 2 EtherChannel so that all the ports always attempt to be part of the
            channel, simply add the correct "
            {text_command("channel-group", TextCommandColor::Gold)}
            " interface configuration subcommand to each physical
            interface, on each switch, all with the "
            {text_command("on", TextCommandColor::Gold)}
            " keyword, and all with the same number."
            br {}
            "The "
            {text_command("on", TextCommandColor::Gold)}
            " keyword tells the switches to place a physical interface into an EtherChannel, and the number identifies 
            the PortChannel interface number that the interface should be a part of."
        }

        {h3_heading("Terms synonyms")}
        p {
            "Before getting into the configuration and verification, however, you need to start using
            three terms as synonyms: "
            i { "EtherChannel" }
            ", "
            i { "PortChannel" }
            ", and "
            i { "Channel-group" }
            "."
            br {}
            "Oddly, IOS uses the "
            {text_command("channel-group", TextCommandColor::Gold)}
            " configuration command, but then to display its status, IOS uses the "
            {text_command("show etherchannel", TextCommandColor::Gold)}
            " command."
            br {}
            "Then the output of this "
            {text_command("show", TextCommandColor::Gold)}
            " command refers to neither an “EtherChannel” nor a “Channel-group,” instead using the 
            term “PortChannel.”"
            br {}
            "So, pay close attention to these three terms in the example."
        }

        KeyTopic {}
        ConfigChecklist {}
        p { "To configure an EtherChannel manually, follow these steps:" }
        ol { class: "mb-4 pl-1",
            li {
                span { class: "text-blue-500 mr-4", "Step 1." }
                "Add the "
                {text_command("channel-group", TextCommandColor::Gold)}
                i { " number " }
                {text_command("mode on", TextCommandColor::Gold)}
                " command in interface configuration mode under each physical interface that should be in the channel to 
                add it to the channel."
            }
            li {
                span { class: "text-blue-500 mr-4", "Step 2." }
                "Use the same number for all commands on the same switch, but the channelgroup number on the neighboring switch can differ."
            }
        }

        p { class: "mb-4",
            "Example 10-4 shows a simple example, with two links between switches SW1 and SW2, as
            shown in Figure 10-6."
            br {}
            "The configuration shows SW1's two interfaces placed into channelgroup 1, with two show commands to follow."
        }

        img {
            class: "mb-4 rounded-lg",
            alt: "Figure 10-6 Sample LAN Used in EtherChannel Example",
            loading: "lazy",
            src: asset!("/assets/static/v1p3c10s2sh1f10-6.png", AssetOptions::image().with_avif()),
        }

        p { class: "mb-4", "See Example 10-4 in volume 1 on page 248." }

        {h3_heading("The show spanning-tree command")}
        p { class: "mb-4",
            "Take a few moments to look at the output in the two "
            {text_command("show", TextCommandColor::Gold)}
            " commands in the example, as well."
            br {}
            "First, the "
            {text_command("show spanning-tree", TextCommandColor::Gold)}
            " command lists Po1, short for PortChannel1, as an interface."
            br {}
            "This interface exists because of the "
            {text_command("channel-group", TextCommandColor::Gold)}
            " commands using the 1 parameter."
            br {}
            strong {
                "STP no longer operates on physical interfaces Fa0/14 and Fa0/15, instead
                operating on the PortChannel1 interface, so only that interface is listed in the output."
            }
        }

        {h3_heading("The show etherchannel command")}
        p { class: "mb-4",
            "Next, note the output of the "
            {text_command("show etherchannel 1 summary", TextCommandColor::Gold)}
            " command."
            br {}
            "It lists as a heading “Port-channel,” with Po1 below it."
            br {}
            "It also lists both Fa0/14 and Fa0/15 in the list of ports, with a (P) beside each."
            br {}
            "Per the legend, the P means that the ports are bundled in the port channel, which is a code that means these 
            ports have passed all the configuration checks and are valid to be included in the channel."
        }

        RedNote {
            p {
                strong { "NOTE" }
                " In Packet Tracer version 8.2.2.0400."
                br {}
                " You cannot specified the channel group for the verification etherchannel "
                {text_command("show", TextCommandColor::Black)}
                " command."
            }
        }

        {h3_heading("RECAP")}
        ol { class: "list-disc list-inside",
            li {
                "Before getting into the configuration and verification, however, you need to start using
                three terms as synonyms: "
                i { "EtherChannel" }
                ", "
                i { "PortChannel" }
                ", and "
                i { "Channel-group" }
                "."
            }
            li {
                "To configure a Layer 2 EtherChannel so that all the ports always attempt to be part of the
                channel, simply add the correct "
                {text_command("channel-group", TextCommandColor::Gold)}
                " interface configuration subcommand to each physical
                interface, on each switch, all with the "
                {text_command("on", TextCommandColor::Gold)}
                " keyword, and all with the same number."
            }
            li {
                "The "
                {text_command("on", TextCommandColor::Gold)}
                " keyword tells the switches to place a physical interface into an EtherChannel, and the number identifies 
                the PortChannel interface number that the interface should be a part of."
            }
            li {
                "The non-matching interfaces will be put in a notconnect or up/down state."
                " They remain configured as part of the PortChannel but it will not be used and be put in a nonworking state."
            }
            li {
                "In addition, switches check the settings on the neighboring switch, using Cisco Discovery Protocol (CDP) 
                if using manual configuration. When checking neighbors, all settings except the STP settings must match."
            }
        }
    }
}