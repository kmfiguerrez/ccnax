use dioxus::prelude::*;

use crate::{
    utils::h3_heading,
    components::{
        KeyTopic,
        my_accordion::{Accordion, AccordionItem, AccordionTrigger, AccordionContent}
    }
};

#[component]
pub fn SectionIntroductionContent() -> Element {
    rsx! {
        p { class: "mb-4",
            "Suppose you want to deploy a WLC to support multiple lightweight APs in your network."
            br {}
            "Where should you put the WLC?"
            br {}
            "The split-MAC concept can be applied to several different network architectures."
            br {}
            strong {
                "Each architecture places the WLC in a different location within the network—a choice that also affects how many WLCs 
                might be needed to support the number of APs required."
            }
        }

        Accordion { class: "mb-4",
            AccordionItem {
                AccordionTrigger { {h3_heading("Unified or centralized WLC deployment")} }
                AccordionContent {
                    p { class: "mb-4",
                        strong {
                            "One approach is to locate the WLC in a central location so that you can maximize the number of APs joined to it."
                        }
                        br {}
                        "This is usually called a "
                        i { "unified" }
                        " or "
                        i { "centralized WLC deployment" }
                        ", which tends to follow the concept that most of the resources users need to reach are located in a central location 
                        such as a data center or the Internet."
                        br {}
                        strong {
                            "Traffic to and from wireless users would travel over CAPWAP tunnels that reach into the center of the network, near 
                            the core, as shown in Figure 27-8."
                        }
                        br {}
                        "A centralized WLC also provides a convenient place to enforce security policies that affect all wireless users."
                    }

                    KeyTopic {}
                    img {
                        class: "mb-4 rounded-lg",
                        alt: "Figure 27-8 WLC Location in a Unified Deployment",
                        loading: "lazy",
                        src: asset!("/assets/static/v1p8c27s4f27-8.png", AssetOptions::image().with_avif()),
                    }

                    p { class: "mb-4",
                        "Figure 27-8 shows four APs joined to a single WLC."
                        br {}
                        "Your network might have more APs— many, many more."
                        br {}
                        "A large enterprise network might have thousands of APs connected to its access layer."
                        br {}
                        "Scalability then becomes an important factor in the centralized design."
                        br {}
                        "Typical unified WLCs can support a maximum of 6000 APs."
                        br {}
                        strong {
                            "If you have more APs than the maximum, you will need to add more WLCs to the design, each located centrally."
                        }
                    }
                }
            }
            AccordionItem {
                AccordionTrigger { {h3_heading("Cloud-based WLC deployment")} }
                AccordionContent {
                    p { class: "mb-4",
                        "A WLC can also be located in a central position in the network, inside a data center in a
                        private cloud, as shown in Figure 27-9."
                        br {}
                        "This is known as a "
                        i { "cloud-based WLC deployment" }
                        ", where the WLC exists as a virtual machine rather than a physical device."
                        br {}
                        "If the cloud computing platform already exists, then deploying a cloud-based WLC becomes straightforward."
                        br {}
                        "Such a controller can typically support up to 3000 APs."
                        br {}
                        "If your wireless network scales beyond that, then additional WLCs can be added as more virtual machines."
                    }

                    KeyTopic {}
                    img {
                        class: "mb-4 rounded-lg",
                        alt: "Figure 27-9 WLC Location in a Cloud-based Deployment",
                        loading: "lazy",
                        src: asset!("/assets/static/v1p8c27s4f27-9.png", AssetOptions::image().with_avif()),
                    }
                }
            }
            AccordionItem {
                AccordionTrigger { {h3_heading("Embedded WLC deployment")} }
                AccordionContent {
                    p { class: "mb-4",
                        "For small campuses or distributed branch locations, where the number of APs is relatively small in each, 
                        the WLC can be co-located with a stack of switches, as shown in Figure 27-10."
                        br {}
                        "This is known as an "
                        i { "embedded WLC deployment" }
                        " because the controller is embedded within the switching hardware."
                        br {}
                        "Typical Cisco embedded WLCs can support up to 200 APs."
                        br {}
                        strong {
                            "The APs do not necessarily have to be connected to the switches that host the WLC; APs connected to 
                            other switches in other locations can join the embedded WLC too."
                        }
                        br {}
                        "As the number of APs grows, additional WLCs can be added by embedding them in other switch stacks at the 
                        site."
                    }

                    KeyTopic {}
                    img {
                        class: "mb-4 rounded-lg",
                        alt: "Figure 27-10 WLC Location in an Embedded Deployment",
                        loading: "lazy",
                        src: asset!("/assets/static/v1p8c27s4f27-10.png", AssetOptions::image().with_avif()),
                    }
                }
            }
            AccordionItem {
                AccordionTrigger { {h3_heading("Mobility Express WLC deployment")} }
                AccordionContent {
                    p { class: "mb-4",
                        strong {
                            "Finally, in small-scale environments, such as small, midsize, or multisite branch locations, you
                            might not want to invest in dedicated WLCs at all."
                        }
                        br {}
                        "In this case, the WLC function can be co-located with an AP that is installed at the branch site."
                        br {}
                        "This is known as a Cisco "
                        i { "Mobility Express WLC deployment" }
                        ", as shown in Figure 27-11."
                        br {}
                        "The AP that hosts the WLC forms a CAPWAP tunnel with the WLC, along with any other APs at the same 
                        location."
                        br {}
                        "A Mobility Express WLC can support up to 100 APs. "
                    }

                    KeyTopic {}
                    img {
                        class: "mb-4 rounded-lg",
                        alt: "Figure 27-11 WLC Location in a Mobility Express Deployment",
                        loading: "lazy",
                        src: asset!("/assets/static/v1p8c27s4f27-11.png", AssetOptions::image().with_avif()),
                    }
                }
            }
        }

        p { class: "mb-4",
            "See Table 27-2 for a summary of WLC deployment models, WLC locations, and a typical maximum number of APs and clients 
            that each one supports."
        }

        img {
            class: "mb-4 rounded-lg",
            alt: "Table 27-2 Summary of WLC Deployment Models",
            loading: "lazy",
            src: asset!("/assets/static/v1p8c27s4t27-2.png", AssetOptions::image().with_avif()),
        }

        {h3_heading("RECAP")}
        ol { class: "list-disc list-inside",
            li { "The split-MAC concept can be applied to several different network architectures." }
            li {
                "In a unified or centralized WLC deployment, the WLC is located in a central location such as a data center 
                (near the core) or the Internet."
            }
            li {
                "In a cloud-based WLC deployment, the WLC can also be located in a central location such as a data center 
                (near the core) or the Internet, but the WLC exists as a virtual machine rather than a physical device."
            }
            li {
                "The embedded WLC deployment, is used for small campuses or distributed branch locations , where the WLC is 
                embedded within the switching hardware."
            }
            li {
                "The Mobility Express WLC deployment, is used for small-scale environments, such as small, midsize, or multisite branch locations,
                where the WLC function is co-located with an AP that is installed at the branch site."
            }
        }
    }
}