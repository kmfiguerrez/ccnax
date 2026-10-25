use dioxus::prelude::*;

use crate::{
    utils::h3_heading,
    components::{KeyTopic, GreenNote}
};

#[component]
pub fn Content() -> Element {
    rsx! {
        p { class: "mb-4",
            strong {
                "It might be obvious that wireless devices and APs should all be capable of operating on the same band."
            }
            br {}
            "For example, a 5-GHz wireless phone can communicate only with an AP that offers Wi-Fi service on 5-GHz channels."
            br {}
            strong {
                "In addition, the devices and APs must also share a compatibility with the parts of the 802.11 standard they support."
            }
        }

        {h3_heading("Ammendments to 802.11 Wi-Fi standard")}
        p { class: "mb-4",
            "As the IEEE 802.11 Wi-Fi standard evolves and develops, new amendments with new functionality get proposed."
            br {}
            strong {
                "These amendments are known by “802.11” followed by a one- or two-letter suffix until they are accepted and rolled up 
            into the next generation of the complete 802.11 standard."
            }
            br {}
            "Even then, it is common to see the amendment suffixes still used to distinguish specific functions."
        }

        p { class: "mb-4",
            strong {
                "You should be aware of several amendments that define important characteristics such as
                data rates, methods used to transmit and receive data, and so on."
            }
            br {}
            "For the CCNA 200-301 exam, you should know which band each of the amendments listed in Table 26-3 uses."
            br {}
            "The ENCOR 300-401 exam goes further into the data rates and modulation and coding schemes used by each."
        }

        KeyTopic {}
        img {
            class: "mb-4 rounded-lg",
            alt: "Table 26-3 Basic Characteristics of Some IEEE 802.11 Amendments",
            loading: "lazy",
            src: asset!("/assets/static/v1p8c26s4sh2f26-3.png", AssetOptions::image().with_avif()),
        }

        p { class: "mb-4",
            "The 802.11 amendments are not mutually exclusive."
            br {}
            strong {
                "Wireless client devices and APs can be compatible with one or more amendments; however, a client and an AP can 
                communicate only if they both support and agree to use the same amendment."
            }
            br {}
            "When you look at the specifications for a wireless device, you may find supported amendments listed in a single
            string, separated by slashes."
            br {}
            "For example, a device that supports 802.11b/g will support both 802.11b and 802.11g."
            br {}
            "One that supports b/g/a/n/ac will support 802.11b, 802.11g, 802.11n, and 802.11ac."
            br {}
            "You should become familiar with Table 26-3 so that you can know which bands a device can use based on its 802.11 
            amendment support."
        }

        {h3_heading("2.4-GHz vs 5-GHz")}
        p { class: "mb-4",
            "If a device can operate on both bands, how does it decide which band to use?"
            br {}
            strong {
                "APs can usually operate on both bands simultaneously to support any clients that might be present on each band."
            }
            br {}
            "However, "
            strong { "wireless clients typically associate with an AP on one band at a time" }
            ", while scanning for potential APs on both bands."
            br {}
            "The band used to connect to an AP is chosen according to the operating system, wireless adapter driver, and other 
            internal configuration."
            br {}
            "A wireless client can initiate an association with an AP on one band and then switch to the other band if the signal 
            
            conditions are better there."
        }

        GreenNote {
            p {
                strong { "NOTE" }
                " Cisco APs have dual radios (sets of transmitters and receivers) to support BSSs on one 2.4-GHz channel and other 
                BSSs on one 5-GHz channel simultaneously."
                br {}
                "Some models also have two 5-GHz radios that can be configured to operate BSSs on two different channels at the 
                same time, providing wireless coverage to higher densities of users that are located in the same vicinity."
                br {}
                "You can configure a Cisco AP to operate on a specific channel number. As the number of APs grows, manual channel 
                assignment can become a difficult task. Fortunately, Cisco wireless architectures can automatically and dynamically 
                assign each AP to an appropriate channel. The architecture is covered in Chapter 27, “Analyzing Cisco Wireless 
                Architectures,” while dynamic channel assignment is covered on the ENCOR 300-401 exam."
            }
        }

        p { class: "mb-4",
            strong {
                "In open space, RF signals propagate or reach further on the 2.4-GHz band than on the 5-GHz band."
            }
            br {}
            "They also tend to penetrate indoor walls and objects easier at 2.4 GHz than 5 GHz."
            br {}
            strong { "However, the 2.4-GHz band is commonly more crowded with wireless devices." }
            br {}
            "Remember that only three nonoverlapping channels are available, so the chances of other neighboring
            APs using the same channels is greater."
            br {}
            "In contrast, the 5-GHz band has many more channels available to use, making channels less crowded and experiencing 
            less interference."
        }

        {h3_heading("RECAP")}
        ol { class: "list-disc list-inside",
            li {
                "To communicate, wireless devices and APs should all be capable of operating on the same band and 
                if they both support and agree to use the same amendment."
            }
            li {
                "Ammendments to 802.11 Wi-Fi standard are known by 802.11 followed by a one- or two-letter suffix until they are 
                accepted and rolled up into the next generation of the complete 802.11 standard."
            }
            li {
                "APs can usually operate on both bands simultaneously to support any clients that might be present on each band."
            }
            li {
                "Wireless clients can only associate with an AP on one band at a time, while scanning for potential APs on both bands."
            }
            li {
                "In open space, RF signals propagate or reach further on the 2.4-GHz band than on the 5-GHz band. 
                They also tend to penetrate indoor walls and objects easier at 2.4 GHz than 5 GHz."
            }
            li {
                "Remember in 2.4-GHz band, there are only three nonoverlapping channels are available: 1, 6 and 11."
            }
        }
    }
}