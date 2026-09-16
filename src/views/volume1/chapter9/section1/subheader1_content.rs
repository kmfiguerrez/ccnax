use dioxus::prelude::*;

use crate::{
    components::{KeyTopic, GreenNote},
    utils::h3_heading
};

#[component]
pub fn Content() -> Element {
    rsx! {
        p { class: "mb-4",
            strong { "STP/RSTP prevents three common problems in Ethernet LANs." }
            br {}
            "All three problems occur as a side effect of one fact: without STP/RSTP, some Ethernet frames would loop around the 
            network for a long time (hours, days, literally forever if the LAN devices and links never failed)."
        }

        {h3_heading("First problem: Broadcast storms")}
        p { class: "mb-4",
            "Just one looping frame causes what is called a "
            i { "broadcast storm" }
            "."
            br {}
            strong {
                "Broadcast storms happen when any kind of Ethernet frames—broadcast frames, multicast frames, or 
                unknown-destination unicast frames—loop around a LAN indefinitely."
            }
            br {}
            "Broadcast storms can saturate all the links with copies of that one single frame, crowding out good frames, as 
            well as significantly impacting end-user device performance by making the PCs process too many broadcast frames."
        }

        p { class: "mb-4",
            "To help you understand how this occurs, Figure 9-1 shows a sample network in which Bob
            sends a broadcast frame."
            br {}
            "The dashed lines show how the switches forward the frame when STP/RSTP does not exist."
        }

        img {
            class: "mb-4 rounded-lg",
            alt: "Example 8-12 Operational Trunking State",
            loading: "lazy",
            src: asset!("/assets/static/v1p3c9s1sh1f9-1.png", AssetOptions::image().with_avif()),
        }

        GreenNote {
            p {
                strong { "NOTE" }
                " Bob's original broadcast would also be forwarded around the other direction, with
                SW3 sending a copy of the original frame out its Gi0/1 port."
                " To reduce clutter, Figure 9-1 does not show that frame."
            }
        }

        p { class: "mb-4",
            "Remember that LAN switch? That logic tells switches to flood broadcasts out all interfaces
            in the same VLAN except the interface in which the frame arrived."
            "In Figure 9-1, that means SW3 forwards Bob's frame to SW2, SW2 forwards the frame to SW1, SW1 
            forwards the frame back to SW3, and SW3 forwards it back to SW2 again."
        }

        p { class: "mb-4",
            "When broadcast storms happen, frames like the one in Figure 9-1 keep looping until something changes—someone 
            shuts down an interface, reloads a switch, or does something else to break the loop."
            br {}
            "Also note that the same event happens in the opposite direction. When Bob sends the original frame, SW3 also 
            forwards a copy to SW1, SW1 forwards it to SW2, and so on."
        }

        {h3_heading("Second problem: MAC table instability")}
        p { class: "mb-4",
            "The storm also causes a much more subtle problem called "
            i { "MAC table instability" }
            "."
            br {}
            "MAC table instability means that the switches' MAC address tables keep changing because frames
            with the same source MAC arrive on different ports."
            br {}
            "To see why, follow this example, in which SW3 begins Figure 9-1 with a MAC table entry for Bob, at the 
            bottom of the figure, associated with port Fa0/13:"
            div { class: "pl-5 flex gap-x-5",
                span { "0200.3333.3333" }
                span { "Fa0/13" }
                span { "VLAN 1" }
            }
        }

        p { class: "mb-4",
            "However, now think about the switch-learning process that occurs when the looping frame
            goes to SW2, then SW1, and then back into SW3's Gi0/1 interface."
            br {}
            "SW3 thinks, “Hmm…the source MAC address is 0200.3333.3333, and it came in my Gi0/1 interface."
            br {}
            "Update my MAC table!” This results in the following entry on SW3, with interface Gi0/1 instead of Fa0/13:"
            div { class: "pl-5 flex gap-x-5",
                span { "0200.3333.3333" }
                span { "Gi0/1" }
                span { "VLAN 1" }
            }
        }

        p { class: "mb-4",
            "At this point, SW3 itself cannot correctly deliver frames to Bob's MAC address."
            br {}
            "At that instant, if a frame arrives at SW3 destined for Bob—a different frame than the looping frame
            that causes the problems—SW3 incorrectly forwards the frame out Gi0/1 to SW1, creating
            even more congestion."
        }

        {h3_heading("Third problem: Multiple frame transmission")}
        p { class: "mb-4",
            "The looping frames in a broadcast storm also cause a third problem: multiple copies of the
            frame arrive at the destination."
            br {}
            "Consider a case in which Bob sends a frame to Larry but none of the switches know Larry's MAC address."
            br {}
            "Switches flood frames sent to unknown destination unicast MAC addresses."
            br {}
            "When Bob sends the frame destined for Larry's MAC address, SW3 sends a copy to both SW1 and SW2."
            br {}
            "SW1 and SW2 also flood the frame, causing copies of the frame to loop."
            br {}
            "SW1 also sends a copy of each frame out Fa0/11 to Larry."
            br {}
            "As a result, Larry gets multiple copies of the frame, which may result in an application
            failure, if not more pervasive networking problems."
        }

        p { class: "mb-4",
            "Table 9-2 summarizes the main three classes of problems that occur when STP/RSTP is not
            used in a LAN that has redundancy."
        }

        KeyTopic {}
        img {
            class: "mb-4 rounded-lg",
            alt: "Example 8-12 Operational Trunking State",
            loading: "lazy",
            src: asset!("/assets/static/v1p3c9s1sh1t9-2.png", AssetOptions::image().with_avif()),
        }

        {h3_heading("RECAP")}
        ol { class: "list-disc list-inside",
            li {
                "STP/RSTP prevents three common problems in Ethernet LANs with redundant links: Broadcast storms, 
                MAC table instability, Multiple frame transmission."
            }
            li {
                "Broadcast storms happen when any kind of Ethernet frames—broadcast frames, multicast frames, or unknown-destination 
                unicast frames—loop around a LAN indefinitely."
            }
            li {
                "MAC table instability means that the switches' MAC address tables keep changing because frames with the same 
                source MAC arrive on different ports."
            }
            li {
                "Multiple frame transmission is a side effect of looping frames in which multiple copies of one frame
                are delivered to the intended host, confusing the host 
                (may result in an application failure, if not more pervasive networking problems)."
            }
        }
    }
}