use dioxus::prelude::*;

use crate::{
    components::{KeyTopic, GreenNote},
    utils::h3_heading
};

#[component]
pub fn SectionIntroductionContent() -> Element {
    rsx! {
        p { class: "mb-4",
            "Because autonomous APs are…well, autonomous, managing their RF operation can be quite difficult."
            br {}
            "As a network administrator, you are in charge of selecting and configuring the channel used by each AP and detecting 
            and dealing with any rogue APs that might be interfering."
            br {}
            strong {
                "You must also manage things such as the transmit power level to make sure that the wireless coverage is sufficient, 
                it does not overlap too much, and there aren't any coverage holes—even when an AP's radio fails."
            }
        }

        {h3_heading("Wireless network security")}
        p { class: "mb-4",
            strong { "Managing wireless network security can also be difficult." }
            br {}
            "Each autonomous AP handles its own security policies, with no central point of entry between the wireless and wired 
            networks."
            br {}
            "That means there is no convenient place to monitor traffic for things such as intrusion detection and prevention, 
            quality of service, bandwidth policing, and so on."
        }

        {h3_heading("Dividing autonomous APs functions")}
        p { class: "mb-4",
            "To overcome the limitations of distributed autonomous APs, many of the functions found
            within autonomous APs have to be shifted toward some central location."
            br {}
            "In Figure 27-4, most of the activities performed by an autonomous AP on the left are broken up into two
            groups—management functions on the top and real-time processes on the bottom."
        }

        KeyTopic {}
        img {
            class: "mb-4 rounded-lg",
            alt: "Figure 27-4 Autonomous Versus Lightweight Access Point",
            loading: "lazy",
            src: asset!("/assets/static/v1p8c27s3f27-4.png", AssetOptions::image().with_avif()),
        }

        {h3_heading("The real-time functions")}
        p { class: "mb-4",
            "The real-time processes involve sending and receiving 802.11 frames, beacons, and probe messages."
            br {}
            "802.11 data encryption is also handled in real time, on a per-packet basis."
            br {}
            "The AP must interact with wireless clients on some low level, known as the Media Access Control (MAC) layer."
            br {}
            "These functions must stay with the AP hardware, closest to the clients."
        }

        {h3_heading("The management functions")}
        p { class: "mb-4",
            "The management functions are not integral to handling frames over the RF channels, but are things that should be 
            centrally administered."
            br {}
            "Therefore, those functions can be moved to a centrally located platform away from the AP."
        }

        {h3_heading("Light-weight Access Points")}
        p { class: "mb-4",
            "When the functions of an autonomous AP are divided, the AP hardware is known as a "
            i { "lightweight access point" }
            ", and performs only the real-time 802.11 operation."
            br {}
            "The lightweight AP gets its name because the code image and the local intelligence are stripped down, or lightweight, 
            compared to the traditional autonomous AP."
        }

        {h3_heading("Wireless LAN Controller (WLC)")}
        p { class: "mb-4",
            "The management functions are usually performed on a "
            i { "wireless LAN controller" }
            " (WLC), which controls many lightweight APs."
            br {}
            "This is shown in the bottom right portion of Figure 27-4."
            br {}
            strong {
                "Notice that the AP is left with duties in Layers 1 and 2,  where frames are moved into and out of the RF domain."
            }
            br {}
            "The AP becomes totally dependent on the WLC for every other WLAN function, such as authenticating users, managing 
            security policies, and even selecting RF channels and output power."
        }

        GreenNote {
            p {
                strong { "NOTE" }
                " Remember that a lightweight AP cannot normally operate on its own; it is very
                dependent on a WLC somewhere in the network. The only exception is the FlexConnect
                architecture, which is discussed later in this chapter."
            }
        }

        p { class: "mb-4",
            "The lightweight AP-WLC division of labor is known as a "
            i { "split-MAC architecture" }
            ", where the normal MAC operations are pulled apart into two distinct locations."
            br {}
            strong {
                "This occurs for every AP in the network; each one must boot and bind itself to a WLC to support wireless clients."
            }
            br {}
            "The WLC becomes the central hub that supports a number of APs scattered about in the network."
        }

        {h3_heading("WLC and APs locations")}
        p { class: "mb-4",
            "How does a lightweight AP bind with a WLC to form a complete working access point?"
            br {}
            strong {
                "The two devices must use a tunneling protocol between them, to carry 802.11-related messages and also client data."
            }
            br {}
            strong {
                "Remember that the AP and WLC can be located on the same VLAN or IP subnet, but they do not have to be."
            }
            br {}
            "Instead, they can be located on two entirely different IP subnets in two entirely different locations."
        }

        {h3_heading("CAPWAP")}
        p { class: "mb-1",
            "The Control and Provisioning of Wireless Access Points ("
            i { "CAPWAP" }
            ") tunneling protocol makes this all possible by 
            encapsulating the data between the LAP and WLC within new IP packets."
            br {}
            "The tunneled data can then be switched or routed across the campus network."
            br {}
            "As Figure 27-5 shows, the CAPWAP relationship actually consists of two separate tunnels, as follows:"
        }
        ol { class: "list-disc list-inside mb-4",
            li {
                span { class: "font-bold mr-1", "CAPWAP control messages:" }
                "Carries exchanges that are used to configure the AP and manage its operation."
                " The control messages are authenticated and encrypted, so the AP is securely controlled by only the appropriate 
                WLC, then transported over the control tunnel."
            }
            li {
                span { class: "font-bold mr-1", "CAPWAP data:" }
                "Used for packets traveling to and from wireless clients that are associated with the AP."
                " Data packets are transported over the data tunnel but are not encrypted by default."
                " When data encryption is enabled for an AP, packets are protected with Datagram Transport Layer Security (DTLS)."
            }
        }

        GreenNote {
            p {
                strong { "NOTE" }
                " CAPWAP is defined in RFCs 5415, 5416, 5417, and 5418. CAPWAP is based on the
                Lightweight Access Point Protocol (LWAPP), which was a legacy Cisco proprietary solution."
            }
        }

        KeyTopic {}
        img {
            class: "mb-4 rounded-lg",
            alt: "Figure 27-5 Linking a Lightweight AP and WLC with CAPWAP",
            loading: "lazy",
            src: asset!("/assets/static/v1p8c27s3f27-5.png", AssetOptions::image().with_avif()),
        }

        {h3_heading("Digital certificates")}
        p { class: "mb-4",
            strong { "Every AP and WLC must also authenticate each other with digital certificates." }
            br {}
            "An X.509 certificate is preinstalled in each device when it is purchased."
            br {}
            "By using certificates behind the scenes, every device is properly authenticated before becoming part of the wireless 
            network."
            br {}
            strong { "This process helps assure that no one can add an unauthorized AP to your network." }
        }

        {h3_heading("CAPWAP tunneling")}
        p { class: "mb-4",
            strong {
                "The CAPWAP tunneling allows the AP and WLC to be separated geographically and logically."
            }
            br {}
            "It also breaks the dependence on Layer 2 connectivity between them."
            br {}
            "For example, Figure 27-6 uses shaded areas to show the extent of VLAN 100."
            br {}
            "Notice how VLAN 100 exists at the WLC and in the air as SSID 100, near the wireless clients—but not in between the AP
            and the WLC."
            br {}
            "Instead, traffic to and from clients associated with SSID 100 is transported across the network infrastructure 
            encapsulated inside the CAPWAP data tunnel."
            br {}
            strong {
                "The tunnel exists between the IP address of the WLC and the IP address of the AP, which allows all of
                the tunneled packets to be routed at Layer 3."
            }
        }

        p { class: "mb-4",
            "Also, notice how the AP is known by only a single IP address: 10.10.10.10."
            br {}
            "Because the AP sits on the access layer where its CAPWAP tunnels terminate, it can use one IP address for
            both management and tunneling."
            br {}
            strong {
                "No trunk link is needed because all of the VLANs it supports are encapsulated and tunneled as Layer 3 IP packets, 
                rather than individual Layer 2 VLANs."
            }
        }

        p { class: "mb-4",
            strong {
                "As the wireless network grows, the WLC simply builds more CAPWAP tunnels to reach more APs."
            }
            br {}
            "Figure 27-7 depicts a network with four APs."
            br {}
            "Each AP has a control and a data tunnel back to the centralized WLC."
            br {}
            strong {
                "SSID 100 can exist on every AP, and VLAN 100 can reach every AP through the network of tunnels."
            }
        }

        img {
            class: "mb-4 rounded-lg",
            alt: "Figure 27-6 Extent of VLAN 100 in a Cisco Wireless Network",
            loading: "lazy",
            src: asset!("/assets/static/v1p8c27s3f27-6.png", AssetOptions::image().with_avif()),
        }

        img {
            class: "mb-4 rounded-lg",
            alt: "Figure 27-7 Using CAPWAP Tunnels to Connect APs to One Central WLC",
            loading: "lazy",
            src: asset!("/assets/static/v1p8c27s3f27-7.png", AssetOptions::image().with_avif()),
        }

        p { class: "mb-1",
            "Once CAPWAP tunnels are built from a WLC to one or more lightweight APs, the WLC can begin offering a variety of 
            additional functions."
            br {}
            "Think of all the puzzles and shortcomings that were discussed for the traditional autonomous WLAN architecture as 
            you read over the following list of WLC activities:"
        }
        ol { class: "list-disc list-inside mb-4",
            li {
                span { class: "font-bold mr-1", "Dynamic channel assignment:" }
                "The WLC can automatically choose and configure the RF channel used by each AP, based on other active access 
                points in the area."
            }
            li {
                span { class: "font-bold mr-1", "Transmit power optimization:" }
                "The WLC can automatically set the transmit power of each AP based on the coverage area needed."
            }
            li {
                span { class: "font-bold mr-1", "Self-healing wireless coverage:" }
                "If an AP radio dies, the coverage hole can be “healed” by turning up the transmit power of surrounding APs 
                automatically."
            }
            li {
                span { class: "font-bold mr-1", "Flexible client roaming:" }
                "Clients can roam between APs with very fast roaming times."
            }
            li {
                span { class: "font-bold mr-1", "Dynamic client load balancing:" }
                "If two or more APs are positioned to cover the same geographic area, the WLC can associate clients with the least 
                used AP. This distributes the client load across the APs."
            }
            li {
                span { class: "font-bold mr-1", "RF monitoring:" }
                "The WLC manages each AP so that it scans channels to monitor the RF usage."
                br {}
                "By listening to a channel, the WLC can remotely gather information about RF interference, noise, signals from 
                neighboring APs, and signals from rogue APs or ad hoc clients."
            }
            li {
                span { class: "font-bold mr-1", "Security management:" }
                "The WLC can authenticate clients from a central service and can require wireless clients to obtain an IP address 
                from a trusted DHCP server before allowing them to associate and access the WLAN."
            }
            li {
                span { class: "font-bold mr-1", "Wireless intrusion protection system:" }
                "Leveraging its central location, the WLC can monitor client data to detect and prevent malicious activity."
            }
        }

        {h3_heading("RECAP")}
        ol { class: "list-disc list-inside",
            li {
                "In Split-MAC architecture, the functions of an autonomous AP is divided into two groups: management and real-time 
                functions. The management functions are given to the WLC and the real-time functions stay wtih the AP."
            }
            li {
                "When the functions of an autonomous AP are divided, the AP hardware is known as a "
                i { "lightweight access point" }
                ", and performs only the real-time 802.11 operation."
            }
            li {
                "Remember that a lightweight AP cannot normally operate on its own; it is very dependent on a WLC somewhere in the 
                network with the exception of FlexConnect architecture."
            }
            li {
                "The AP and WLC can be located on the same VLAN or IP subnet, but they do not have to be. Each AP must boot and 
                bind itself to a WLC to support wireless clients."
            }
            li {
                "Every AP and WLC must also authenticate each other with digital certificates. An X.509 certificate is preinstalled 
                in each device when it is purchased."
            }
            li {
                "The CAPWAP tunneling allows the AP and WLC to be separated geographically and logically. It also breaks the 
                dependence on Layer 2 connectivity between them."
            }
        }
    }
}