use dioxus::prelude::*;

use crate::utils::h3_heading;

#[component]
pub fn ChapterIntroductionContent() -> Element {
    rsx! {
        p { class: "mb-4",
            "When reading this chapter, think of it as three separate small topics rather than one large topic."
            br {}
            "The content just happens to include a few IP-based services that have little to do with
            each other, but the length of coverage of each topic is too short to justify a separate chapter."
            br {}
            "The result: Chapter 12, “Miscellaneous IP Services.”"
            br {}
            "So when reading, feel free to treat each of the three major headings as a separate study event."
        }

        {h3_heading("First Hop Redundancy Protocols")}
        p { class: "mb-4",
            "First Hop Redundancy Protocols (FHRPs), which provides redundancy for the function of
            the default router in any subnet, begins the chapter."
            br {}
            "The term FHRP refers to a class of solutions, with three options, and with the examples showing the most popular 
            option, Hot Standby Router Protocol (HSRP)."
        }

        {h3_heading("Simple Network Management Protocol")}
        p { class: "mb-4",
            "Simple Network Management Protocol (SNMP) follows in the second major section."
            br {}
            "As per the associated exam topic, this section focuses on SNMP concepts rather than configuration, including how 
            managed devices—SNMP agents—can be interrogated by network management systems—SNMP clients—to find the current status 
            of each device."
        }

        {h3_heading("FTP/TFTP")}
        p { class: "mb-4",
            "File Transfer Protocol (FTP) and Trivial File Transfer Protocol (TFTP) star in the third major section."
            br {}
            "The first branch of this section focuses on a few practical uses of TFTP and FTP,
            specifically how to use these protocols on Cisco routers to upgrade the IOS."
            br {}
            "Armed with that practical knowledge, you then look at the protocol details of both FTP and TFTP in the
            rest of the section."
        }
    }
}