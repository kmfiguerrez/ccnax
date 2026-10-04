use dioxus::prelude::*;

use crate::utils::h3_heading;

#[component]
pub fn ChapterIntroductionContent() -> Element {
    rsx! {
        p { class: "mb-4",
            "IPv4 access control lists (ACL) give network engineers the ability to program a filter into a router."
            br {}
            "Each router, on each interface, for both the inbound and outbound direction, can
            enable a different ACL with different rules."
            br {}
            "Each ACL's rules tell the router which packets to discard and which to allow through."
        }

        p { class: "mb-4",
            "This chapter discusses the basics of IPv4 ACLs, and in particular, one type of IP ACL: standard numbered IP ACLs."
            br {}
            strong { "Standard numbered ACLs use simple logic, matching on the source IP address field only" }
            ", and use a configuration style that references the ACL using a number."
            br {}
            "This chapter sets out to help you learn this simpler type of ACL first. The next chapter,
            titled, “Advanced IPv4 Access Control Lists,” completes the discussion by describing other
            types of IP ACLs."
            br {}
            "The other types of ACLs use features that build on the concepts you
            learn in this chapter, but with more complexity and additional configuration options"
        }

        {h3_heading("RECAP")}
        ol { class: "list-disc list-inside",
            li {
                "Each router, on each interface, for both the inbound and outbound direction, can enable a different ACL with 
                different rules."
            }
            li { "Each ACL's rules tell the router which packets to discard and which to allow through." }
            li {
                "Standard numbered ACLs use simple logic, matching on the source IP address field only, and use a configuration style that 
                references the ACL using a number."
            }
        }
    }
}