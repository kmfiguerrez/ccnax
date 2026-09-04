use dioxus::prelude::*;

use crate::{components::GreenNote, utils::{TextCommandColor, h3_heading, text_command}};

#[component]
pub fn Content() -> Element {
    rsx! {
        p { class: "mb-4",
            "Of the eight log message severity levels, one level, debug level (7), has a special purpose: for
            messages generated as a result of a user logged in to the router or switch who issues a "
            {text_command("debug", TextCommandColor::Gold)}
            " command."
        }

        p { class: "mb-4",
            "The "
            {text_command("debug", TextCommandColor::Gold)}
            " EXEC command gives the network engineer a way to ask IOS to monitor for
            certain internal events, with that monitoring process continuing over time, so that IOS can
            issue log messages when those events occur."
            br {}
            "The engineer can log in, issue the "
            {text_command("debug", TextCommandColor::Gold)}
            " command, and move on to other work."
            br {}
            "The user can even log out of the device, and the debug remains enabled."
            br {}
            "IOS continues to monitor the request in that "
            {text_command("debug", TextCommandColor::Gold)}
            " command and generate log messages about any related events."
            br {}
            "The debug remains active until some user issues the "
            {text_command("no debug", TextCommandColor::Gold)}
            " command with the same parameters, disabling the debug."
        }

        GreenNote {
            p {
                strong { "NOTE" }
                " While the "
                {text_command("debug", TextCommandColor::Black)}
                " command is just one command, it has a huge number of options,
            much like the "
                {text_command("show", TextCommandColor::Black)}
                " command may be one command, but it also has many, many options."
            }
        }

        p { class: "mb-4", "See example 9-5 in the textbook on page 180" }

        {h3_heading("Note on using the debug command")}
        p {
            "Note that all enabled debug options use router CPU, which can cause problems for the router."
            br {}
            "You can monitor CPU use with the "
            {text_command("show process cpu", TextCommandColor::Gold)}
            " command, but you should use caution
            when using "
            {text_command("debug", TextCommandColor::Gold)}
            " commands carefully on production devices."
            br {}
            " Also, note the more CLI users that receive debug messages, the more CPU that is consumed."
            br {}
            "So, some installations choose to not include debug-level log messages for console and terminal logging, 
            requiring users to look at the logging buffer or syslog for those messages, just to reduce router CPU load."
        }
    }
}