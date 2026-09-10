use dioxus::prelude::*;

use crate::{
    components::{KeyTopic, RedNote}, 
    utils::{h3_heading, text_command, TextCommandColor}
};

#[component]
pub fn Content() -> Element {
    rsx! {
        p { class: "mb-4",
            "To discuss some of the speed and duplex issues, first consider the output from the "
            {text_command("show interfaces status", TextCommandColor::Gold)}
            " and "
            {text_command("show interfaces", TextCommandColor::Gold)}
            " commands as demonstrated in Example 7-7."
            br {}
            "The first of these commands lists a one-line summary of the interface status, while the second command gives many 
            details—but surprisingly, the briefer "
            {text_command("show interfaces status", TextCommandColor::Gold)}
            " command tells us more about autonegotiation"
        }

        KeyTopic {}
        p { class: "mb-4", "See Example 7-7 in volume 1 on page 164." }

        RedNote {
            p {
                strong { "NOTE" }
                " On switches with IOS C2960-LANBASEK9-M version 15.0(2)SE4 in Packet Tracer version 8.2.2.0400."
            }
            ul { class: "list-disc list-inside",
                li {
                    "When the speed and duplex are manually configured, the specified values will be listed
                    with the prefix of "
                    span { class: "font-bold", "a-" }
                    " next to those values under the Duplex and Speed headings."
                }
                li {
                    "Autonegotiated values will not be shown under the Duplex and Speed headings but rather just auto auto
                    texts respectively."
                }
                li {
                    "The book from 2020 says otherwise: The prefix "
                    span { class: "font-bold", "a-" }
                    " next to a value means the value was autonegotiated and 
                    manually configured values are listed without the "
                    span { class: "font-bold", "a-" }
                    " prefix."
                }
            }
        }

        {
            h3_heading(
                "Only the show interfaces status command implies autonegotiated values",
            )
        }
        p { class: "mb-4",
            "Although both commands in the example can be useful, only the "
            {text_command("show interfaces status", TextCommandColor::Gold)}
            " command implies how the switch determined the speed and duplex settings."
            br {}
            "The command output lists autonegotiated settings with a prefix of "
            span { class: "font-bold", "a-" }
            " and the manually set values without
            the "
            span { class: "font-bold", "a-" }
            " prefix. "
        }

        p { class: "mb-4",
            "For example, consider ports Fa0/12 and Fa0/13 in the output of the show interfaces status command."
            br {}
            "For Fa0/13, "
            span { class: "font-bold", "a-full" }
            " means full duplex as autonegotiated, whereas "
            span { class: "font-bold", "half" }
            " on Fa0/12 means half duplex but as manually configured."
            br {}
            "The example shades the command output that implies that the switch's Fa0/12 interface's speed and duplex were 
            not found through autonegotiation, but Fa0/13 did use autonegotiation."
        }

        p { class: "mb-4",
            "In comparison, note that the "
            {text_command("show interfaces fa0/13", TextCommandColor::Gold)}
            " command (without the "
            {text_command("status", TextCommandColor::Gold)}
            " option)
            simply lists the speed and duplex for interface Fast Ethernet 0/13, with nothing implying that
            the values were learned through autonegotiation."
        }

        {h3_heading("IEEE Autonegotiation Revisited")}
        p {
            "When the IEEE autonegotiation process works on both devices—that is, both are sending autonegotiation 
            messages—both devices agree to the fastest speed and best duplex supported by both devices."
            br {}
            "However, when one device uses autonegotiation and the other disables it, the first device must resort to 
            default settings as detailed earlier in section “Autonegotiation Results When Only One Node Uses Autonegotiation.”"
            br {}
            "As a reminder, those defaults are "
        }
        KeyTopic {}
        ul { class: "list-disc list-inside mb-4",
            li {
                strong { "Speed:" }
                "  Sense the speed (without using autonegotiation), but if that fails, use the IEEE
                            default (slowest supported speed, often 10 Mbps)."
            }
            li {
                strong { "Duplex:" }
                "  Use the IEEE defaults: If speed = 10 or 100, use half duplex; otherwise, use full duplex."
            }
        }

        p { class: "mb-4",
            "When a switch must use its defaults, it should get the speed correct, but it may choose the
            wrong duplex setting, creating a duplex mismatch."
        }

        {h3_heading("Disabling autonegotiation on one device")}
        p { class: "mb-4",
            "For example, in Figure 7-4, imagine that SW2's Gi0/2 interface was configured with the "
            {text_command("speed 100", TextCommandColor::Gold)}
            " and "
            {text_command("duplex full", TextCommandColor::Gold)}
            " commands (these settings are not recommended on a Gigabitcapable interface, by the way)."
            br {}
            "On Cisco switches, configuring both the "
            {text_command("speed", TextCommandColor::Gold)}
            " and "
            {text_command("duplex", TextCommandColor::Gold)}
            " commands disables IEEE autonegotiation on that port."
            br {}
            "If SW1's Gi0/1 interface tries to use autonegotiation, SW1 would also use a speed of 100 Mbps, but default to use 
            half duplex."
            br {}
            "Example 7-8 shows the results of this specific case on SW1."
        }

        img {
            class: "mb-4 rounded-lg",
            alt: "Figure 7-4 Conditions to Create a Duplex Mismatch Between SW1 and SW2",
            loading: "lazy",
            src: asset!("/assets/static/v1p2c7s2sh2f7-4.png", AssetOptions::image().with_avif()),
        }

        img {
            class: "mb-4 rounded-lg",
            alt: "Example 7-8 Confirming Duplex Mismatch on Switch SW1",
            loading: "lazy",
            src: asset!("/assets/static/v1p2c7s2sh2ex7-8.png", AssetOptions::image().with_avif()),
        }

        p { class: "mb-4",
            "First, note that even though SW1 had to use an autonegotiation default, the "
            {text_command("show interfaces status", TextCommandColor::Gold)}
            " command still shows the speed and duplex with the "
            span { class: "font-bold", "a-" }
            " prefix."
            br {}
            "SW2's port was manually set to 100/Full,  so SW1 sensed the speed and runs at 100 Mbps; 
            however, the autonegotiation rules then tell SW1 to use half duplex, 
            as confirmed by the output in Example 7-8."
        }

        p { class: "mb-4",
            "The output does not identify the duplex mismatch in any way; in fact, finding a duplex
            mismatch can be much more difficult than finding a speed mismatch."
            br {}
            strong {
                "For instance, if you purposefully set the speed on the link in Figure 7-4 to be 10 Mbps on one switch and 100
                Mbps on the other, both switches would list the port in a down/down or notconnect state."
            }
            br {}
            "However, in the case shown in Example 7-8, with a duplex mismatch, "
            i {
                "if the duplex settings do not match on the ends of an Ethernet segment, the switch interface will still be 
                in a connected (up/up) or connected state."
            }
        }

        RedNote {
            p {
                strong { "NOTE" }
                " In Packet Tracer version 8.2.2.0400."
                br {}
                "Manually configuring the speed and duplex settings (disabling autonegotiation) on one end and use 
                autonegotiation on the other end, switches would list  the port in a down/down or notconnect state. 
                Just like manually configuring different speeds to both ends."
                br {}
                "In other words, the link is unsuable or disabled!"
                br {}
                "So it's either both ends manually configured exact values for speed and duplex settings or both use 
                autonegotiation. Otherwise the link is unsuable!"
            }
        }

        {h3_heading("Half duplex uses CSMA/CD")}
        p { class: "mb-4",
            "Not only does the "
            {text_command("show", TextCommandColor::Gold)}
            " command give an appearance that the link has no issues, but the
            link will likely work poorly, with symptoms of intermittent problems."
            br {}
            "The reason is that the device using half duplex (SW1 in this case) uses carrier sense multiple access 
            collision detect (CSMA/CD) logic, waiting to send when receiving a frame, believing collisions occur when
            they physically do not—and actually stopping sending a frame because the switch thinks a
            collision occurred."
            "With enough traffic load, the interface could be in a connect state, but
            it's extremely inefficient for passing traffic."
        }

        {h3_heading("Identifying Duplex Mismatch")}
        p { class: "mb-4",
            "To identify duplex mismatch problems, check the duplex setting on each end of the link to
            see if the values mismatch."
            br {}
            "You can also watch for incrementing collision and late collision
            counters."
        }

        {h3_heading("RECAP")}
        ol { class: "list-disc list-inside",
            li {
                "Only the "
                {text_command("show interfaces status", TextCommandColor::Gold)}
                " command implies how the switch determined the speed and duplex settings 
                (autonegotiated or manually configured) but does not identify the duplex mismatch in any way."
            }
            li {
                "On Cisco switches, configuring both the "
                {text_command("speed", TextCommandColor::Gold)}
                " and "
                {text_command("duplex", TextCommandColor::Gold)}
                " commands disables IEEE autonegotiation on that port."
            }
            li {
                "If the duplex settings do not match on the ends of an Ethernet segment, the switch interface will still be in a 
                connected (up/up) or connected state but the link will likely work poorly, with symptoms of intermittent problems."
            }
            li {
                "To identify duplex mismatch problems, check the duplex setting on each end of the link to see if the values mismatch."
            }
        }

    }
}