use dioxus::prelude::*;

use crate::{components::GreenNote, utils::{TextCommandColor, h3_heading, text_command}};

#[component]
pub fn Content() -> Element {
    rsx! {
        p { class: "mb-4",
            "One purpose for the specific commands shown in Part II of the book is to teach you about
            that command."
            br {}
            "In some cases, the commands are not the end goal, and the text is attempting
            to teach you something about how the CLI works."
            br {}
            "This next short topic is more about the process than about the commands."
        }

        p {
            "With some IOS configuration commands (but not all), you can revert to the default setting by issuing a "
            {text_command("no", TextCommandColor::Gold)}
            " version of the command."
            br {}
            "What does that mean?"
            br {}
            "Let me give you a few examples:"
        }
        ul { class: "list-disc list-inside mb-4",
            li {
                " If you earlier had configured "
                {text_command("speed 100", TextCommandColor::Gold)}
                " on an interface, the "
                {text_command("no speed", TextCommandColor::Gold)}
                " command on that
                same interface reverts to the default speed setting (which happens to be "
                {text_command("speed auto", TextCommandColor::Gold)}
                ")."
            }
            li {
                "Same idea with the "
                {text_command("duplex", TextCommandColor::Gold)}
                " command: an earlier configuration of "
                {text_command("duplex half", TextCommandColor::Gold)}
                " or "
                {text_command("duplex full", TextCommandColor::Gold)}
                ", followed by "
                {text_command("no duplex", TextCommandColor::Gold)}
                " on the same interface, reverts the configuration back to the
                default of duplex auto."
            }
            li {
                "If you had configured a "
                {text_command("description", TextCommandColor::Gold)}
                " command with some text, to go back to the default state of having no "
                {text_command("description", TextCommandColor::Gold)}
                " command at all for that interface, use the "
                {text_command("no description", TextCommandColor::Gold)}
                " command."
            }
        }

        p { class: "mb-4",
            "Example 7-6 shows the process."
            br {}
            "In this case, switch SW1's F0/2 port has been configured with "
            {
                text_command(
                    "speed 100, duplex half, description link to 2901-2",
                    TextCommandColor::Gold,
                )
            }
            ", and "
            {text_command("shutdown.", TextCommandColor::Gold)}
            br {}
            "You can see evidence of all four settings in the command that begins the example."
            br {}
            "(This command lists the running-config, but only the part for that one interface.)"
            br {}
            "The example then shows the no versions of those commands and closes with a confirmation that all the commands have
            reverted to default."
        }

        p { class: "mb-4", "See Example 7-6 in volume 1 on page 157." }

        GreenNote {
            p {
                strong { "NOTE" }
                " The "
                {text_command("show running-config", TextCommandColor::Black)}
                " and "
                {text_command("show startup-config", TextCommandColor::Black)}
                " commands typically do not display default configuration settings, 
                so the absence of commands listed under interface F0/2 at the end of the example means that those commands 
                now use default values."
            }
        }

        {h3_heading("RECAP")}
        ol { class: "list-disc list-inside",
            li {
                "With some IOS configuration commands (but not all), you can revert to the default setting by issuing a "
                {text_command("no", TextCommandColor::Gold)}
                " version of the command."
            }
        }
    }
}