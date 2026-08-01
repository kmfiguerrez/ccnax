use dioxus::prelude::*;

use crate::utils::text_command::text_command;

#[component]
pub fn Content() -> Element {
    rsx! {
        p { class: "mb-4",
            "Of the eight log message severity levels, one level, debug level (7), has a special purpose: for
            messages generated as a result of a user logged in to the router or switch who issues a "
            {text_command("debug")}
            " command."
        }

        p {
            "The "
            {text_command("debug")}
            " EXEC command gives the network engineer a way to ask IOS to monitor for
            certain internal events, with that monitoring process continuing over time, so that IOS can
            issue log messages when those events occur."
            br {}
            "The engineer can log in, issue the "
            {text_command("debug")}
            " command, and move on to other work."
            br {}
            "The user can even log out of the device, and the debug remains enabled."
            br {}
            "IOS continues to monitor the request in that "
            {text_command("debug")}
            " command and generate log messages about any related events."
            br {}
            "The debug remains active until some user issues the "
            {text_command("no debug")}
            " command with the same parameters, disabling the debug."
        }
    }
}