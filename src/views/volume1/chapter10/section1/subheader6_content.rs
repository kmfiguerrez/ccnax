use dioxus::prelude::*;

use crate::utils::text_command;

#[component]
pub fn Content() -> Element {
    rsx! {
        p { class: "mb-1",
            "This chapter does not attempt to work through all the configuration options available for RSTP."
            br {}
            "However, many of the configuration settings may be intuitive now that you know
            quite a bit about the protocol."
            br {}
            "This final topic in the first section of the chapter summarizes a few of the configuration concepts."
            br {}
            "As a reminder, for those interested in continuing on to CCNP Enterprise, you might be interested in reading more 
            about RSTP configuration in the companion website's Appendix O, “Spanning Tree Protocol Implementation.”"
        }
        ol { class: "list-disc list-inside mb-4",
            li {
                span { class: "font-bold", "Switch Priority:" }
                "  The global command "
                {text_command("spanning-tree vlan", text_command::TextCommandColor::Gold)}
                i { " x " }
                {text_command("priority", text_command::TextCommandColor::Gold)}
                i { " y" }
                " lets an engineer set the switch's priority in that VLAN."
            }
            li {
                span { class: "font-bold", "Primary and Secondary Root Switches:" }
                "  The global command "
                {text_command("spanning-tree vlan", text_command::TextCommandColor::Gold)}
                i { " x " }
                {text_command("root", text_command::TextCommandColor::Gold)}
                {text_command(" primary | secondary", text_command::TextCommandColor::Gold)}
                "  also lets you set the priority, but the switch decides on a value to
                make that switch likely to be the primary root switch (the root) or the secondary root
                switch (the switch that becomes root if the primary fails). "
            }
            li {
                span { class: "font-bold", "Port Costs:" }
                "  The interface subcommand "
                {text_command("spanning-tree", text_command::TextCommandColor::Gold)}
                " ["
                {text_command("vlan", text_command::TextCommandColor::Gold)}
                i { " x " }
                "] "
                {text_command("cost", text_command::TextCommandColor::Gold)}
                i { " y " }
                "  lets an engineer set the switch's STP/RSTP cost on that port, either for all VLANs or for a specific VLAN
                on that port. Changing those costs then changes the root cost for some switches, which
                impacts the choice of root ports and designated ports."
            }
        }

        p {
            "That concludes this chapter's examination of RSTP configuration—now on to Layer 2 EtherChannel!"
        }
    }
}