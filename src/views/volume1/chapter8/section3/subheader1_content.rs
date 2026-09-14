use dioxus::prelude::*;

use crate::{
    components::{
        KeyTopic,
        GreenNote,
    }, utils::{TextCommandColor, h3_heading, text_command}
};

#[component]
pub fn Content() -> Element {
    rsx! {
        p { "Switches do not forward frames for VLANs that are:" }
        ul { class: "list-disc list-inside",
            li { "(a) not known because the VLAN is not configured or has not been learned with VTP" }
            li { "(b) the VLAN is known, but it is disabled (shut down)." }
        }
        p { class: "mb-4",
            "This next topic summarizes the best ways to confirm that a switch knows that a
            particular VLAN exists, and if it exists, determines the shutdown state of the VLAN."
        }

        {h3_heading("Check VLANs existence")}
        p {
            "First, on the issue of whether a VLAN exists on a switch, a VLAN can be defined to a
            switch in two ways: 
            "
        }
        ul { class: "list-disc list-inside",
            li {
                "using the "
                {text_command("vlan", TextCommandColor::Gold)}
                i { " number" }
                " global configuration command,"
            }
            li { "or it can be learned from another switch using VTP." }
        }
        p { class: "mb-4",
            "As mentioned earlier in this chapter, the examples in this book assume that you are not using VTP."
            br {}
            strong { "If you discover that a VLAN does not exist on a switch, simply configure the VLAN" }
            " as discussed earlier in the 
            section, “Creating VLANs and Assigning Access VLANs to an Interface.”"
        }

        {h3_heading("Check VLANs status")}
        p {
            "In addition to checking the configuration, you can check for the status of the VLAN (as well
            as whether it is known to the switch) using the "
            {text_command("show vlan", TextCommandColor::Gold)}
            " command."
            br {}
            "No matter the VTP mode, this command will list all VLANs known to the switch, plus one of two VLAN state
            values, depending on the current state:"
        }
        ul { class: "list-disc list-inside",
            li { "active" }
            li { "act/lshut" }
        }
        p { class: "mb-4",
            "The second of these states means that the VLAN is shut down."
            br {}
            "Shutting down a VLAN disables the VLAN on that switch only, so the switch will not forward frames in that VLAN."
        }

        {h3_heading("Enabling/Disabling VLANs")}
        p { class: "mb-4",
            "Switch IOS gives you two similar configuration methods with which to disable ("
            {text_command("shutdown", TextCommandColor::Gold)}
            ")
            and enable ("
            {text_command("no shutdown", TextCommandColor::Gold)}
            ") a VLAN."
            br {}
            "Example 8-11 shows how, first by using the global
            command "
            {text_command("[no] shutdown vlan", TextCommandColor::Gold)}
            i { " number" }
            " and then using the VLAN mode subcommand "
            {text_command("[no] shutdown", TextCommandColor::Gold)}
            "."
            br {}
            "The example shows the global commands enabling and disabling VLANs 10 and
            20, respectively, and using VLAN subcommands to enable and disable VLANs 30 and 40,
            respectively."
        }

        p { class: "mb-4", "See Example 8-11 in volume 1 on page 201." }

        GreenNote {
            p {
                strong { "NOTE" }
                " The output of the "
                {text_command("show vlan brief", TextCommandColor::Black)}
                " command also lists a state of “act/unsup” for the
                reserved VLAN IDs 1002-1005, with “unsup” meaning “unsupported.”"
            }
        }

        {h3_heading("RECAP")}
        ol { class: "list-disc list-inside",
            li {
                "Switches do not forward frames for VLANs that are (a) not known because the VLAN is not
                configured or has not been learned with VTP or (b) the VLAN is known, but it is disabled
                (shut down)."
            }
            li {
                "First, on the issue of whether a VLAN exists on a switch, a VLAN can be defined to a
                switch in two ways: using the "
                {text_command("vlan", TextCommandColor::Gold)}
                i { " number" }
                " global configuration command, or it can be learned from another switch using VTP."
            }
            li {
                "The "
                {text_command("show vlan", TextCommandColor::Gold)}
                " command lists all VLANs known to a switch, plus one of two VLAN state values, 
                depending on the current state: either "
                i { "active" }
                " or "
                i { "act/lshut" }
                "."
            }
            li {
                "Shutting down a VLAN disables the VLAN on that switch only, so the switch will not forward frames in that VLAN."
            }
            li {
                "You can disable VLANs by using the "
                {text_command("[no] shutdown vlan", TextCommandColor::Gold)}
                i { " number" }
                " global command and the "
                {text_command("[no] shutdown", TextCommandColor::Gold)}
                " VLAN mode 
                subcommand."
            }
        }

    }
}