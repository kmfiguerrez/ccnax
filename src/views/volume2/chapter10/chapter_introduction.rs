use dioxus::prelude::*;

#[component]
pub fn ChapterIntroductionContent() -> Element {
    rsx! {
        p { class: "mb-4",
            "This chapter examines a very popular and very important part of both enterprise and small
            office/home office (SOHO) networks: Network Address Translation , or NAT."
            br {}
            strong { "NAT helped solve a big problem with IPv4" }
            ": the IPv4 address space would have been completely consumed by the mid-1990s."
            br {}
            "After it was consumed, the Internet could not continue to grow, which would have significantly slowed the development 
            of the Internet."
        }

        p { class: "mb-1", "This chapter breaks the topics into three major sections." }
        ol { class: "list-disc list-inside mb-4",
            li {
                "The first section explains the challenges to the IPv4 address space caused by the Internet revolution of the 1990s."
            }
            li {
                "The second section explains the basic concept behind NAT, how several variations of NAT work, and
                how the Port Address Translation (PAT) option conserves the IPv4 address space."
            }
            li {
                "The final section shows how to configure NAT from the Cisco IOS Software command-line interface
                (CLI) and how to troubleshoot NAT."
            }
        }
    }
}