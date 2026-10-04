use dioxus::prelude::*;

use crate::{components::{KeyTopic, GreenNote}, utils::{ TextCommandColor, text_command}};

#[component]
pub fn Content() -> Element {
    rsx! {
        p { class: "mb-4",
            "This chapter has already introduced all the configuration steps in bits and pieces."
            br {}
            "This section summarizes those pieces as a configuration process."
            br {}
            "The process also refers to the "
            {text_command("access-list", TextCommandColor::Gold)}
            " command, whose generic syntax is repeated here for reference:"
        }

        p { class: "pl-4 mb-4",
            {text_command("access-list", TextCommandColor::Gold)}
            i { " access-list-number " }
            "{{"
            {text_command("deny", TextCommandColor::Gold)}
            " | "
            {text_command("permit", TextCommandColor::Gold)}
            "}}"
            i { " source [source-wildcard]" }
        }

        KeyTopic {}
        ol { class: "mb-4",
            // Step 1
            li { class: "flex flex-col md:flex-row md:gap-x-4",
                span { class: "text-sky-500 font-semibold shrink-0", "Step 1." }
                div {
                    span {
                        "Plan the location (router and interface) and direction (in or out) on that interface:"
                    }
                    ol {
                        // Step A
                        li {
                            span { class: "text-sky-500 font-semibold uppercase mr-2",
                                "a."
                            }
                            "Standard ACLs should be placed near to the destination of the packets so that
                            they do not unintentionally discard packets that should not be discarded."
                        }
                        // Step B
                        li {
                            span { class: "text-sky-500 font-semibold uppercase mr-2",
                                "b."
                            }
                            "Because standard ACLs can only match a packet's source IP address, identify the source IP addresses of 
                            packets as they go in the direction that the ACL is examining."
                        }
                    }
                }
            
            }
            // Step 2
            li { class: "flex flex-col md:flex-row md:gap-x-4",
                span { class: "text-sky-500 font-semibold shrink-0", "Step 2." }
                div {
                    span {
                        "Configure one or more "
                        {text_command("access-list", TextCommandColor::Gold)}
                        " global configuration commands to create
                        the ACL, keeping the following in mind:"
                    }
                    ol {
                        // Step A
                        li {
                            span { class: "text-sky-500 font-semibold uppercase mr-2",
                                "a."
                            }
                            "The list is searched sequentially, using first-match logic."
                        }
                        // Step B
                        li {
                            span { class: "text-sky-500 font-semibold uppercase mr-2",
                                "b."
                            }
                            "The default action, if a packet does not match any of the "
                            {text_command("access-list", TextCommandColor::Gold)}
                            " commands, is to "
                            {text_command("deny", TextCommandColor::Gold)}
                            " (discard) the packet."
                        }
                    }
                }
            
            }
            // Step 3
            li {
                span { class: "text-sky-500 font-semibold mr-4", "Step 3." }
                "Enable the ACL on the chosen router interface, in the correct direction, using
                        the "
                {text_command("ip access-group", TextCommandColor::Gold)}
                i { " number" }
                " {{"
                {text_command("in", TextCommandColor::Gold)}
                " | "
                {text_command("out", TextCommandColor::Gold)}
                "}} interface subcommand."
            
            }
        }

        p { class: "mb-4", "See Example 1 and 2, in volume 2 on page 35." }

        GreenNote {
            p {
                strong { "NOTE" }
                " When routers apply an ACL to filter packets in the outbound direction, as shown
                in Example 2-3, the router checks packets that it routes against the ACL."
                " However, a router does not filter packets that the router itself creates with an outbound ACL."
                " Examples of those packets include routing protocol messages and packets sent by the "
                {text_command("ping", TextCommandColor::Black)}
                " and "
                {text_command("traceroute", TextCommandColor::Black)}
                " commands on that router."
            }
        }

    }
}