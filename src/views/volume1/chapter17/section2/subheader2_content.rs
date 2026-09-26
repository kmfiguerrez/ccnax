use dioxus::prelude::*;

use crate::components::GreenNote;


#[component]
pub fn Content() -> Element {
    rsx! {
        p { class: "mb-4",
            "With the VLAN configuration shown in the previous section, the switch is ready to route
            packets between the VLANs as shown in Figure 17-3."
            br {}
            strong {
                "To support the routing of packets, the switch adds connected IP routes as shown in Example 17-7; note that each route 
                is listed as being connected to a different VLAN interface."
            }
        }

        img {
            class: "mb-4 rounded-lg",
            alt: "Example 17-7 Connected Routes on a Layer 3 Switch",
            loading: "lazy",
            src: asset!("/assets/static/v1p3c17s2sh2ex17-7.png", AssetOptions::image().with_avif()),
        }

        p { class: "mb-4",
            "The switch would also need additional routes to the rest of the network (not shown in the
            figures in this chapter)."
            br {}
            "The Layer 3 switch could use static routes or a routing protocol, depending on the capabilities of the switch."
            br {}
            "For instance, if you then enabled OSPF on the Layer 3 switch, the configuration and verification would work the 
            same as it does on a router, as discussed in Chapter 20, “Implementing OSPF.”"
            br {}
            strong {
                "The routes that IOS adds to the Layer 3 switch's IP routing table would list the VLAN interfaces as outgoing interfaces."
            }
        }

        GreenNote {
            p {
                strong { "NOTE" }
                " Some models of Cisco enterprise switches, based on model, IOS version, and IOS
                feature set, support different capabilities for IP routing and routing protocols, so for real networks, 
                check the capabilities of the switch model by browsing at Cisco.com."
            }
        }
    }
}