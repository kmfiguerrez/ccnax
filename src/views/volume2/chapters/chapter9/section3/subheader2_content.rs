use dioxus::prelude::*;

use crate::{components::red_div::RedNote, utils::{text_command, TextCommandColor, h3_heading}};

#[component]
pub fn Content() -> Element {
    rsx! {
        p { class: "mb-4",
            "Most of the work you do with CDP relates to what CDP can tell you with "
            {text_command("show", TextCommandColor::Gold)}
            " commands."
            br {}
            "However, it is an IOS feature, so you can configure CDP and use some "
            {text_command("show", TextCommandColor::Gold)}
            " commands to
            examine the status of CDP itself."
        }

        p { class: "mb-4",
            "IOS typically enables CDP globally and on each interface by default."
            br {}
            "You can then disable CDP per interface with the "
            {text_command("no cdp enable", TextCommandColor::Gold)}
            " interface subcommand and later re-enable it
            with the "
            {text_command("cdp enable", TextCommandColor::Gold)}
            " interface subcommand."
            br {}
            "To disable and re-enable CDP globally on the device, use the "
            {text_command("no cdp run", TextCommandColor::Gold)}
            " and "
            {text_command("cdp run", TextCommandColor::Gold)}
            " global commands, respectively."
        }

        p { class: "mb-3", "To examine the status of CDP itself, use the commands in Table 9-4." }
        img {
            class: "mb-4 rounded-lg",
            alt: "A picture of table 9-4 of commands used to verify CDP Operations",
            src: asset!("/assets/static/v2p3c9s3sh2t9-4.png", AssetOptions::image().with_avif()),
        }

        p { class: "mb-3",
            "Example 9-17 lists sample output from each of the commands in Table 9-4, based on switch
            SW2 in Figure 9-8."
        }
        img {
            class: "mb-4 rounded-lg",
            alt: "Example 9-17: A screenshot of the show cdp command",
            src: asset!("/assets/static/v2p3c9s3sh2ex9-17.png", AssetOptions::image().with_avif()),
        }
        RedNote {
            p {
                strong { "NOTE" }
                " On cisco IOS Denali version 16.3.2 in packet tracer, the "
                span { class: "font-semibold", "show cdp traffic" }
                " is not available."
            }
        }

        p { class: "mb-4",
            "The first two commands in the example list two related settings about how CDP works:
            the send time and the hold time."
            br {}
            "CDP sends messages every 60 seconds by default, with a hold time of 180 seconds."
            br {}
            "The hold time tells the device how long to wait after no longer
            hearing from a device before removing those details from the CDP tables."
            br {}
            "You can override the defaults with the "
            {text_command("cdp timer", TextCommandColor::Gold)}
            i { " seconds" }
            " and "
            {text_command("cdp holdtime", TextCommandColor::Gold)}
            i { " seconds" }
            " global commands, respectively."
        }

        {h3_heading("REMEMBER")}
        ul { class: "list-disc pl-4",
            li {
                "IOS typically enables CDP globally and on each interface by default, but can be overriden by global 
                and subconfiguration commands."
            }
            li { "The send time and hold time can also be overriden by global commands." }
            li { "CDP is a Cisco-proprietary layer 2 protocol." }
        }
    }
}