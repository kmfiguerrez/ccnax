use dioxus::prelude::*;

use crate::utils::h3_heading;

#[component]
pub fn SectionIntroductionContent() -> Element {
    rsx! {
        p { class: "mb-4",
            "When networks use a design that includes redundant routers, switches, LAN links, and
            WAN links, in some cases other protocols are required to take advantage of that redundancy and to prevent problems 
            caused by it."
        }

        {h3_heading("Redundant WAN links")}
        p { class: "mb-4",
            "For instance, imagine a WAN with many remote branch offices."
            br {}
            "If each remote branch has two WAN links connecting it to the rest of the network, those routers can use an IP 
            routing protocol to pick the best routes."
            br {}
            "The routing protocol learns routes over both WAN links, adding the best route into the routing table."
            br {}
            "When the better WAN link fails, the routing protocol adds the alternate route to the IP routing table, taking 
            advantage of the redundant link."
        }

        {h3_heading("Redundant LAN links")}
        p { class: "mb-4",
            "As another example, consider a LAN with redundant links and switches."
            br {}
            "Those LANs have problems unless the switches use Spanning Tree Protocol (STP) or Rapid STP (RSTP)."
            br {}
            "STP/RSTP prevents the problems created by frames that loop through those extra redundant paths in the LAN."
        }

        {h3_heading("Redundancy with default routers")}
        p { class: "mb-4",
            "This section examines yet another type of protocol that helps when a network uses some redundancy, this time with 
            redundant default routers."
            br {}
            "When two or more routers connect to the same LAN subnet, all those routers could be used as the default router for 
            the hosts in the subnet."
            br {}
            "However, to make the best use of the redundant default routers, another protocol is needed."
            br {}
            "The term "
            i { "First Hop Redundancy Protocol" }
            " (FHRP) refers to the category of protocols that can be used so that the hosts take advantage of redundant routers in 
            a subnet."
        }

        p { class: "mb-4",
            "This first major section of the chapter discusses the major concepts behind how different FHRPs work."
            br {}
            "This section begins by discussing a network's need for redundancy in general and the need for redundant default routers."
            br {}
            "It then shows how the three available FHRP options can each solve the problems that occur when using redundant default 
            routers."
        }
    }
}