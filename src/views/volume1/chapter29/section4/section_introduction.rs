use dioxus::prelude::*;

use crate::{
    components::{KeyTopic, GreenNote},
    utils::h3_heading
};

#[component]
pub fn SectionIntroductionContent() -> Element {
    rsx! {
        p { class: "mb-4",
            "A wireless LAN controller and an access point work in concert to provide network connectivity to wireless clients."
            br {}
            "From a wireless perspective, the AP advertises a Service Set Identifier (SSID) for the client to join."
            br {}
            "From a wired perspective, the controller connects to a virtual LAN (VLAN) through one of its dynamic interfaces."
            br {}
            "To complete the path between the SSID and the VLAN, as illustrated in Figure 29-7, you must first define a WLAN on the
            controller."
        }

        GreenNote {
            p {
                strong { "NOTE" }
                " Two of the CCNA exam objectives involve configuring a WLAN for client connectivity with WPA2 and a PSK using only 
                the controller GUI. As you work through this section, you will find that it presents a complete WLAN example that is 
                based on the topology shown in Figure 29-7 using the WPA2-Personal (PSK) security model."
            }
        }

        KeyTopic {}
        img {
            class: "mb-4 rounded-lg",
            alt: "Figure 29-7 Connecting Wired and Wireless Networks with a WLAN",
            loading: "lazy",
            src: asset!("/assets/static/v1p8c29s4f29-7.png", AssetOptions::image().with_avif()),
        }

        p { class: "mb-4",
            strong {
                "The controller will bind the WLAN to one of its interfaces and then push the WLAN configuration out to all of its APs 
                by default."
            }
            br {}
            "From that point on, wireless clients will be able to learn about the new WLAN by receiving its beacons and will be 
            able to probe and join the new BSS."
        }

        {h3_heading("WLANs act like VLANs")}
        p { class: "mb-4",
            "Like VLANs, you can use WLANs to segregate wireless users and their traffic into logical networks."
            br {}
            "Users associated with one WLAN cannot cross over into another one unless their traffic is bridged or routed from one 
            VLAN to another through the wired network infrastructure."
        }

        {h3_heading("Plan the WLANs")}
        p { class: "mb-1",
            "Before you begin to create new WLANs, it is usually wise to plan your wireless network first."
            br {}
            "In a large enterprise, you might have to support a wide variety of wireless devices, user communities, security 
            policies, and so on."
            br {}
            "You might be tempted to create a new WLAN for every occasion, just to keep groups of users isolated from each other or 
            to support different types of devices."
            br {}
            "Although that is an appealing strategy, you should be aware of two limitations:"
        }
        ol { class: "list-disc list-inside mb-4",
            li {
                "Cisco controllers support a maximum of 512 WLANs, but only 16 of them can be actively configured on an AP."
            }
            li { "Advertising each WLAN to potential wireless clients uses up valuable airtime." }
        }

        {h3_heading("Beacons")}
        p { class: "mb-4",
            strong {
                "Every AP must broadcast beacon management frames at regular intervals to advertise the existence of a BSS."
            }
            br {}
            "Because each WLAN is bound to a BSS, each WLAN must be advertised with its own beacons."
            br {}
            "Beacons are normally sent 10 times per second, or once every 100 ms, at the lowest mandatory data rate."
            br {}
            "The more WLANs you have created, the more beacons you will need to announce them."
        }

        p { class: "mb-4",
            "Even further, the lower the mandatory data rate, the more time each beacon will take to be transmitted."
            br {}
            strong {
                "The end result is this: if you create too many WLANs, a channel can be starved of any usable airtime."
            }
            br {}
            "Clients will have a hard time transmitting their own data because the channel is overly busy with beacon 
            transmissions coming from the AP."
            br {}
            strong {
                "As a rule of thumb, always limit the number of WLANs to five or fewer; a maximum of three WLANs is best."
            }
        }

        p { class: "mb-1",
            "By default, a controller has a limited initial configuration, so no WLANs are defined."
            br {}
            "Before you create a new WLAN, think about the following parameters it will need to have:"
        }
        ol { class: "list-disc list-inside mb-4",
            li { " SSID string" }
            li { "Controller interface and VLAN number" }
            li { "Type of wireless security needed" }
        }

        p { class: "mb-1",
            "As you work through this section, you will create the appropriate dynamic controller interface to support the new 
            WLAN; then you will enter the necessary WLAN parameters."
            br {}
            "Each configuration step is performed using a web browser session that is connected to the WLC's
            management IP address."
        }

        {h3_heading("RECAP")}
        ol { class: "list-disc list-inside",
            li {
                "The controller will bind the WLAN to one of its interfaces and then push the WLAN configuration out to all of its 
                APs by default."
            }
            li {
                "You can think of WLANs as VLANs, by logically separating wireless users and their traffic into different networks."
            }
            li {
                "Every AP must broadcast beacon management frames at regular intervals to advertise the existence of a BSS."
            }
            li {
                "Because each WLAN is bound to a BSS, each WLAN must be advertised with its own beacons. if you create too many 
                WLANs, a channel can be starved of any usable airtime."
            }
            li {
                "As a rule of thumb, always limit the number of WLANs to five or fewer; a maximum of three WLANs is best."
            }
        }
    }
}