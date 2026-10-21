use dioxus::prelude::*;

use crate::{utils::h3_heading, components::{KeyTopic, GreenNote}};

#[component]
pub fn Content() -> Element {
    rsx! {
        p { class: "mb-4",
            "The term "
            i { "First Hop Redundancy Protocol" }
            " does not name any one protocol."
            br {}
            "Instead, it names a family of protocols that fill the same role."
            br {}
            "For a given network, like the left side of Figure 12-4, the engineer would pick one of the protocols from the FHRP 
            family."
        }

        GreenNote {
            p {
                strong { "NOTE" }
                i { " First Hop" }
                " is a reference to the default router being the first router, or first router
                hop, through which a packet must pass."
            }
        }

        p { class: "mb-4",
            "Table 12-2 lists the three FHRP protocols in chronological order, based on when these were first used."
            br {}
            "Cisco first introduced the proprietary Hot Standby Router Protocol (HSRP), and it worked well for many of its customers."
            br {}
            "Later, the IETF developed an RFC for a similar protocol, Virtual Router Redundancy Protocol (VRRP)."
            br {}
            "Finally, Cisco developed a more robust option, Gateway Load Balancing Protocol (GLBP)."
        }

        KeyTopic {}
        img {
            class: "mb-4 rounded-lg",
            loading: "lazy",
            alt: "Table 12-2 Three FHRP Options",
            src: asset!("/assets/static/v2p3c12s1sh3t12-2.png", AssetOptions::image().with_avif()),
        }

        p { class: "mb-4",
            strong {
                "This chapter focuses on HSRP and does not discuss VRRP and GLBP other than this brief mention."
            }
            br {}
            "HSRP, the first of the three FHRP protocols to enter the market, remains a popular option in many networks."
            br {}
            "The current CCNA 200-301 exam requires you to know the functions of an FHRP, so the example of HSRP meets that need, 
            with the next few pages walking through the concepts of how HSRP works."
            br {}
            "("
            strong {
                "Note that Appendix D, “Topics from Previous Editions,” contains a section with more depth about GLBP, copied from an 
                earlier edition of the book, as well as a section on HSRP configuration if you are interested in reading more that goes 
                beyond the current exam's topics"
            }
            ".)"
        }

        {h3_heading("RECAP")}
        ol { class: "list-disc list-inside",
            li {
                "The term "
                i { "First Hop Redundancy Protocol" }
                " does not name any one protocol."
                " Instead, it names a family of protocols that fill the same role."
            }
        }
    }
}