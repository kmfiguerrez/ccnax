use dioxus::prelude::*;

use crate::{components::{green_div::GreenNote, red_div::RedNote}, utils::text_command::text_command};

#[component]
pub fn Content() -> Element {
    rsx! {
        p { class: "mb-4",
            "LLDP uses a similar configuration model as CDP, but with a few key differences."
            br {}
            "First, Cisco devices default to disable LLDP."
            br {}
            "Additionally, LLDP separates the sending and receiving  of LLDP messages as separate functions."
            br {}
            "For instance, LLDP support processing receives LLDP messages on an interface so that the switch or router learns 
            about the neighboring device while not transmitting LLDP messages to the neighboring device."
            br {}
            "To support that model, the commands include options to toggle on|off the transmission of LLDP messages
            separately from the processing of received messages."
        }

        p { "The three LLDP configuration commands are as follows:" }
        ol { class: "list-inside list-disc mb-4",
            li {
                {text_command("[no] lldp run:")}
                " A global configuration command that sets the default mode of LLDP
                operation for any interface that does not have more specific LLDP subcommands
                ("
                {text_command("lldp transmit, lldp receive")}
                "). The "
                {text_command("lldp run")}
                " global command enables LLDP in both
                directions on those interfaces, while "
                {text_command("no lldp run")}
                " disables LLDP."
            }
            li {
                {text_command("[no] lldp transmit:")}
                " An interface subcommand that defines the operation of LLDP on the
                interface regardless of the global "
                {text_command("[no] lldp run")}
                " command. The "
                {text_command("lldp transmit")}
                " interface subcommand causes the device to transmit LLDP messages, while "
                {text_command("no lldp transmit")}
                " causes it to not transmit LLDP messages."
            }
            li {
                {text_command("[no] lldp receive:")}
                " An interface subcommand that defines the operation of LLDP on the
                interface regardless of the global "
                {text_command("[no] lldp run")}
                " command. The "
                {text_command("lldp receive")}
                " interface subcommand causes the device to process received LLDP messages, while "
                {text_command("no lldp receive")}
                " causes it to not process received LLDP messages."
            }
        }

        p { class: "mb-3",
            "For example, consider a switch that has no LLDP configuration commands at all."
            br {}
            "Example 9-20 adds a configuration that first enables LLDP for all interfaces (in both directions) with
            the "
            {text_command("lldp run")}
            " global command."
            br {}
            "It then shows how to disable LLDP in both directions on Gi1/0/17 and how to disable LLDP in one direction on Gi1/0/18. "
        }
        img {
            class: "mb-4 rounded-lg",
            alt: "Example 9-20: a screentshot of Enabling LLDP on All Ports, Disabling on a Few Ports",
            src: asset!("/assets/static/v2p3c9s3sh4ex9-20.png", AssetOptions::image().with_avif()),
        }

        p { class: "mb-3",
            "Example 9-21 adds another example that again begins with a switch with all default
            settings."
            br {}
            "In this case, the configuration does not enable LLDP for all interfaces with the "
            {text_command("lldp run")}
            " command, meaning that all interfaces default to not transmit and not receive LLDP
            messages."
            br {}
            "The example does show how to then enable LLDP for both directions on one
            interface and in one direction for a second interface."
        }
        img {
            class: "mb-4 rounded-lg",
            alt: "Example 9-21: a screentshot of Enabling LLDP on Limited Ports, Leaving Disabled on Most",
            src: asset!("/assets/static/v2p3c9s3sh4ex9-21.png", AssetOptions::image().with_avif()),
        }

        p { class: "mb-3",
            "Finally, checking LLDP status uses the exact same commands as CDP as listed in Table 9-4,
            other than the fact that you use the "
            {text_command("lldp")}
            " keyword instead of "
            {text_command("cdp.")}
            br {}
            "For instance, "
            {text_command("show lldp interface")}
            " lists the interfaces on which LLDP is enabled."
            br {}
            "Example 9-22 shows some examples from switch SW2 based on earlier Figure 9-8 (the same figure used in the CDP examples),
            with LLDP enabled in both directions on all interfaces with the "
            {text_command("cdp run")}
            " global command."
        }
        img {
            class: "mb-4 rounded-lg",
            alt: "Example 9-21: a screentshot of Enabling LLDP on Limited Ports, Leaving Disabled on Most",
            src: asset!("/assets/static/v2p3c9s3sh4ex9-22.png", AssetOptions::image().with_avif()),
        }

        p { class: "mb-4",
            "Also, note that like CDP, LLDP uses a send timer and hold timer for the same purposes as CDP."
            br {}
            "The example shows the default settings of 30 seconds for the send timer and 120 seconds for the hold timer."
            br {}
            "You can override the defaults with the "
            {text_command("lldp timer")}
            i { " seconds" }
            " and "
            {text_command("lldp holdtime")}
            i { " seconds" }
            " global commands , respectively."
        }

        h3 { class: "font-semibold text-lg mb-1", "REMEMBER" }
        ul { class: "list-disc pl-4",
            li { "Cisco devices default to disable LLDP." }
            li { "LLDP separates the sending and receiving of LLDP messages as separate functions." }
            li {
                "Also, note that like CDP, LLDP uses a send timer and hold timer for the same purposes as CDP."
            }
        }
    }
}