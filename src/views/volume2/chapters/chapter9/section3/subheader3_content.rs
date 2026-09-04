use dioxus::prelude::*;

use crate::{components::{green_div::GreenNote, red_div::RedNote}, utils::{text_command, TextCommandColor, h3_heading}};

#[component]
pub fn Content() -> Element {
    rsx! {
        p { class: "mb-4",
            "Cisco created the Cisco-proprietary CDP before any standard existed for a similar protocol."
            br {}
            "CDP has many benefits."
            br {}
            "As a Layer 2 protocol, sitting on top of Ethernet, it does not rely on a working Layer 3 protocol."
            br {}
            "It provides device information that can be useful in a variety of ways."
            br {}
            "Cisco had a need but did not see a standard that met the need, so Cisco made up a
            protocol, as has been the case many times over history with many companies and protocols."
        }

        p { class: "mb-4",
            "Link Layer Discovery Protocol (LLDP), defined in IEEE standard 802.1AB, provides a standardized protocol that provides the 
            same general features as CDP."
            br {}
            "LLDP has similar configuration and practically identical "
            {text_command("show", TextCommandColor::Gold)}
            " commands as compared with CDP. "
        }

        p { class: "mb-3",
            "The LLDP examples all use the same topology used in the CDP examples per Figure 9-8
            (the same figure used in the CDP examples)."
            br {}
            "Example 9-18 lists switch SW2's LLDP neighbors as learned after LLDP was enabled on all devices and "
            "ports in that figure."
            br {}
            "The example highlights the items that match the similar output from the "
            {text_command("show cdp neighbors", TextCommandColor::Gold)}
            " command listed at the end of the example, also from switch SW2."
        }
        img {
            class: "mb-4 rounded-lg",
            alt: "Example 9-18: a screentshot of the output of show lldp neighbors and show cdp neighbors",
            src: asset!("/assets/static/v2p3c9s3sh3ex9-18.png", AssetOptions::image().with_avif()),
        }

        {h3_heading("The Output similarities of CDP and LLDP")}
        p { class: "mb-4",
            "The most important take-away from the output is the consistency between CDP and LLDP
            in how they refer to the interfaces."
            br {}
            "Both the "
            {text_command("show cdp neighbors", TextCommandColor::Gold)}
            " and "
            {text_command("show lldp neighbors", TextCommandColor::Gold)}
            " commands have “local intf” (interface) and “port ID” columns."
            br {}
            "These columns refer to the local device's interface and the neighboring device's interface, respectively."
        }

        {h3_heading("The Output differences of CDP and LLDP")}
        p { "However, the LLDP output in the example does differ from CDP in a few important ways:" }
        ol { class: "list-inside list-disc mb-4",
            li {
                "LLDP uses B as the capability code for switching, referring to "
                span { class: "font-semibold", "bridge" }
                ", a term for the
                device type that existed before switches that performed the same basic functions."
            }
            li {
                "LLDP does not identify IGMP as a capability, while CDP does ("
                span { class: "font-semibold", "I" }
                ")."
            }
            li {
                "CDP lists the neighbor's "
                span { class: "font-semibold", "platform" }
                ", a code that defines the device type, while LLDP does not."
            }
            li { "LLDP lists capabilities with different conventions (see upcoming Example 9-19)." }
        }

        p { class: "mb-4",
            "The first three items in the list are relatively straightforward, but that last item in the list
            requires a closer look with more detail."
            br {}
            "Interestingly, CDP lists all the capabilities of the neighbor in the "
            {text_command("show cdp neighbors", TextCommandColor::Gold)}
            " command output, 
            no matter whether the device currently enables all those features."
            br {}
            "LLDP instead lists the enables (configured) capabilities, rather than all supported capabilities, in the output 
            from "
            {text_command("show lldp neighbors", TextCommandColor::Gold)}
            " command. "
        }

        p { class: "mb-3",
            "LLDP makes the difference in a neighbor's total capabilities and configured capabilities with
            the "
            {text_command("show lldp neighbors detail", TextCommandColor::Gold)}
            " and "
            {text_command("show lldp entry", TextCommandColor::Gold)}
            i { " hostname" }
            " commands."
            br {}
            "These commands provide identical detailed output, with the first command providing detail for all neighbors,
            and the second providing detail for the single listed neighbor."
            br {}
            "Example 9-19 shows the detail for neighbor R1."
        }
        img {
            class: "mb-4 rounded-lg",
            alt: "Example 9-19: a screentshot of the output of show lldp entry",
            src: asset!("/assets/static/v2p3c9s3sh3ex9-19.png", AssetOptions::image().with_avif()),
        }
        RedNote {
            p {
                strong { "NOTE" }
                " On cisco IOS Denali version 16.3.2 in packet tracer, the "
                span { class: "font-semibold", "show lldp entry" }
                i { " hostname" }
                " is not available."
            }
        }

        p {
            "First, regarding the device capabilities, note that the LLDP command output lists two lines
            about the neighbor's capabilities:"
        }
        ol { class: "list-inside list-disc mb-4",
            li {
                strong { class: "text-sky-600", "System Capabilities:" }
                " What the device can do"
            }
            li {
                strong { class: "text-sky-600", "Enabled Capabilities:" }
                " What the device does now with its current configuration"
            }
        }

        p { class: "mb-4",
            "For instance, in Example 9-19, the neighboring R1 claims the ability to perform routing and
            switching (codes "
            span { class: "font-semibold", "R" }
            " and "
            span { class: "font-semibold", "B" }
            ") but also claims to currently be using only its routing capability, as
            noted in the “enabled capabilities” line."
        }

        p { class: "mb-4",
            "Also, take a moment to look at the output for the similarities to CDP."
            br {}
            "For instance, this output lists detail for neighbor, R1, which uses its local port G0/0/1, 
            with a host name of R1."
            br {}
            "The output also notes the IOS name and version, from which an experienced person can
            infer the model number, but there is no explicit mention of the model."
        }

        GreenNote {
            p {
                strong { "NOTE" }
                " LLDP uses the same messaging concepts as CDP, encapsulating messages directly
                in data-link headers. Devices do not forward LLDP messages so that LLDP learns only
                of directly connected neighbors. LLDP does use a different multicast MAC address (0180.C200.000E)"
            }
        }

        {h3_heading("REMEMBER")}
        ul { class: "list-disc pl-4",
            li { "Link Layer Discovery Protocol (LLDP), is defined in IEEE standard 802.1AB." }
            li {
                "LLDP uses the same messaging concepts as CDP, encapsulating messages directly
                in data-link headers."
            }
            li { "LLDP does use a different multicast MAC address (0180.C200.000E)." }
            li { "Cisco devices default to disable LLDP." }
        }

    }
}