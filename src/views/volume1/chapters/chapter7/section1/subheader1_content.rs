use dioxus::prelude::*;

use crate::{
    components::RedNote, 
    utils::{h3_heading, text_command, TextCommandColor}
};

#[component]
pub fn Content() -> Element {
    rsx! {
        p { class: "mb-4",
            "Switch interfaces that support multiple speeds (10/100 and 10/100/1000 interfaces), by
            default, will autonegotiate what speed to use."
            br {}
            "However, you can configure the speed and duplex settings with the "
            {text_command("duplex {auto | full | half}", TextCommandColor::Gold)}
            " and "
            {text_command("speed {auto | 10 | 100 | 1000}", TextCommandColor::Gold)}
            " interface subcommands."
            br {}
            "Simple enough."
        }

        p { class: "mb-4",
            "Most of the time, using autonegotiation makes good sense, so when you set the duplex
            and speed manually using these commands, you typically have a good reason to do so."
            br {}
            "For instance, maybe you want to set the speed to the fastest possible on links between switches
            just to avoid the chance that autonegotiation chooses a slower speed."
        }

        p { class: "mb-4",
            "The "
            {text_command("description", TextCommandColor::Gold)}
            " text interface subcommand lets you add a text description to the interface."
            br {}
            "For instance, if you have good reason to configure the speed and duplex on a port, maybe
            add a description that says why you did."
            br {}
            "Example 7-1 shows how to configure "
            {text_command("duplex", TextCommandColor::Gold)}
            " and "
            {text_command("speed", TextCommandColor::Gold)}
            ", as well as the "
            {text_command("description", TextCommandColor::Gold)}
            " command, which is simply a text description that can be configured by the administrator."
        }

        img {
            class: "mb-4 rounded-lg",
            alt: "Example 7-1 Configuring speed, duplex, and description on Switch Emma",
            loading: "lazy",
            src: asset!("/assets/static/v1p2c7s1sh1ex7-1.png", AssetOptions::image().with_avif()),
        }

        p { class: "mb-4",
            "First, focus on the mechanics of moving around in configuration mode again by looking
            closely at the command prompts."
            br {}
            "The various interface commands move the user from global mode into interface configuration mode for a 
            specific interface."
            br {}
            "For instance, the example configures the duplex, speed, and description commands all just after the interface
            FastEthernet 0/1 command, which means that all three of those configuration settings apply
            to interface Fa0/1, and not to the other interfaces."
        }

        p { class: "mb-4",
            "The "
            {text_command("show interfaces status", TextCommandColor::Gold)}
            " command lists much of the detail configured in Example 7-1,
            even with only one line of output per interface."
            br {}
            "Example 7-2 shows an example, just after the configuration in Example 7-1 was added to the switch."
        }

        p { class: "mb-4", "See Example 7-2 in volume 1 book on page 153" }

        p { class: "mb-4", "Working through the output in the example:" }
        ul { class: "list-disc list-inside mb-4",
            li {
                span { class: "font-bold", "FastEthernet 0/1 (Fa0/1):" }
                " This output lists the first few characters of the configured
                description. It also lists the configured speed of 100 and duplex full per the "
                {text_command("speed", TextCommandColor::Gold)}
                " and "
                {text_command("duplex", TextCommandColor::Gold)}
                " commands in Example 7-1. However, it also states that Fa0/1 has a status of notconnect, 
                meaning that the interface is not currently working. (That switch port did not
                have a cable connected when collecting this example, on purpose.)"
            }
            li {
                span { class: "font-bold", "FastEthernet 0/2 (Fa0/2):" }
                " Example 7-1 did not configure this port at all. This port had all
                default configuration. Note that the “auto” text under the speed and duplex heading means
                that this port will attempt to autonegotiate both settings when the port comes up. However,
                this port also does not have a cable connected (again on purpose, for comparison)."
            }
            li {
                span { class: "font-bold", "FastEthernet 0/4 (Fa0/4):" }
                " Like Fa0/2, this port has all default configuration but was cabled
                to another working device to give yet another contrasting example. This device completed
                the autonegotiation process, so instead of “auto” under the speed and duplex headings,
                the output lists the negotiated speed and duplex ("
                span { class: "font-bold", "a-full" }
                " and "
                span { class: "font-bold", "a-100" }
                "). Note that the text
                includes the "
                span { class: "font-bold", "a-" }
                " to mean that the listed speed and duplex values were autonegotiated."
            }
        }

        RedNote {
            p {
                strong { "NOTE" }
                " On switches with IOS C2960-LANBASEK9-M version 15.0(2)SE4 in Packet Tracer version 8.2.2.0400."
            }
            ul { class: "list-disc list-inside",
                li {
                    "When the speed and duplex are manually configured, the specified values will be listed
                    with "
                    span { class: "font-bold", "a-" }
                    " next to those values under the Duplex and Speed headings."
                }
                li {
                    "Autonegotiated values will not be shown under the Duplex and Speed headings but rather just auto auto
                    texts respectively."
                }
                li {
                    "The book from 2020 says otherwise. The text "
                    span { class: "font-bold", "a-" }
                    " next to a value means the value was autonegotiated and 
                    manually configured values are listed without the "
                    span { class: "font-bold", "a-" }
                    " text."
                }
            }
        }

        {h3_heading("RECAP")}
        ol { class: "list-disc list-inside",
            li {
                "Switch interfaces that support multiple speeds (10/100 and 10/100/1000 interfaces), by
                default, will autonegotiate what speed to use."
            }
            li {
                "Most of the time, using autonegotiation makes good sense, so when you set the duplex
                and speed manually using these commands, you typically have a good reason to do so."
            }
            li {
                "The "
                {text_command("description", TextCommandColor::Gold)}
                " text interface subcommand lets you add a text description to the interface."
            }
            li {
                "The "
                {text_command("show interfaces status", TextCommandColor::Gold)}
                " command displays interface status with only one line per interface"
            }
        }
    }
}