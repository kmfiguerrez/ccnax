use dioxus::prelude::*;

use crate::{
    components::KeyTopic,
    utils::h3_heading
};

#[component]
pub fn SectionIntroductionContent() -> Element {
    rsx! {
        p { class: "mb-4",
            strong { "Recall that an autonomous AP needs quite a bit of configuration and management." }
            br {}
            "To help manage more and more autonomous APs as the wireless network grows, you could place an AP management platform 
            such as Cisco Prime Infrastructure in a central location within the enterprise."
            br {}
            "The management platform would need to be purchased, configured, and maintained too."
        }

        {h3_heading("Cisco Meraki")}
        p { class: "mb-4",
            "A simpler approach is a "
            i { "cloud-based AP" }
            " architecture, where the AP management function is pushed out of the enterprise and into the Internet cloud."
            br {}
            "Cisco Meraki is cloud-based and offers centralized management of wireless, switched, and security networks built 
            from Meraki products."
            br {}
            "For example, through the cloud networking service, you can configure and manage APs, monitor wireless performance and 
            activity, generate reports, and so on."
        }

        p { class: "mb-4",
            strong {
                "Cisco Meraki APs can be deployed automatically, once you register with the Meraki cloud."
            }
            br {}
            "Each AP will contact the cloud when it powers up and will self-configure."
            br {}
            "From that point on, you can manage the AP through the Meraki cloud dashboard."
        }

        p { class: "mb-4",
            "Figure 27-3 illustrates the basic cloud-based architecture."
            br {}
            "Notice that the network is arranged identically to that of the autonomous AP network."
            br {}
            strong { "The reason is that the APs in a cloud-based network are all autonomous, too." }
            br {}
            "The most visible difference is that all of the APs are managed, controlled, and monitored centrally from the cloud."
        }

        KeyTopic {}
        img {
            class: "mb-4 rounded-lg",
            alt: "Figure 27-3 Cisco Meraki Cloud-Based Wireless Network Architecture",
            loading: "lazy",
            src: asset!("/assets/static/v1p8c27s2f27-3.png", AssetOptions::image().with_avif()),
        }

        {h3_heading("Cisco Meraki controlling the APs")}
        p { class: "mb-4",
            "From the cloud, you can push out code upgrades and configuration changes to the APs in the enterprise."
            br {}
            strong {

                "The Cisco Meraki cloud also adds the intelligence needed to automatically instruct each AP on which channel and 
                transmit power level to use."
            }

            br {}
            "It can also collect information from all of the APs about things such as RF interference, rogue or unexpected wireless 
            devices that were overheard, and wireless usage statistics."
        }

        {h3_heading("Cloud-based architecture data path")}
        p { class: "mb-4",
            "Finally, there are a couple of things you should observe about the cloud-based architecture."
            br {}
            "The data path from the wireless network to the wired network is very short; the autonomous
            AP links the two networks."
            br {}
            "Data to and from wireless clients does not have to travel up into
            the cloud and back; "
            strong { "the cloud is used to bring management functions into the data plane" }
            "."
        }

        {h3_heading("Control vs Data plane")}
        p { class: "mb-1",
            "Also, notice that the network in Figure 27-3 consists of two distinct paths—one for data
            traffic and another for management traffic, corresponding to the following two functions:"
        }
        ol { class: "list-disc list-inside mb-4",
            li {
                span { class: "font-bold mr-1", "A control plane:" }
                "Traffic used to control, configure, manage, and monitor the AP itself"
            }
            li {
                span { class: "font-bold mr-1", "A data plane:" }
                "End-user traffic passing through the AP"
            }
        }

        p { class: "mb-4",
            "This division will become important in the following sections as other types of architecture
            are discussed."
        }

        {h3_heading("RECAP")}
        ol { class: "list-disc list-inside",
            li {
                "To help manage more and more autonomous APs as the wireless network grows, you could place an AP management 
                platform such as Cisco Prime Infrastructure in a central location within the enterprise."
            }
            li {
                "An alternative simpler approach to Cisco Prime Infrastructure is "
                i { "cloud-based AP" }
                " architecture, where the AP management function is pushed out of the enterprise and into the Internet cloud."
            }
            li {
                "Cisco Meraki is cloud-based and offers centralized management of wireless, switched, and security networks built 
                from Meraki products."
            }
            li {
                "Cisco Meraki APs can be deployed automatically, once you register with the Meraki cloud. 
                Each AP will contact the cloud when it powers up and will self-configure."
            }
            li {
                "APs in a cloud-based network are all autonomous, too. The most visible difference is that all of the APs are 
                managed, controlled, and monitored centrally from the cloud."
            }
        }
    }
}