use dioxus::prelude::*;

use crate::utils::h3_heading;

#[component]
pub fn ChapterIntroductionContent() -> Element {
    rsx! {
        p { class: "mb-4",
            "This chapter shows how to configure Rapid Spanning Tree Protocol (RSTP) and Layer 2
            EtherChannels."
            br {}
            "The EtherChannel content, in the second major section of the chapter, follows
            a typical flow for most configuration/verification topics in a certification guide: it reviews
            concepts, shows configurations, and provides show commands that point out the configuration
            settings and operational state."
            br {}
            "The details include how to manually configure a channel, how to
            cause a switch to dynamically create a channel, and how EtherChannel load distribution works."
        }

        p { class: "mb-4",
            "The first section of the chapter explores RSTP implementation taking a different approach."
            br {}
            strong {
                "Cisco mentions RSTP concepts, but not configuration/verification, in the CCNA exam topics."
            }
            br {}
            "However, to get a real sense of RSTP concepts, especially some concepts specific to
            Cisco Catalyst switches, you need to work with RSTP configuration and verification."
            br {}
            "The first section of the chapter explores RSTP implementation, but as a means to the end of more
            fully understanding RSTP concepts. "
        }

        p { "See Appendix O on page 1009 in pdf book, to practice RSTP/STP configuration/verification." }
    }
}