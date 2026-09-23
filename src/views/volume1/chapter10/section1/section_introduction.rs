use dioxus::prelude::*;

use crate::{
    components::{KeyTopic, GreenNote},
    utils::h3_heading
};

#[component]
pub fn SectionIntroductionContent() -> Element {
    rsx! {
        p { class: "mb-4",
            strong {
                "Cisco IOS switches today typically default to using RSTP rather than STP, with default settings so that RSTP works 
                with no configuration."
            }
            br {}
            "You can buy some Cisco switches and connect them with Ethernet cables in a redundant topology, and RSTP will ensure 
            that frames do not loop."
            br {}
            strong {
                "And even if some switches use RSTP and some use STP, the switches can interoperate and still build a working 
                spanning tree—and you never even have to think about changing any settings!"
            }
        }

        p { class: "mb-4",
            "Although RSTP works without any configuration, most medium-size to large-size campus
            LANs benefit from some STP configuration."
            br {}
            "For instance, Figure 10-1 shows a typical LAN design model, with two distribution layer switches (D1 and D2)."
            br {}
            "The design may have dozens of access layer switches that connect to end users; the figure shows just three access
            switches (A1, A2, and A3)."
            br {}
            "For a variety of reasons, most network engineers make the distribution layer switches be the root. "
        }

        KeyTopic {}
        img {
            class: "mb-4 rounded-lg",
            alt: "Figure 10-1 Typical Configuration Choice: Making Distribution Switch Be Root",
            loading: "lazy",
            src: asset!("/assets/static/v1p3c10s1f10-1.png", AssetOptions::image().with_avif()),
        }

        GreenNote {
            p {
                strong { "NOTE" }
                " Cisco uses the term "
                i { "access switch" }
                " to refer to switches used to connect to endpoint
                devices."
                " The term "
                i { "distribution switch" }
                " refers to switches that do not connect to endpoints
                but rather connect to each access switch, providing a means to distribute frames throughout the LAN."
                " If you want to read more about LAN design concepts and terms, refer to this
                book's companion website for Appendix K, “Analyzing Ethernet LAN Designs.”"
            }
        }

        p { class: "mb-4",
            "As discussed in the introduction to this chapter, this first section of the chapter examines
            a variety of STP/RSTP configuration topics, but with a goal of revealing a few more details
            about how STP and RSTP operate."
            br {}
            "Following this opening section about RSTP configuration,
            the next section examines how to configure Layer 2 EtherChannels, and how that impacts
            STP/RSTP."
        }

        {h3_heading("RECAP")}
        ol { class: "list-disc list-inside",
            li {
                "Cisco IOS switches today typically default to using RSTP rather than STP, with default settings so that 
                RSTP works with no configuration."
            }
            li {
                "You can buy some Cisco switches and connect them with Ethernet cables in a redundant topology, and RSTP will 
                ensure that frames do not loop."
            }
            li {
                "Even if some switches use RSTP and some use STP, the switches can interoperate and still build a working spanning 
                tree—and you never even have to think about changing any settings!"
            }
            li {
                " Cisco uses the term "
                i { "access switch" }
                " to refer to switches used to connect to endpoint
                devices."
            }
            li {
                "The term "
                i { "distribution switch" }
                " refers to switches that do not connect to endpoints
                but rather connect to each access switch, providing a means to distribute frames throughout the LAN."
            }
        }
    }
}
