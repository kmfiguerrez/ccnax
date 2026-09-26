use dioxus::prelude::*;

use crate::{
    components::KeyTopic, utils::{TextCommandColor, h3_heading, text_command}
};

#[component]
pub fn Content() -> Element {
    rsx! {
        p { class: "mb-1",
            "When you are troubleshooting a Layer 3 EtherChannel, there are two main areas to consider."
        }
        ol { class: "list-disc list-inside mb-4",
            li {
                "First, you need to look at the configuration of the "
                {text_command("channel-group", TextCommandColor::Gold)}
                " command, which enables an interface for an EtherChannel."
            }
            li {
                "Second, you should check a list of settings that must match on the interfaces for a Layer 3 EtherChannel to 
                work correctly."
            }
        }

        p { class: "mb-4",
            "As for the "
            {text_command("channel-group", TextCommandColor::Gold)}
            " interface subcommand, this command can enable EtherChannel statically or dynamically."
            br {}
            "If dynamic, this command's keywords imply either Port Aggregation Protocol (PaGP) or 
            Link Aggregation Control Protocol (LACP) as the protocol to negotiate between the neighboring switches whether they 
            put the link into the EtherChannel."
        }

        {h3_heading("Layer 3 and 2 EtherChannel dynamic configurations")}
        p { class: "mb-4",
            "If all this sounds vaguely familiar, it is the exact same configuration covered way back in
            the Chapter 10 section “Configuring Dynamic EtherChannels.”"
            br {}
            "The configuration of the "
            {text_command("channel-group", TextCommandColor::Gold)}
            " subcommand is exactly the same, with the same requirements, whether
            configuring Layer 2 or Layer 3 EtherChannels."
            br {}
            "So, it might be a good time to review those EtherChannel configuration details from Chapter 10."
            br {}
            "However, regardless of when you review and master those commands, note that the configuration of the EtherChannel 
            (with the "
            {text_command("channel-group", TextCommandColor::Gold)}
            " subcommand) is the same, whether Layer 2 or Layer 3."
        }

        {h3_heading("Layer 3 EtherChannel requirements")}
        p { class: "mb-4",
            "Additionally, you must do more than just configure the "
            {text_command("channel-group", TextCommandColor::Gold)}
            " command correctly
            for all the physical ports to be bundled into the EtherChannel."
            br {}
            "Layer 2 EtherChannels have a longer list of requirements, but Layer 3 EtherChannels also require a few consistency 
            checks between the ports before they can be added to the EtherChannel."
            br {}
            "The following is the list of requirements for Layer 3 EtherChannels:"
        }
        KeyTopic {}
        ul { class: "list-disc list-inside mb-4",
            li {
                {text_command("no switchport", TextCommandColor::Gold)}
                ":  The PortChannel interface must be configured with the "
                {text_command("no switchport", TextCommandColor::Gold)}
                " command, and so must the physical interfaces. If a physical interface is not also configured
                with the "
                {text_command("no switchport", TextCommandColor::Gold)}
                " command, it will not become operational in the EtherChannel."
            }
            li {
                {text_command("speed", TextCommandColor::Gold)}
                ": The physical ports in the channel must use the same speed."
            }
            li {
                {text_command("duplex", TextCommandColor::Gold)}
                ": The physical ports in the channel must use the same duplex."
            }
        }

        {h3_heading("RECAP")}
        ol { class: "list-disc list-inside",
            li {
                "The "
                {text_command("channel-group", TextCommandColor::Gold)}
                " interface subcommand, this command can enable EtherChannel statically or dynamically 
                (using PaGP or LACP.)"
            }
            li {
                "Just like in layer 2 EtherChannel, layer 3 interfaces must match interface settings to be part
                of the channel and work correctly."
            }
        }
    }
}