use dioxus::prelude::*;

use crate::utils::h3_heading;

#[component]
pub fn SectionIntroductionContent() -> Element {
    rsx! {
        p { class: "mb-4",
            "The title of this section serves as a great introduction, if you can decode what Cisco means
            by each specific word."
            br {}
            "This section is about a type of Cisco filter (ACL) that matches only the source IP address of the packet (standard), 
            is configured to identify the ACL using numbers rather than names (numbered), and looks at IPv4 packets."
        }

        p { class: "mb-4",
            "This section examines the particulars of standard numbered IP ACLs."
            br {}
            "First, it examines the idea that one "
            strong { "ACL is a list and what logic that list uses" }
            "."
            br {}
            "Following that, the text closely looks at how to match the source IP address field in the packet header, 
            including the syntax of the commands."
            br {}
            "This section ends with a complete look at the configuration and verification commands to implement standard ACLs."
        }

        {h3_heading("RECAP")}
        ol { class: "list-disc list-inside",
            li { "Standard IP ACL matches only the source IP address of the packet." }
            li {
                "Standard Numbered IP ACL  is configured to identify the ACL using numbers rather than names, 
                and looks at IPv4 packets."
            }
        }
    }
}