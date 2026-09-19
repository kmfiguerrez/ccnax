use dioxus::prelude::*;

use crate::{
    components::GreenNote,
    utils::h3_heading
};

#[component]
pub fn SectionIntroductionContent() -> Element {
    rsx! {
        p { class: "mb-4",
            "The original STP worked well given the assumptions about networks and networking devices
            in that era."
            br {}
            "However, as with any computing or networking standard, as time passes, hardware and software capabilities improve, 
            so new protocols emerge to take advantage of those new capabilities."
            br {}
            "For STP, one of the most significant improvements over time has been the
            introduction of Rapid Spanning Tree Protocol (RSTP), introduced as standard IEEE 802.1w."
        }

        GreenNote {
            p {
                strong { "NOTE" }
                " Just to make sure you are clear about the terminology: Throughout the rest of the
                chapter, STP refers to the original STP standard only, and use of the term RSTP does not
                include STP. "
            }
        }

        {h3_heading("802.1w")}
        p { class: "mb-4",
            "Before getting into the details of RSTP, it helps to make sense of the standards numbers a bit."
            br {}
            strong { "802.1w was actually an amendment to the 802.1D standard." }
            br {}
            "The IEEE first published 802.1D in 1990, and anew in 1998."
            br {}
            "After the 1998 version of 802.1D, the IEEE published the 802.1w amendment to 802.1D in 2001, which first 
            standardized RSTP."
        }

        {h3_heading("802.1Q")}
        p { class: "mb-4",
            "Over the years, other meaningful changes happened in the standards as well, although those
            changes probably do not impact most networkers' thinking when it comes to working with
            STP or RSTP."
            br {}
            strong {
                "But to be complete, the IEEE replaced STP with RSTP in the revised 802.1D standard in 2004."
            }
            br {}
            "In another move, in 2011 the IEEE moved all the RSTP details into a revised 802.1Q standard."
            br {}
            "As it stands today, RSTP actually sits in the 802.1Q standards document."
        }

        p { class: "mb-4",
            "As a result, when reading about RSTP, you will see documents, books, videos, and the like
            that refer to RSTP and include various references to 802.1w, 802.1D, and 802.1Q—and they
            might all be correct based on timing and context."
            br {}
            strong {
                "At the same time, many people refer to RSTP as 802.1w because that was the first IEEE document to define it."
            }
            br {}
            "However, for the purposes of this book, focus instead on the RSTP acronym rather than the IEEE standards
            numbers used with RSTP over its history. "
        }

        GreenNote {
            p {
                strong { "NOTE" }
                " The IEEE sells its standards, but through the “Get IEEE 802” program, you can get
                free PDFs of the current 802 standards. To read about RSTP today, you will need to download the 802.1Q standard, 
                and then look for the sections about RSTP."
            }
        }

        {h3_heading("Details in RSTP")}
        p { class: "mb-4",
            "Now on to the details about RSTP in this chapter. As discussed throughout this chapter,
            RSTP and STP have many similarities, so this section next compares and contrasts the two."
            br {}
            "Following that, the rest of this section discusses the concepts "
            strong {
                "unique to RSTP that are not
                found in STP—alternate root ports, different port states, backup ports, and the port roles
                used by RSTP."
            }
        }

        {h3_heading("RECAP")}
        ol { class: "list-disc list-inside",
            li { "The IEEE 802.1D first defined STP in 1990." }
            li {
                "IEEE published 802.1w as an amendment to 802.1D in 2001, the 802.w was first to define RSTP."
            }
            li { "The IEEE replaced STP with RSTP in the revised 802.1D standard in 2004." }
            li {
                "In another move, in 2011 the IEEE moved all the RSTP details into a revised 802.1Q standard."
            }
            li { "As it stands today, RSTP actually sits in the 802.1Q standards document." }
            li {
                "RSTP has unique concepts that are not found in STP such as: alternate root ports, different port states, 
                backup ports, and the port roles used by RSTP."
            }
        }

    }
}