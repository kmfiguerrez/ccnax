use dioxus::prelude::*;

use crate::utils::h3_heading;

#[component]
pub fn SectionIntroductionContent() -> Element {
    rsx! {
        p { class: "mb-4",
            "Almost all enterprise networks use VLANs."
            br {}
            "To route IP packets in and out of those VLANs, some devices (either routers or Layer 3 switches) need to have an 
            IP address in each subnet and have a connected route to each of those subnets."
            br {}
            "Then the IP addresses on those routers or Layer 3 switches can serve as the default gateways in those subnets."
        }

        p { class: "mb-1", "This chapter breaks down the LAN routing options into four categories:" }

        ol { class: "list-disc list-inside mb-4",
            li {
                "Use a router, with one router LAN interface and cable connected to the switch for each
                and every VLAN (typically not used)"
            }
            li {
                "Use a router, with a VLAN trunk connecting to a LAN switch (known as router-on-astick, or ROAS)"
            }
            li { "Use a Layer 3 switch with switched virtual interfaces (SVI)" }
            li {
                "Use a Layer 3 switch with routed interfaces (which may or may not be Layer 3 EtherChannels)"
            }
        }

        p { class: "mb-4",
            "Of the items in the list, the first option works, but to be practical, it requires far too many interfaces."
            br {}
            "It is mentioned here only to make the list complete."
        }

        p { class: "mb-4",
            "As for the other three options, this chapter discusses each in turn as the main focus of one
            of the three major sections in this chapter."
            br {}
            "Each feature is used in real networks today, with the choice to use one or the other driven by the design and needs 
            for a particular part of the network."
            br {}
            "Figure 17-1 shows cases in which these options could be used."
        }

        img {
            class: "mb-4 rounded-lg",
            alt: "Figure 17-1 Layer 3 Switching at the Central Site",
            loading: "lazy",
            src: asset!("/assets/static/v1p3c17s1f17-1.png", AssetOptions::image().with_avif()),
        }

        {h3_heading("Campus LAN")}
        p { class: "mb-4",
            "Figure 17-1 shows two switches, labeled A and B, which could act as Layer 3 switches—both
            with SVIs and routed interfaces."
            br {}
            "The figure shows a central site campus LAN on the left, with 12 VLANs."
            br {}
            "Switches A and B act as Layer 3 switches, combining the functions of a router and a switch, routing between all 
            12 subnets/VLANs, as well as routing to/from the Core router."
            br {}
            "Those Layer 3 switches could use SVIs, routed interfaces, or both."
        }

        {h3_heading("Small remote sites")}
        p { class: "mb-4",
            "Figure 17-1 also shows a classic case for using a router with a VLAN trunk."
            br {}
            "Sites like the remote sites on the right side of the figure may have a WAN-connected router and a LAN switch."
            br {}
            strong {
                "These sites might use ROAS to take advantage of the router's ability to route over an 802.1Q trunk."
            }
        }

        p { class: "mb-4",
            "Note that Figure 17-1 just shows an example."
            br {}
            strong {
                "The engineer could use Layer 3 switching at each site or routers with VLAN trunking at each site."
            }
        }

        {h3_heading("RECAP")}
        ol { class: "list-disc list-inside",
            li {
                "To route IP packets in and out of VLANs, some devices (either routers or Layer 3 switches) need to have an 
                IP address in each subnet and have a connected route to each of those subnets."
            }
            li {
                "Then the IP addresses on those routers or Layer 3 switches can serve as the default gateways in those subnets."
            }
        }

    }
}