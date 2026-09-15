use dioxus::prelude::*;

use crate::{
    components::{KeyTopic, RedNote},
    utils::{TextCommandColor, h3_heading, text_command}
};

#[component]
pub fn Content() -> Element {
    rsx! {
        p { class: "mb-4",
            "A Cisco switch can forward traffic for all defined and active VLANs."
            br {}
            "However, a particular VLAN trunk may not forward traffic for a defined and active VLAN for a variety of other
            reasons."
            br {}
            strong {
                "You should learn how to identify which VLANs a particular trunk port currently
                supports and the reasons why the switch might not be forwarding frames for a VLAN on
                that trunk port."
            }
        }

        p {
            "The first category in this step can be easily done using the "
            {text_command("show interfaces", TextCommandColor::Gold)}
            i { " interface-id " }
            {text_command("trunk", TextCommandColor::Gold)}
            " command, which only lists information about currently operational trunks."
            br {}
            "The best place to begin with this command is the last section of output, which lists the VLANs whose 
            traffic will be forwarded over the trunk."
            br {}
            "Any VLANs that make it to this final list of VLANs in the command output meet the following criteria:"
        }
        ol { class: "list-disc list-inside mb-4",
            li {
                "The VLAN has not been removed from the allowed VLAN list on the trunk 
                (as configured with the "
                {text_command("switchport trunk allowed vlan", TextCommandColor::Gold)}
                " interface subcommand)."
            }
            li {
                "The VLAN exists and is active on the local switch (as seen in the "
                {text_command("show vlan", TextCommandColor::Gold)}
                " command)."
            }
            li {
                "The VLAN has not been VTP-pruned from the trunk."
                " (Because this book attempts to ignore VTP as much as possible, this section assumes that VTP is not used and this 
                feature has no impact on any  trunks.)"
                " The trunk is in an STP forwarding state in that VLAN (as also seen in the show spanning-tree vlan vlan-id command)."
            }
        }

        RedNote {
            p {
                strong { "NOTE" }
                " In Packet Tracer version 8.2.2.0400."
                br {}
                "The "
                {text_command("show interfaces", TextCommandColor::Black)}
                i { " interface-id " }
                {text_command("trunk", TextCommandColor::Black)}
                " is not available."
            }
        }

        {h3_heading("Limit VLANs supported on the trunk link")}
        p { class: "mb-4",
            "The "
            {text_command("switchport trunk allowed vlan", TextCommandColor::Gold)}
            " interface subcommand gives the network engineer a
            method to administratively limit the VLANs whose traffic uses a trunk."
            br {}
            "If the engineer wants all defined VLANs to be supported on a trunk, the engineer simply does not configure this
            command."
            br {}
            "If the engineer would like to limit the trunk to support a subset of the VLANs
            known to the switch, however, the engineer can add one or more "
            {text_command("switchport trunk allowed vlan", TextCommandColor::Gold)}
            " interface subcommands."
        }

        p { class: "mb-4",
            "For instance, in a switch that has configured VLANs 1 through 100, but no others, by
            default the switch would allow traffic in all 100 VLANs."
            br {}
            "However, the trunk interface command "
            {text_command("switchport trunk allowed vlan 1-60", TextCommandColor::Gold)}
            " would limit the trunk to forward traffic for
            VLANs 1 through 60, but not the rest of the VLANs."
            br {}
            "Example 8-13 shows a sample of the command output from the "
            {text_command("show interfaces trunk", TextCommandColor::Gold)}
            " command, which confirms the first list of VLAN IDs now lists VLANs 1-60."
            br {}
            "Without the "
            {text_command("switchport trunk allowed vlan", TextCommandColor::Gold)}
            " command, the first list would have included VLANs 1-4094."
        }

        img {
            class: "mb-4 rounded-lg",
            alt: "Example 8-13 Allowed VLAN List and List of Active VLANs",
            loading: "lazy",
            src: asset!("/assets/static/v1p3c8s3sh3ex8-13.png", AssetOptions::image().with_avif()),
        }

        {h3_heading("Analyzing the output of show interfaces trunk command")}
        p { class: "mb-4",
            "The output of the "
            {text_command("show interfaces trunk", TextCommandColor::Gold)}
            " command creates three separate lists of VLANs,
            each under a separate heading."
            br {}
            strong {
                "These three lists show a progression of reasons why a VLAN is not forwarded over a trunk."
            }
            br {}
            "Table 8-4 summarizes the headings that precede each list and the reasons why a switch chooses to include or not 
            include a VLAN in each list."
            br {}
            "For instance, in Example 8-13, VLAN 60 has been shut down, and VLAN 59 happens to be in an STP blocking state."
            br {}
            "(Chapter 9, “Spanning Tree Protocol Concepts,” has more information about STP.)"
        }

        KeyTopic {}
        img {
            class: "mb-4 rounded-lg",
            alt: "Table 8-4 VLAN Lists in the show interfaces trunk Command",
            loading: "lazy",
            src: asset!("/assets/static/v1p3c8s3sh3t8-4.png", AssetOptions::image().with_avif()),
        }

        {h3_heading("RECAP")}
        ol { class: "list-disc list-inside",
            li {
                "A particular VLAN trunk may not forward traffic for a defined and active VLAN for a variety of other reasons."
            }
            li {
                "Only VLANs that make it to the final list in the output of "
                {text_command("show interfaces trunk", TextCommandColor::Gold)}
                " will be forwarded over the trunk."
            }
            li {
                "The "
                {text_command("switchport trunk allowed vlan", TextCommandColor::Gold)}
                " interface subcommand gives the network engineer a
                method to administratively limit the VLANs whose traffic uses a trunk."
            }
        }
    }
}