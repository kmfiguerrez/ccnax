use dioxus::prelude::*;

use crate::{
    components::{KeyTopic, GreenNote, RedNote}, 
    utils::{h3_heading, text_command, TextCommandColor}
};

#[component]
pub fn Content() -> Element {
    rsx! {
        p { "Cisco switches actually use two different sets of interface status codes:" }
        ul { class: "list-disc list-inside",
            li {
                "one set of two codes (words) that use the same conventions as do router interface status codes,"
            }
            li { "and another set with a single code (word)." }
        }
        p { class: "mb-4", "Both sets of status codes can determine whether an interface is working." }

        {h3_heading("The two-code status")}
        p { class: "mb-4",
            "The switch "
            {text_command("show interfaces", TextCommandColor::Gold)}
            " and "
            {text_command("show interfaces description", TextCommandColor::Gold)}
            " commands list the two-code
            status named the "
            i { "line status" }
            " and "
            i { "protocol status" }
            "."
            br {}
            "The line status generally refers to whether
            Layer 1 is working, with protocol status generally referring to whether Layer 2 is working."
        }

        GreenNote {
            p {
                strong { "NOTE" }
                " This book refers to these two status codes in shorthand by just listing the two
                codes with a slash between them, such as up/up."
            }
        }

        {h3_heading("Mapping the single-code status to two-code status")}
        p { class: "mb-4",
            "The single-code interface status corresponds to different combinations of the traditional
            two-code interface status codes and can be easily correlated to those codes."
            br {}
            "For example, the "
            {text_command("show interfaces status", TextCommandColor::Gold)}
            " command lists a single-word state of connected state for
            working interfaces, with the same meaning as the two-word up/up state seen with the "
            {text_command("show interfaces", TextCommandColor::Gold)}
            " and "
            {text_command("show interfaces description", TextCommandColor::Gold)}
            " commands."
            br {}
            "Table 7-2 lists the code combinations
            and some root causes that could have caused a particular interface status."
        }

        RedNote {
            p {
                strong { "NOTE" }
                " On Cisco IOS C2960-LANBASEK9-M version 15.0(2)SE4 in Packet Tracer version 8.2.2.0400."
                " The "
                {text_command("show interfaces description", TextCommandColor::Black)}
                " command is not available."
            }
        }

        KeyTopic {}
        img {
            class: "mb-4 rounded-lg",
            alt: "Table 7-2 LAN Switch Interface Status Codes",
            loading: "lazy",
            src: asset!("/assets/static/v1p2c7s2sh1t7-2.png", AssetOptions::image().with_avif()),
        }

        p { class: "mb-4",
            "Examining the notconnect state for a moment, note that this state has many causes that have
            been mentioned through this book."
            br {}
            "For example, using incorrect cabling pinouts, instead of the correct pinouts explained in Chapter 2, 
            “Fundamentals of Ethernet LANs,” causes a problem."
            br {}

            span { class: "text-red-400",
                "However, one topic can be particularly difficult to troubleshoot—the possibility
                for both speed and duplex mismatches, as explained in the next section."
            }
        }

        {h3_heading("Examples of the root causes of cabling problems")}
        p {
            "As you can see in the table, having a bad cable is just one of many reasons for the down/down
            state (or notconnect, per the "
            {text_command("show interfaces status", TextCommandColor::Gold)}
            " command)."
            br {}
            "Some examples of the root causes of cabling problems include the following:"
        }
        ul { class: "list-disc list-inside mb-4",
            li {
                "The installation of any equipment that uses electricity, even non-IT equipment, can interfere with the transmission on 
                the cabling and make the link fail."
            }
            li {
                "The cable could be damaged, for example, if it lies under carpet. If the user's chair keeps
                squashing the cable, eventually the electrical signal can degrade."
            }
            li {
                "Although optical cables do not suffer from electromagnetic interference (EMI), someone
                can try to be helpful and move a fiber-optic cable out of the way—bending it too much.
                A bend into too tight a shape can prevent the cable from transmitting bits (called "
                i { "macrobending" }
                ")."
            }
        }

        {h3_heading("Make sure of the up/up (connected) state")}
        p { class: "mb-4",
            "For the other interface states listed in Table 7-2, only the up/up (connected) state needs more
            discussion."
            br {}
            "An interface can be in a working state, and it might really be working—or it
            might be working in a degraded state."
            br {}
            "The next few topics discuss how to examine an up/up
            (connected) interface to find out whether it is working well or having problems."
        }

        {h3_heading("RECAP")}
        ol { class: "list-disc list-inside",
            li {
                "Cisco switches actually use two different sets of interface status codes—one set of two
                codes (words) that use the same conventions as do router interface status codes, and another
                set with a single code (word)."
            }
            li {
                "The two-code status has a name "
                i { "line status" }
                " and"
                i { " protocol status" }
                ". The line status generally refers to whether Layer 1 is working, with protocol status generally referring 
                to whether Layer 2 is working."
            }
            li {
                "An interface in an up/up (connected) state could be working in a degraded state, so make sure to also
                examine it."
            }
        }
    }
}