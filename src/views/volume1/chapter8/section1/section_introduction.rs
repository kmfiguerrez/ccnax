use dioxus::prelude::*;

use crate::{
    components::KeyTopic,
    utils::h3_heading
};

#[component]
pub fn SectionIntroductionContent() -> Element {
    rsx! {
        p { class: "mb-4",
            "Before understanding VLANs, you must first have a specific understanding of the definition of a LAN."
            br {}
            "For example, from one perspective, a LAN includes all the user devices, servers,
            switches, routers, cables, and wireless access points in one location."
            br {}
            "However, an alternative narrower definition of a LAN can help in understanding the concept of a virtual LAN:"
            br {}
            br {}
            strong { class: "sm:pl-5", "A LAN includes all devices in the same broadcast domain." }
        }

        {h3_heading("Broadcast domains")}
        p { class: "mb-4",
            "A broadcast domain includes the set of all LAN-connected devices, so that when any of the
            devices sends a broadcast frame, all the other devices get a copy of the frame."
            br {}
            "So, from one perspective, you can think of a LAN and a broadcast domain as being basically the same thing."
        }

        {h3_heading("Switches default settings")}
        p { class: "mb-4",
            "Using only default settings, a switch considers all its interfaces to be in the same broadcast domain."
            br {}
            "That is, for one switch, when a broadcast frame entered one switch port, the switch
            forwards that broadcast frame out all other ports."
            br {}
            "With that logic, to create two different LAN broadcast domains, you had to buy two different Ethernet LAN switches, 
            as shown in Figure 8-1."
        }

        img {
            class: "mb-4 rounded-lg",
            alt: "Figure 8-1 Creating Two Broadcast Domains with Two Physical Switches and No VLANs",
            loading: "lazy",
            src: asset!("/assets/static/v1p3c8s1f8-1.png", AssetOptions::image().with_avif()),
        }

        {h3_heading("VLANs")}
        p { class: "mb-4",
            "By using two VLANs, a single switch can accomplish the same goals of the design in Figure
            8-1—to create two broadcast domains—with a single switch."
            br {}
            "With VLANs, a switch can
            configure some interfaces into one broadcast domain and some into another, creating multiple broadcast domains."
            br {}
            "These individual broadcast domains created by the switch are called virtual LANs (VLAN)."
        }

        p { class: "mb-4",
            "For example, in Figure 8-2, the single switch creates two VLANs, treating the ports in each
            VLAN as being completely separate."
            br {}
            "The switch would never forward a frame sent by Dino (in VLAN 1) over to either Wilma or Betty (in VLAN 2)."
        }

        KeyTopic {}
        img {
            class: "mb-4 rounded-lg",
            alt: "Figure 8-2 Creating Two Broadcast Domains Using One Switch and VLANs",
            loading: "lazy",
            src: asset!("/assets/static/v1p3c8s1f8-2.png", AssetOptions::image().with_avif()),
        }

        {h3_heading("Benefits of using VLANs")}
        p {
            "Designing campus LANs to use more VLANs, each with a smaller number of devices, often
            helps improve the LAN in many ways."
            br {}
            "For example, a broadcast sent by one host in a VLAN will be received and processed by all the other hosts in the 
            VLAN—but not by hosts in a different VLAN."
            br {}
            "Limiting the number of hosts that receive a single broadcast frame reduces the number of hosts that waste effort 
            processing unneeded broadcasts."
            br {}
            "It also reduces security risks because fewer hosts see frames sent by any one host."
            br {}
            "These are just a few reasons for separating hosts into different VLANs."
            br {}
            "The following list summarizes the most common reasons for choosing to create smaller broadcast domains (VLANs):"
        }
        KeyTopic {}
        ul { class: "list-disc list-inside mb-4",
            li {
                "To reduce CPU overhead on each device, improving host performance, by reducing the
                number of devices that receive each broadcast frame."
            }
            li {
                "To reduce security risks by reducing the number of hosts that receive copies of frames
                that the switches flood (broadcasts, multicasts, and unknown unicasts)."
            }
            li {
                "To improve security for hosts through the application of different security policies per VLAN"
            }
            li {
                "To create more flexible designs that group users by department, or by groups that work
                together, instead of by physical location."
            }
            li {
                "To solve problems more quickly, because the failure domain for many problems is the
                same set of devices as those in the same broadcast domain."
            }
            li {
                "To reduce the workload for the Spanning Tree Protocol (STP) by limiting a VLAN to a
                single access switch."
            }
        }

        p { class: "mb-4",
            "The rest of this chapter looks closely at the mechanics of how VLANs work across multiple
            Cisco switches, including the required configuration."
            br {}
            "To that end, the next section examines VLAN trunking, a feature required when installing a VLAN that exists on more 
            than one LAN switch."
        }

        {h3_heading("RECAP")}
        ol { class: "list-disc list-inside",
            li {
                "From one perspective, a LAN includes all the user devices, servers, switches, routers, cables, and wireless access points 
                in one location."
            }
            li {
                "An alternative narrower definition of a LAN is a LAN includes all devices in the same broadcast domain."
            }
            li {
                "A broadcast domain includes the set of all LAN-connected devices, so that when any of the devices sends a broadcast frame, 
                all the other devices get a copy of the frame."
            }
            li {
                "Using only default settings, a switch considers all its interfaces to be in the same broadcast domain."
            }
            li {
                "So, from one perspective, you can think of a LAN and a broadcast domain as being basically the same thing."
            }
            li {
                "The individual broadcast domains created by the switch are called virtual LANs (VLAN)."
            }
            li { "By default, Cisco switch's interfaces are all part of VLAN 1." }
            li {
                "So in conclusion, you can think of a LAN, Broadcast domain and a VLAN as being the same thing."
            }
        }
    }
}