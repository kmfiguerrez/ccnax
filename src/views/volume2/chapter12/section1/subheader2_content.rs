use dioxus::prelude::*;

use crate::{utils::h3_heading, components::KeyTopic};

#[component]
pub fn Content() -> Element {
    rsx! {
        p { class: "mb-4",
            "Of the designs shown so far in this chapter, only Figure 12-3's design has two routers to
            support the LAN on the left side of the figure, specifically the same VLAN and subnet."
            br {}
            "While having the redundant routers on the same subnet helps, the network needs to use an
            FHRP when these redundant routers exist."
        }

        {h3_heading("Redundant routers without FHRP")}
        p { class: "mb-1",
            "To see the need and benefit of using an FHRP, first think about how these redundant routers could be used as default 
            routers by the hosts in VLAN 10/subnet 10.1.1.0/24, as shown in Figure 12-4."
            br {}
            strong {
                "The host logic will remain unchanged, so each host has a single default router setting."
            }
            br {}
            "So, some design options for default router settings include the following:"
        }
        ol { class: "list-disc list-inside mb-4",
            li {
                "All hosts in the subnet use R1 (10.1.1.9) as their default router, and they statically reconfigure their 
                default router setting to R2's 10.1.1.129 if R1 fails."
            }
            li {
                "All hosts in the subnet use R2 (10.1.1.129) as their default router, and they statically
                reconfigure their default router setting to R1's 10.1.1.9 if R2 fails."
            }
            li {
                "Half the hosts use R1, and half use R2, as their default router, and if either router fails,
                that half of the users statically reconfigure their default router setting."
            }
        }

        p { class: "mb-4",
            "To make sure the concept is clear, Figure 12-4 shows this third option, with half the hosts
            using R1 and the other half using R2."
            br {}
            "The figure removes all the LAN switches just to unclutter the figure."
            br {}
            "Hosts A and B use R1 as their default router, and hosts C and D use R2 as their default router."
        }

        img {
            class: "mb-4 rounded-lg",
            loading: "lazy",
            alt: "Figure 12-4 Balancing Traffic by Assigning Different Default Routers to Different Clients",
            src: asset!("/assets/static/v2p3c12s1sh2f12-4.png", AssetOptions::image().with_avif()),
        }

        p { class: "mb-4",
            strong { "All of these options have a problem: the users have to take action." }
            br {}
            "They have to know an outage occurred."
            br {}
            "They have to know how to reconfigure their default router setting."
            br {}
            "And they have to know when to change it back to the original setting."
        }

        {h3_heading("Redundant routers using FHRP")}
        p { class: "mb-4",
            "FHRPs make this design work better."
            br {}
            "The two routers appear to be a single default router."
            br {}
            "The users never have to do anything: their default router setting remains the same, and their
            ARP table even remains the same."
        }

        KeyTopic {}
        p { class: "mb-1",
            "To allow the hosts to remain unchanged, the routers have to do some more work, as defined
            by one of the FHRP protocols."
            br {}
            "Generically, each FHRP makes the following happen:"
        }
        ol { class: "list-decimal list-inside mb-4",
            li {
                "All hosts act like they always have, with one default router setting that never has to change."
            }
            li { "The default routers share a virtual IP address in the subnet, defined by the FHRP." }
            li { "Hosts use the FHRP virtual IP address as their default router address." }
            li {
                "The routers exchange FHRP protocol messages so that both agree as to which router does what work at any point 
                in time."
            }
            li {
                "When a router fails or has some other problem, the routers use the FHRP to choose
                which router takes over responsibilities from the failed router."
            }
        }

        {h3_heading("RECAP")}
        ol { class: "list-disc list-inside",
            li {
                "First Hop Redundancy Protocols make redundant routers appear as a single default router."
                " The users never have to do anything when an outage occur: their default router setting remains the same, and their ARP 
                table even remains the same."
            }
            li {
                "Without FHRPs, users have to know an outage occured and they have to reconfigure their default router setting."
            }
        }
    }
}