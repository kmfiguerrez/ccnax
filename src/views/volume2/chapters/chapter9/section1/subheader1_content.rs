use dioxus::prelude::*;

use crate::utils::{text_command, TextCommandColor};

#[component]
pub fn Content() -> Element {
    rsx! {
        p { class: "mb-4",
            "Cisco IOS running on a device at least tries to allow current users to see log messages when
            they happen."
            br {}
            "if some user is logged in, the router or switch benefits by making the network engineer aware of any issues."
        }
        h3 { class: "font-semibold underline underline-offset-4 mb-1", "Console users" }
        p { class: "mb-4",
            "By default, IOS shows log messages to console users for all severity levels of messages."
            br {}
            "That default happens because of the default "
            {text_command("logging console", TextCommandColor::Gold)}
            " global configuration command."
            br {}
            "You likely have already noticed many syslog messages, like messages about interfaces coming up
            or going down."
        }

        h3 { class: "font-semibold underline underline-offset-4 mb-1", "Telnet and SSH users" }
        p {
            "For other users (that is, Telnet and SSH users), the device requires a two-step process before
            the user sees the messages."
        }
        // The pl-4 creates a space for the "outside" marker to sit exactly in line with the un-padded paragraph above it.
        ol { class: "list-decimal pl-4 mb-4",
            li {
                "First, IOS has another global configuration setting— "
                {text_command("logging monitor", TextCommandColor::Gold)}
                "—that tells IOS to enable the sending of log messages to all logged users."
                br {}
                "However, that default configuration is not enough to allow the user to see the log messages."
            }
            li {
                "The user must also issue the "
                {text_command("terminal monitor", TextCommandColor::Gold)}
                " EXEC command during the login session, which tells IOS that this terminal session would like to receive log messages."
            }
        }

        h3 { class: "font-semibold underline underline-offset-4 mb-1", "REMEMBER" }
        ul { class: "list-disc pl-4",
            li {
                "Both the "
                {text_command("logging console", TextCommandColor::Gold)}
                " and "
                {text_command("logging monitor", TextCommandColor::Gold)}
                " are default global configuration command."
            }
            li {
                "Both the Telnet and SSH users must issue the "
                {text_command("terminal monitor", TextCommandColor::Gold)}
                " EXEC command to receive log messages."
            }
        }
    }
}


