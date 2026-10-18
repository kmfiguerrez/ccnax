use dioxus::prelude::*;

use crate::components::KeyTopic;

#[component]
pub fn SectionIntroductionContent() -> Element {
    rsx! {
        p { class: "mb-4",
            strong {
                "A router's IP routing process requires that the router compare the destination IP address of
                each packet with the existing contents of that router's IP routing table."
            }
            br {}
            "Often, only one route matches a particular destination address."
            br {}
            "When only one route matches the packet's destination, the action is obvious: forward the packet based on the details 
            listed in that route."
        }

        p { class: "mb-1",
            strong {
                "In some cases, a particular destination address matches more than one of the router's routes."
            }
            br {}
            "For instance, one route might list subnet 10.1.0.0/16, another 10.1.1.0/25, and another 10.1.1.1/32."
            br {}
            "All would match packets sent to IP address 10.1.1.1."
            br {}
            "Many legitimate router features can cause these multiple routes to appear in a router's routing table, including"
        }
        ol { class: "list-disc list-inside mb-4",
            li { "Static routes" }
            li { "Route autosummarization" }
            li { "Manual route summarization" }
        }

        p { class: "mb-4",
            "This fourth of four major sections of this chapter explains how a router makes its routing
            decisions when a packet matches multiple routes."
            br {}
            "When more than one route matches a packet's destination address, the router uses the “best” route, defined as follows:"
        }
        div { class: "flex items-center gap-x-3",
            KeyTopic {}
            p {
                strong {
                    "When a particular destination IP address matches more than one route in a router's IPv4
                    routing table, the router uses the most specific route—in other words, the route with the
                    longest prefix length mask."
                }
            }
        }
    }
}