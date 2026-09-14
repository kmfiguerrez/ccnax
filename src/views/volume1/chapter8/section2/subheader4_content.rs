use dioxus::prelude::*;

use crate::{
    components::{
        ConfigChecklist,
        GreenNote,
        KeyTopic,
        my_accordion::{Accordion, AccordionContent, AccordionItem, AccordionTrigger}
    }, 
    utils::{TextCommandColor, h3_heading, h4_heading, text_command}
};

#[component]
pub fn Content() -> Element {
    rsx! {
        p { class: "mb-4",
            "This next topic is strange, at least in the context of access links and trunk links."
            br {}
            strong {
                "In the world of IP telephony, telephones use Ethernet ports to connect to an Ethernet network so they
            can use IP to send and receive voice traffic sent via IP packets."
            }
            br {}
            strong {
                "To make that work, the switch's Ethernet port acts like an access port, but at the same time, the port acts 
            like a trunk in some ways."
            }
            br {}
            "This last topic of the chapter works through those main concepts."
        }

        Accordion { class: "mb-4",
            AccordionItem {
                AccordionTrigger { {h3_heading("Data and Voice VLAN Concepts")} }
                AccordionContent {
                    p { class: "mb-4",
                        "Before IP telephony, a PC could sit on the same desk as a phone."
                        br {}
                        "The phone happened to use UTP cabling, with that phone connected to some voice device 
                        (often called a voice switch or a private branch exchange [PBX])."
                        br {}
                        "The PC, of course, connected using an unshielded twisted-pair (UTP) cable to the usual LAN switch that 
                        sat in the wiring closet—sometimes in the same wiring closet as the voice switch."
                        br {}
                        "Figure 8-11 shows the idea."
                    }

                    img {
                        class: "mb-4 rounded-lg",
                        alt: "Figure 8-11 Before IP Telephony: PC and Phone, One Cable Each, Connect to Two Different Devices",
                        loading: "lazy",
                        src: asset!("/assets/static/v1p3c8s2sh4f8-11.png", AssetOptions::image().with_avif()),
                    }

                    p { class: "mb-4",
                        "The term "
                        i { "IP telephony" }
                        " refers to the branch of networking in which the telephones use IP
                        packets to send and receive voice as represented by the bits in the data portion of the IP
                        packet."
                        br {}
                        strong {
                            "The phones connect to the network like most other end-user devices, using either Ethernet or Wi-Fi."
                        }
                        br {}
                        strong {
                            "These new IP phones did not connect via cable directly to a voice switch,
                            instead connecting to the IP network using an Ethernet cable and an Ethernet port built
                            in to the phone."
                        }
                        br {}
                        "The phones then communicated over the IP network with software that
                        replaced the call setup and other functions of the PBX."
                        br {}
                        strong {
                            "The current products from Cisco that perform this IP telephony control function are called Cisco 
                            Unified Communication Manager."
                        }
                    }

                    {h4_heading("Problems with migrating to new IP phones")}
                    p {
                        "The migration from using the already-installed telephone cabling to these new IP phones
                        that needed UTP cables that supported Ethernet caused some problems in some offices."
                        "In particular:"
                    }
                    ul { class: "list-disc list-inside mb-4",
                        li {
                            "The older non-IP phones used a category of UTP cabling that often did not support 100-
                            Mbps or 1000-Mbps Ethernet."
                        }
                        li {
                            "Most offices had a single UTP cable running from the wiring closet to each desk, but now
                            two devices (the PC and the new IP phone) both needed a cable from the desktop to the
                            wiring closet."
                        }
                        li {
                            "Installing a new cable to every desk would be expensive, plus you would need more switch ports."
                        }
                    }

                    p { class: "mb-4",
                        strong {
                            "To solve this problem, Cisco embedded small three-port switches into each phone"
                        }
                    }

                    {h4_heading("IP phones with embedded switch")}
                    p { class: "mb-4",
                        "IP telephones have included a small LAN switch, on the underside of the phone, since the
                        earliest IP telephone products."
                        br {}
                        "Figure 8-12 shows the basic cabling, with the wiring closet cable connecting to one physical port on 
                        the embedded switch, the PC connecting with a short patch cable to the other physical port, and the 
                        phone's internal CPU connecting to an internal switch port."
                    }

                    img {
                        class: "mb-4 rounded-lg",
                        alt: "Figure 8-12 Cabling with an IP Phone, a Single Cable, and an Integrated Switch",
                        loading: "lazy",
                        src: asset!("/assets/static/v1p3c8s2sh4f8-12.png", AssetOptions::image().with_avif()),
                    }

                    {h4_heading("Cisco IP phones best practices")}
                    p {
                        "Sites that use IP telephony, which includes almost every company today, now have two
                        devices off each access port."
                        br {}
                        "In addition, Cisco best practices for IP telephony design tell us to put the phones in one VLAN and 
                        the PCs in a different VLAN."
                        br {}
                        "To make that happen, the switch port acts a little like an access link (for the PC's traffic), and a 
                        little like a trunk (for the phone's traffic)."
                        br {}
                        "The configuration defines two VLANs on that port, as follows:"
                    }
                    KeyTopic {}
                    ul { class: "list-disc list-inside mb-4",
                        li {
                            span { class: "font-bold", "Data VLAN:" }
                            " Same idea and configuration as the access VLAN on an access port but
                            defined as the VLAN on that link for forwarding the traffic for the device connected to
                            the phone on the desk (typically the user's PC)."
                        }
                        li {
                            span { class: "font-bold", "Voice VLAN:" }
                            " The VLAN defined on the link for forwarding the phone's traffic. Traffic in
                            this VLAN is typically tagged with an 802.1Q header."
                        }
                    }

                    p { class: "mb-4",
                        "Figure 8-13 illustrates this design with two VLANs on access ports that support IP telephones."
                    }

                    img {
                        class: "mb-4 rounded-lg",
                        alt: "Figure 8-13 A LAN Design, with Data in VLAN 10 and Phones in VLAN 11",
                        loading: "lazy",
                        src: asset!("/assets/static/v1p3c8s2sh4f8-13.png", AssetOptions::image().with_avif()),
                    }
                
                }
            }
            AccordionItem {
                AccordionTrigger { {h3_heading("Data and Voice VLAN Configuration and Verification")} }
                AccordionContent {
                    p { class: "mb-4",
                        "Configuring a switch port to support IP phones, once you know the planned voice and data
                        VLAN IDs, requires just a few easy commands."
                        br {}
                        "Making sense of the "
                        {text_command("show", TextCommandColor::Gold)}
                        " commands once it is configured, however, can be a challenge."
                        br {}
                        "The port acts like an access port in many ways."
                        br {}
                        "However, with most configuration options, the voice frames flow with an 802.1Q header, so
                        that the link supports frames in both VLANs on the link."
                        br {}
                        "But that makes for some different "
                        {text_command("show", TextCommandColor::Gold)}
                        " command output."
                    }

                    p { class: "mb-4",
                        "Example 8-8 shows an example configuration."
                        br {}
                        "In this case, all four switch ports F0/1-F0/4 begin with default configuration."
                        br {}
                        "The configuration adds the new data and voice VLANs."
                        br {}
                        "The example then configures all four ports as access ports and defines the access VLAN,
                        which is also called the data VLAN when discussing IP telephony."
                        br {}
                        "Finally, the configuration includes the "
                        {text_command("switchport voice vlan 11", TextCommandColor::Gold)}
                        " command, which defines the voice VLAN used on the port."
                        br {}
                        "The example matches Figure 8-13, using ports F0/1-F0/4."
                    }

                    img {
                        class: "mb-4 rounded-lg",
                        alt: "Figure 8-11 Before IP Telephony: PC and Phone, One Cable Each, Connect to Two Different Devices",
                        loading: "lazy",
                        src: asset!("/assets/static/v1p3c8s2sh4ex8-8.png", AssetOptions::image().with_avif()),
                    }

                    GreenNote {
                        p {
                            strong { "NOTE" }
                            " CDP, which is discussed in the CCNA 200-301 Official Cert Guide, Volume 2,
                            Chapter 9, “Device Management Protocols,” must be enabled on an interface for a voice
                            access port to work with Cisco IP phones. Cisco switches and routers enable CDP by
                            default, so its configuration is not shown here."
                        }
                    }

                    ConfigChecklist {}
                    p {
                        "The following list details the configuration steps for easier review and study:"
                    }
                    ol { class: "mb-4",
                        // Step 1
                        li {
                            span { class: "text-sky-500 font-semibold mr-4", "Step 1." }
                            "Use the "
                            {text_command("vlan", TextCommandColor::Gold)}
                            i { " vlan-id" }
                            " command in global configuration mode to create the data
                            and voice VLANs if they do not already exist on the switch."
                        
                        }
                        // Step 2
                        li { class: "flex flex-col md:flex-row md:gap-x-4",
                            span { class: "text-sky-500 font-semibold shrink-0", "Step 2." }
                            div {
                                span { "Configure the data VLAN like an access VLAN, as usual:" }
                                ol {
                                    // Step A
                                    li {
                                        span { class: "text-sky-500 font-semibold uppercase mr-2",
                                            "a."
                                        }
                                        " Use the "
                                        {text_command("interface", TextCommandColor::Gold)}
                                        i { " type number" }
                                        " command global configuration mode to
                                        move into interface configuration mode."
                                    }
                                    // Step B
                                    li {
                                        span { class: "text-sky-500 font-semibold uppercase mr-2",
                                            "b."
                                        }
                                        "Use the "
                                        {text_command("switchport access vlan", TextCommandColor::Gold)}
                                        i { " id-number" }
                                        " command in interface configuration mode to define the data VLAN."
                                    }
                                    // Step C
                                    li {
                                        span { class: "text-sky-500 font-semibold uppercase mr-2",
                                            "b."
                                        }
                                        " Use the "
                                        {text_command("switchport mode access", TextCommandColor::Gold)}
                                        " command in interface configuration mode to make this port always operate in 
                                        access mode (that is, to not trunk)."
                                    }
                                }
                            }
                        
                        }
                        // Step 3
                        li {
                            span { class: "text-sky-500 font-semibold mr-4", "Step 3." }
                            "Use the "
                            {text_command("switchport voice vlan", TextCommandColor::Gold)}
                            i { " id-number" }
                            " command in interface configuration mode to set the voice VLAN ID."
                        }
                    }

                    p { "See Example 8-9/8-10 in volume 1 on page 199." }
                
                }
            }
            AccordionItem {
                AccordionTrigger { {h3_heading("Summary: IP Telephony Ports on Switches")} }
                AccordionContent {
                    KeyTopic {}
                    p {
                        "It might seem as though this short topic about IP telephony and switch configuration
                        includes a lot of small twists and turns and trivia, and it does."
                        br {}
                        "The most important items to remember are as follows:"
                    }
                    ul { class: "list-disc list-inside",
                        li {
                            "Configure these ports like a normal access port to begin: Configure it as a static access
                            port and assign it an access VLAN."
                        }
                        li {
                            "Add one more command to define the voice VLAN ("
                            {text_command("switchport voice vlan", TextCommandColor::Gold)}
                            i { " vlan-id" }
                            ")."
                        }
                        li {
                            "Look for the mention of the voice VLAN ID, but no other new facts, in the output of the "
                            {text_command("show interfaces", TextCommandColor::Gold)}
                            i { " type number " }
                            {text_command("switchport", TextCommandColor::Gold)}
                            " command."
                        }
                        li {
                            "Look for both the voice and data (access) VLAN IDs in the output of the "
                            {text_command("show interfaces", TextCommandColor::Gold)}
                            i { " type number " }
                            {text_command("trunk", TextCommandColor::Gold)}
                            " command."
                        }
                        li {
                            "Do not expect to see the port listed in the list of operational trunks as listed by the "
                            {text_command("show interfaces trunk", TextCommandColor::Gold)}
                            " command."
                        }
                    }
                }
            }
        }

        {h3_heading("RECAP")}
        ol { class: "list-disc list-inside",
            li {
                "In the world of IP telephony, telephones use Ethernet ports to connect to an Ethernet network 
                so they can use IP to send and receive voice traffic sent via IP packets. 
                To make that work, the switch's Ethernet port acts like an access port, but at the same time, the port acts 
                like a trunk in some ways."
            }
            li {
                "The term "
                i { "IP telephony" }
                " refers to the branch of networking in which the telephones use IP
                packets to send and receive voice as represented by the bits in the data portion of the IP
                packet."
            }
            li {
                "The phones connect to the network like most other end-user devices, using either Ethernet or Wi-Fi."
            }
            li {
                "These new IP phones did not connect via cable directly to a voice switch, instead connecting to the IP network using 
                an Ethernet cable and an Ethernet port built in to the phone."
            }
            li {
                "Cisco IP phones have embedded small three-port switches into each phone to prevent migration problems."
            }
            li {
                "Cisco best practices for IP telephony design tell us to put the phones in one VLAN and the PCs in a different VLAN."
                " To make that happen, the switch port acts a little like an access link (for the PC's traffic), and 
                a little like a trunk (for the phone's traffic)."
            }
            li { "In IP telephony, access VLAN is also called data VLAN." }
            li {
                "CDP must be enabled on an interface for a voice access port to work with Cisco IP phones."
            }
        }
    }
}