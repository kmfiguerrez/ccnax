use dioxus::prelude::*;

use crate::{components::KeyTopic, utils::h3_heading};

#[component]
pub fn Content() -> Element {
    rsx! {
        p { class: "mb-1",
            strong { "Cisco IOS has supported IP ACLs since the early days of Cisco routers." }
            br {}
            "Beginning with the original standard numbered IP ACLs in the early days of IOS, which could enable the
            logic shown earlier around Figure 2-2, Cisco has added many ACL features, including the
            following:"
        }
        ol { class: "list-disc list-inside mb-4",
            li { "Standard numbered ACLs (1-99)" }
            li { "Extended numbered ACLs (100-199)" }
            li { "Additional ACL numbers (1300-1999 standard, 2000-2699 extended)" }
            li { "Named ACLs" }
            li { "Improved editing with sequence numbers" }
        }

        p { class: "mb-4",
            "This chapter focuses solely on standard numbered IP ACLs, while the next chapter discusses
            the other three primary categories of IP ACLs."
            br {}
            "Briefly, IP ACLs will be either numbered or named in that the configuration identifies the ACL either using a number 
            or a name."
            br {}
            "ACLs will also be either standard or extended, with extended ACLs having much more robust abilities in matching 
            packets."
            br {}
            "Figure 2-3 summarizes the big ideas related to categories of IP ACLs."
        }

        KeyTopic {}
        img {
            class: "mb-4 rounded-lg",
            loading: "lazy",
            alt: "Figure 2-3 Comparisons of IP ACL Types",
            src: asset!("/assets/static/v2p3c2s1sh4f2-3.png", AssetOptions::image().with_avif()),
        }

        {h3_heading("RECAP")}
        ol { class: "list-disc list-inside",
            li {
                "IP ACLs will be either numbered or named in that the configuration identifies the ACL either using a number or a name."
            }
            li {
                "ACLs will also be either standard or extended, with extended ACLs having much more robust abilities in matching packets."
            }
        }
    }
}