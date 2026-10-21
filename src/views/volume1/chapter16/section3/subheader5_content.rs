use dioxus::prelude::*;

use crate::{
    components::{KeyTopic, my_accordion::{Accordion, AccordionContent, AccordionItem, AccordionTrigger}}, utils::{TextCommandColor, h3_heading, h4_heading, text_command}
};

#[component]
pub fn Content() -> Element {
    rsx! {
        p { class: "mb-1",
            "These final few pages about IPv4 static routes examine some issues that can occur with static routes, both reviewing 
            some reasons mentioned over the last few pages, while adding more detail."
            br {}
            "This topic breaks static route troubleshooting into three perspectives: "
        }
        ol { class: "list-disc list-inside mb-4",
            li { "The route is in the routing table but is incorrect." }
            li { "The route is not in the routing table." }
            li {
                "The route is in the routing table and is correct, but the packets do not arrive at the destination host."
            }
        }

        Accordion { class: "mb-4",
            AccordionItem {
                AccordionTrigger {
                    {
                        h3_heading(
                            "Troubleshooting Incorrect Static Routes That Appear in the IP Routing Table",
                        )
                    }
                }
                AccordionContent {
                    p { class: "mb-4",
                        "This first troubleshooting item can be obvious, but it is worth pausing to think about."
                        br {}
                        "A static route is only as good as the input typed into the "
                        {text_command("ip route", TextCommandColor::Gold)}
                        " command."
                        br {}
                        "IOS checks the syntax, and as mentioned earlier, makes a few other checks that this section reviews in the
                        next heading."
                        br {}
                        strong {
                            "But once those checks are passed, IOS puts the route into the IP routing table,
                            even if the route had poorly chosen parameters."
                        }
                    }

                    p { class: "mb-4",
                        "For instance, the route might use a subnet and mask that implies a different range of addresses than the 
                        addresses in the destination subnet."
                        br {}
                        "Or, for a router sitting in the middle of a diagram, the next-hop address might be a router to the left, 
                        while the destination subnet is to the right."
                        br {}
                        "Or the next-hop address could be an IP address in a connected subnet, but it might be a typo and be an 
                        address of a PC or even a currently unused IP address."
                    }

                    p { class: "mb-1",
                        "When you see an exam question that has static routes, and you see them in the output of "
                        {text_command("show ip route", TextCommandColor::Gold)}
                        ", remember to check on these items:"
                    }
                    KeyTopic {}
                    ol { class: "list-disc list-inside mb-4",
                        li { "Is there a subnetting math error in the subnet ID and mask?" }
                        li {
                            "Is the next-hop IP address correct and referencing an IP address on a neighboring router?"
                        }
                        li { "Does the next-hop IP address identify the correct router?" }
                        li {
                            "Is the outgoing interface correct, and referencing an interface on the local router (that is,
                            the same router where the static route is configured)?"
                        }
                    }
                }
            }
            AccordionItem {
                AccordionTrigger { {h3_heading("The Static Route Does Not Appear in the IP Routing Table")} }
                AccordionContent {
                    p { class: "mb-1",
                        "After configuring an "
                        {text_command("ip route", TextCommandColor::Gold)}
                        " command, IOS might or might not add the route to the IP
                        routing table."
                        br {}
                        "IOS also considers the following before adding the route to its routing table:"
                    }
                    KeyTopic {}
                    ol { class: "list-disc list-inside mb-4",
                        li {
                            "For "
                            {text_command("ip route", TextCommandColor::Gold)}
                            " commands that list an outgoing interface, that interface must be in an up/up state."
                        }
                        li {
                            "For "
                            {text_command("ip route", TextCommandColor::Gold)}
                            " commands that list a next-hop IP address, the local router must have a route
                            to reach that next-hop address."
                        }
                    
                    }

                    {h4_heading("The permanent keyword")}
                    p { class: "mb-4",
                        strong {
                            "You can configure a static route so that IOS ignores these basic checks, always putting the
                            IP route in the routing table."
                        }
                        br {}
                        "To do so, just use the "
                        {text_command("permanent", TextCommandColor::Gold)}
                        " keyword on the "
                        {text_command("ip route", TextCommandColor::Gold)}
                        " command."
                        br {}
                        "For example, by adding the "
                        {text_command("permanent", TextCommandColor::Gold)}
                        " keyword to the end of the two commands as demonstrated in Example 16-7, R1 would now add these routes, 
                        regardless of whether the two WAN links were up."
                    }

                    img {
                        class: "mb-4 rounded-lg",
                        alt: "Example 16-7 Permanently Adding Static Routes to the IP Routing Table (Router R1)",
                        loading: "lazy",
                        src: asset!("/assets/static/v1p5c16s3sh5ex16-7.png", AssetOptions::image().with_avif()),
                    }

                    {h4_heading("Caution with the permanent keyword")}
                    p { class: "mb-4",
                        "Note that although the "
                        {text_command("permanent", TextCommandColor::Gold)}
                        " keyword lets the router keep the route in the routing table without checking the outgoing interface or 
                        route to the next-hop address, it does not magically fix a broken route."
                        br {}
                        strong {
                            "For example, if the outgoing interface fails, the route will remain in the routing table, but the router 
                            cannot forward packets because the outgoing interface is down."
                        }
                    }
                }
            }
            AccordionItem {
                AccordionTrigger { {h3_heading("The Correct Static Route Appears but Works Poorly")} }
                AccordionContent {
                    p { class: "mb-4",
                        "This last section is a place to make two points—one mainstream and one point to review a
                        bit of trivia."
                    }

                    p { class: "mb-4",
                        "First, on the mainstream point, the static route can be perfect, but the packets from one
                        host to the next still might not arrive because of other problems."
                        br {}
                        "An incorrect static route is just one of many items to check when you're troubleshooting problems like 
                        “host A cannot connect to server B.”"
                        br {}
                        "The root cause may be the static route, or it may be something else."
                        br {}
                        "Chapter 18, “Troubleshooting IPv4 Routing,” goes into some depth about troubleshooting
                        these types of problems."
                    }

                    p { class: "mb-4",
                        "On the more specific point, be wary of any "
                        {text_command("ip route", TextCommandColor::Gold)}
                        " command with the "
                        {text_command("permanent", TextCommandColor::Gold)}
                        " keyword."
                        br {}
                        "IOS puts these routes in the routing table with no checks for accuracy."
                        br {}
                        "You should check whether the outgoing interface is down and/or whether the router has a route to reach
                        the next-hop address. "
                    }
                }
            }
        }

        {h3_heading("RECAP")}
        ol { class: "list-disc list-inside",
            li {
                "You can force a static route into the routing table without checking the outgoing interface or route to the 
                next-hop router with the "
                {text_command("permanent", TextCommandColor::Gold)}
                " keyword. But it does not magically fix a broken route."
            }
        }
    }
}
