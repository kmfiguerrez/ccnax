use dioxus::prelude::*;

use crate::{
    components::GreenNote, 
    utils::{h3_heading, text_command, TextCommandColor}
};

#[component]
pub fn Content() -> Element {
    rsx! {
        p { class: "mb-4",
            "To shorten your configuration work when making the same setting on multiple consecutive interfaces."
            br {}
            "To do so, use the "
            {text_command("interface range", TextCommandColor::Gold)}
            " command."
            br {}
            "The "
            {text_command("interface range FastEthernet 0/11 - 20", TextCommandColor::Gold)}
            " command tells IOS that the next subcommand(s) 
            apply to interfaces Fa0/11 through Fa0/20."
            br {}
            "You can define a range as long as all interfaces are the same type and are numbered consecutively."
        }

        GreenNote {
            p {
                strong { "NOTE" }
                " This book spells out all parameters fully to avoid confusion."
                " However, most everyone abbreviates what they type in the CLI to the shortest unique abbreviation."
                " For instance, the configuration commands "
                {text_command("int f0/1 and int ran f0/11 - 20", TextCommandColor::Black)}
                " would also be acceptable."
            }
        }

        p { class: "mb-4",
            "IOS does not actually put the "
            {text_command("interface range", TextCommandColor::Gold)}
            " command into the configuration."
            br {}
            "Instead, it acts as if you had typed the subcommand under every single interface in the specified
            range."
            br {}
            "Example 7-3 shows an excerpt from the "
            {text_command("show running-config", TextCommandColor::Gold)}
            " command, listing the
            configuration of interfaces F0/11-12 from the configuration in Example 7-1."
            br {}
            "The example shows the same description command on both interfaces; to save space, the example does
            not bother to show all 10 interfaces that have the same description text."
        }

        img {
            class: "mb-4 rounded-lg",
            alt: "Example 7-3 How IOS Expands the Subcommands Typed After interface range",
            loading: "lazy",
            src: asset!("/assets/static/v1p2c7s1sh2ex7-3.png", AssetOptions::image().with_avif()),
        }

        {h3_heading("RECAP")}
        ol { class: "list-disc list-inside",
            li {
                "You can define a range as long as all interfaces are the same type and are numbered consecutively."
            }
            li { "IOS does not put abbreviated commands into the configuration. " }
        }
    }
}