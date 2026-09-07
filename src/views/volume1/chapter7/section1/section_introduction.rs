use dioxus::prelude::*;

use crate::utils::{text_command, TextCommandColor};

#[component]
pub fn SectionIntroductionContent() -> Element {
    rsx! {
        p { class: "mb-4",
            "IOS uses the term "
            i { "interface" }
            " to refer to physical ports used to forward data to and from other
            devices."
            br {}
            "Each interface can be configured with several settings, each of which might differ
            from interface to interface."
            br {}
            "IOS uses interface subcommands to configure these settings."
            br {}
            "Each of these settings may be different from one interface to the next, so you would first
            identify the specific interface, and then configure the specific setting."
        }

        p { "This section begins with a discussion of three relatively basic per-interface settings:" }
        ul { class: "list-disc list-inside",
            li { "the port speed" }
            li { "duplex" }
            li { "and a text description" }
        }
        p {
            "Following that, the text takes a short look at a pair
            of the most common interface subcommands: the "
            {text_command("shutdown", TextCommandColor::Gold)}
            " and "
            {text_command("no shutdown commands", TextCommandColor::Gold)}
            ", which administratively disable and enable the interface, respectively."
            br {}
            "This section ends with a discussion about autonegotiation concepts, which in turn dictates what settings a switch
            chooses to use when using autonegotiation."
        }

    }
}