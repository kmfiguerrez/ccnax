use dioxus::prelude::*;

use crate::utils::h3_heading;

#[component]
pub fn ChapterIntroductionContent() -> Element {
    rsx! {
        p { class: "mb-4",
            "By now you have learned a lot about Ethernet and Ethernet switches."
            br {}
            "You have learned how individual links work, with cabling and duplex settings as well as framing."
            br {}
            "You know how addresses work and how switches forward frames based on those addresses."
            br {}
            "You have seen how switches deal with redundancy, using STP/RSTP and collecting links into
            EtherChannels."
            br {}
            "And here in Volume 2, you have learned about a variety of security features available for switches, including 
            Dynamic ARP Inspection, DHCP Snooping, and ARP Inspection."
        }

        p { class: "mb-4",
            "What the earlier discussions of individual features do not do to any great extent is discuss
            architecture and design."
            br {}
            strong {
                "You now know how switches work, but why would you connect switches in one topology versus another?"
            }
            br {}
            "If you could connect switches in two different topologies, why would you prefer one over the other?"
            br {}
            "This chapter examines a few such design questions, specifically the topic areas mentioned in the CCNA 200-301 exam 
            topics."
            br {}
            "("
            strong {
                "Note that the CCNA 200-301 exam does not include a comprehensive look at LAN design
                issues, but one of the current CCNP Enterprise exams does"
            }
            ".)"
        }

        p { class: "mb-4",
            "This chapter covers three specific topics that have design-related considerations."
            br {}
            "The first section looks at the topology of a wired Ethernet LAN and the design terms two tier and
            three tier, which describe how many switch layers exist between the endpoints and the
            devices that lead out of the campus to some other site."
            br {}
            "Following that, the second section examines small office/home office (SOHO) LANs and how they differ from enterprise
            LANs."
            br {}
            "The final section introduces the concepts behind Power over Ethernet (PoE), along with the reasons why LAN design 
            activities need to consider PoE."
        }
    }
}