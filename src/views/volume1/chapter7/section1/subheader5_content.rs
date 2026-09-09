use dioxus::prelude::*;

use crate::{
    utils::{TextCommandColor, h3_heading, text_command, h4_heading},
    components::{GreenNote, my_accordion::{Accordion, AccordionItem, AccordionTrigger, AccordionContent}}
};

#[component]
pub fn Content() -> Element {
    rsx! {
        p { class: "mb-4",
            "For any 10/100 or 10/100/1000 interfaces—that is, interfaces that can run at different
            speeds—Cisco Catalyst switches default to a setting of "
            {text_command("duplex auto", TextCommandColor::Gold)}
            " and "
            {text_command("speed auto", TextCommandColor::Gold)}
            "."
            br {}
            "As a result, those interfaces attempt to automatically determine the speed and duplex setting to use."
            br {}
            " Alternatively, you can configure most devices, switch interfaces included, to use a specific speed and/or duplex."
        }

        p { class: "mb-4",
            "In practice, using autonegotiation is easy: just leave the speed and duplex at the default setting, and let the 
            switch port negotiate what settings to use on each port."
            br {}
            "However, problems can occur due to unfortunate combinations of configuration."
            br {}
            "Therefore, this next topic walks through more detail about the concepts behind autonegotiation, so you know better 
            how to interpret the meaning of the switch "
            {text_command("show", TextCommandColor::Gold)}
            " commands and when to choose to use a particular
            configuration setting."
        }

        Accordion { class: "mb-4",
            AccordionItem {
                AccordionTrigger { {h3_heading("Autonegotiation Under Working Conditions")} }
                AccordionContent {
                    p { class: "mb-4",
                        "Ethernet devices on the ends of a link must use the same standard; otherwise, they cannot correctly send data."
                        br {}
                        strong {
                            "For example, a NIC cannot use 100BASE-T, which uses a two-pair UTP cable with a 100-Mbps speed, while the switch 
                            port on the other end of the link uses 1000BASE-T."
                        }
                        br {}
                        "Even if you used a cable that works with Gigabit Ethernet, the link would not
                        work with one end trying to send at 100 Mbps while the other tried to receive the data at
                        1000 Mbps."
                    }

                    {h4_heading("Problems with upgrading")}
                    p { class: "mb-4",
                        "Upgrading to new and faster Ethernet standards becomes a problem because both ends have
                        to use the same standard."
                        br {}
                        "For example, if you replace an old PC with a new one, the old one might have been using 100BASE-T while the new 
                        one uses 1000BASE-T."
                        br {}
                        "The switch port on the other end of the link needs to now use 1000BASE-T, so you upgrade the switch."
                        br {}
                        "If that switch had ports that would use only 1000BASE-T, you would need to upgrade all the other
                        PCs connected to the switch."
                        br {}
                        "So, having both PC network interface cards (NIC) and switch ports that support multiple standards/speeds makes 
                        it much easier to migrate to the next better standard."
                    }

                    {h4_heading("IEEE standard 802.3u")}
                    p { class: "mb-4",
                        "The IEEE autonegotiation protocol helps makes it much easier to operate a LAN when NICs
                        and switch ports support multiple speeds."
                        br {}
                        "IEEE autonegotiation (IEEE standard 802.3u) defines a protocol that lets the two UTP-based Ethernet nodes 
                        on a link negotiate so that they each choose to use the same speed and duplex settings."
                        br {}
                        "The protocol messages flow outside the normal Ethernet electrical frequencies as out-of-band signals 
                        over the UTP cable."
                        br {}
                        strong {
                            "Basically, each node states what it can do, and then each node picks the best options that
                        both nodes support: the fastest speed and the best duplex setting, with full duplex being
                        better than half duplex."
                        }
                    }

                    GreenNote {
                        p {
                            strong { "NOTE" }
                            " Autonegotiation relies on the fact that the IEEE uses the same wiring pinouts for
                            10BASE-T and 100BASE-T, and that 1000BASE-T simply adds to those pinouts, adding two
                            pairs."
                        }
                    }

                    {h4_heading("1000BASE-T uses four-pair cabling")}
                    p { class: "mb-4",
                        "Many networks use autonegotiation every day, particularly between user devices and the
                        access layer LAN switches, as shown in Figure 7-1."
                        br {}
                        "The company installed four-pair cabling of the right quality to support 1000BASE-T, to be ready to 
                        support Gigabit Ethernet."
                        br {}
                        "As a result, the wiring supports 10-Mbps, 100-Mbps, and 1000-Mbps Ethernet options."
                        br {}
                        "Both nodes on each link send autonegotiation messages to each other."
                        br {}
                        "The switch in this case has all 10/100/1000 ports, while the PC NICs support different options."
                    }

                    img {
                        class: "mb-4 rounded-lg",
                        alt: "Figure 7-1 IEEE Autonegotiation Results with Both Nodes Working Correctly",
                        loading: "lazy",
                        src: asset!("/assets/static/v1p2c7s1sh5f7-1.png", AssetOptions::image().with_avif()),
                    }

                    p { "The following list breaks down the logic, one PC at a time:" }
                    ul { class: "list-disc list-inside sm:pl-3",
                        li {
                            strong { "PC1:" }
                            " The switch port claims it can go as fast as 1000 Mbps, but PC1's NIC claims a top
                            speed of 10 Mbps. Both the PC and the switch choose the fastest speed that each supports 
                            (10 Mbps) and the best duplex that each supports (full). "
                        }
                        li {
                            strong { "PC2:" }
                            " PC2 claims a best speed of 100 Mbps, which means it can use 10BASE-T or
                            100BASE-T. The switch port and NIC negotiate to use the best speed of 100 Mbps and
                            full duplex."
                        }
                        li {
                            strong { "PC3:" }
                            " It uses a 10/100/1000 NIC, supporting all three speeds and standards, so both the
                            NIC and switch port choose 1000 Mbps and full duplex."
                        }
                    }
                }
            }
            AccordionItem {
                AccordionTrigger { {h3_heading("Autonegotiation Results When Only One Node Uses Autonegotiation")} }
                AccordionContent {
                    p { class: "mb-4",
                        "Figure 7-1 shows the IEEE autonegotiation results when both nodes use the process."
                        br {}
                        "However, most Ethernet devices can disable autonegotiation, so it is just as important to
                        know what happens when a node tries to use autonegotiation but the node gets no response."
                    }

                    {h4_heading("Problems with disabling autonegotiation")}
                    p { class: "mb-4",
                        "Disabling autonegotiation is not always a bad idea."
                        br {}
                        "For instance, many network engineers disable autonegotiation on links between switches and simply 
                        configure the desired speed and duplex on both switches."
                        br {}
                        "However, mistakes can happen when one device on an Ethernet predefines speed and duplex 
                        (and disables autonegotiation), while the device on the other end attempts autonegotiation."
                        br {}
                        "In that case, the link might not work at all, or it might just work poorly."
                    }

                    GreenNote {
                        p {
                            strong { "NOTE" }
                            " Configuring both the speed and duplex on a Cisco Catalyst switch interface disables autonegotiation."
                        }
                    }

                    {h4_heading("IEEE autonegotiation default rules")}
                    p {
                        "IEEE autonegotiation defines some rules (defaults) that nodes should use as defaults when
                        autonegotiation fails—that is, when a node tries to use autonegotiation but hears nothing
                        from the device."
                        br {}
                        "The rules:"
                    }
                    ul { class: "list-disc list-inside mb-4",
                        li {
                            strong { "Speed:" }
                            " Use your slowest supported speed (often 10 Mbps)."
                        }
                        li {
                            strong { "Duplex:" }
                            "  If your speed = 10 or 100, use half duplex; otherwise, use full duplex."
                        }
                    }

                    {h4_heading("Cisco devices can sense speed")}
                    p {
                        "Cisco switches can make a better choice than that base IEEE speed default because Cisco
                        switches can actually sense the speed used by other nodes, even without IEEE autonegotiation."
                        br {}
                        "As a result, Cisco switches use this slightly different logic to choose the speed when
                        autonegotiation fails:"
                    }
                    ul { class: "list-disc list-inside mb-4",
                        li {
                            strong { "Speed:" }
                            "  Sense the speed (without using autonegotiation), but if that fails, use the IEEE
                            default (slowest supported speed, often 10 Mbps)."
                        }
                        li {
                            strong { "Duplex:" }
                            "  Use the IEEE defaults: If speed = 10 or 100, use half duplex; otherwise, use full duplex."
                        }
                    }

                    GreenNote {
                        p {
                            strong { "NOTE" }
                            " Ethernet interfaces using speeds faster than 1 Gbps always use full duplex."
                        }
                    }

                    p { class: "mb-4",
                        "Figure 7-2 shows three examples in which three users change their NIC settings and disable
                        autonegotiation, while the switch (with all 10/100/1000 ports) attempts autonegotiation."
                        br {}
                        "That is, the switch ports all default to speed auto and duplex auto."
                        br {}
                        "The top of the figure shows the configured settings on each PC NIC, with the choices made by 
                        the switch listed next to each switch port."
                    }

                    img {
                        class: "mb-4 rounded-lg",
                        alt: "Figure 7-2 IEEE Autonegotiation Results with Autonegotiation Disabled on One Side",
                        loading: "lazy",
                        src: asset!("/assets/static/v1p2c7s1sh5f7-2.png", AssetOptions::image().with_avif()),
                    }

                    p { "Reviewing each link, left to right:" }
                    ul { class: "list-disc list-inside mb-4",
                        li {
                            strong { "PC1:" }
                            "  The switch receives no autonegotiation messages, so it senses the electrical signal
                            to learn that PC1 is sending data at 100 Mbps. The switch uses the IEEE default duplex
                            based on the 100 Mbps speed (half duplex)."
                        }
                        li {
                            strong { "PC2:" }
                            "  The switch uses the same steps and logic as with the link to PC1, except that the
                            switch chooses to use full duplex because the speed is 1000 Mbps."
                        }
                        li {
                            strong { "PC3:" }
                            "  The user picks poorly, choosing the slower speed (10 Mbps) and the worse duplex
                            setting (half). However, the Cisco switch senses the speed without using IEEE autonegotiation and 
                            then uses the IEEE duplex default for 10-Mbps links (half duplex)."
                        }
                    }

                    {h4_heading("Half duplex uses CSMA/CD")}
                    p { class: "mb-4",
                        "PC1 shows a classic and unfortunately common end result: a duplex mismatch."
                        br {}
                        "The two nodes (PC1 and SW1's port G0/1) both use 100 Mbps, so they can send data."
                        br {}
                        "However, PC1, using full duplex, does not attempt to use carrier sense multiple access with collision 
                        detection (CSMA/CD) logic and sends frames at any time."
                        br {}
                        "Switch port F0/1, with half duplex, does use CSMA/CD."
                        br {}
                        "As a result, switch port F0/1 will believe  collisions occur on the link, even if none physically occur."
                        br {}
                        "The switch port will stop transmitting, back off, resend frames, and so on."
                        br {}
                        "As a result, the link is up, but it performs poorly."
                        br {}
                        "The upcoming section titled “Interface Speed and Duplex Issues” will revisit this problem with a focus 
                        on how to recognize the symptoms of a duplex mismatch."
                    }
                }
            }
            AccordionItem {
                AccordionTrigger { {h3_heading("Autonegotiation and LAN Hubs")} }
                AccordionContent {
                    p { class: "mb-4",
                        "LAN hubs also impact how autonegotiation works."
                        br {}
                        "Basically, hubs do not react to autonegotiation messages, and they do not forward the messages."
                        br {}
                        "As a result, devices connected to a hub must use the IEEE rules for choosing default settings, 
                        which often results in the devices using 10 Mbps and half duplex."
                    }

                    p { class: "mb-4",
                        "Figure 7-3 shows an example of a small Ethernet LAN that uses a 20-year-old 10BASE-T hub."
                        br {}
                        "In this LAN, all devices and switch ports are 10/100/1000 ports."
                        br {}
                        "The hub supports only 10BASE-T."
                    }

                    img {
                        class: "mb-4 rounded-lg",
                        alt: "Figure 7-3 IEEE Autonegotiation with a LAN Hub",
                        loading: "lazy",
                        src: asset!("/assets/static/v1p2c7s1sh5f7-3.png", AssetOptions::image().with_avif()),
                    }

                    p { class: "mb-4",
                        "Note that the devices on the right need to use half duplex because the hub requires the use
                        of the CSMA/CD algorithm to avoid collisions."
                    }

                    GreenNote {
                        p {
                            strong { "NOTE" }
                            " If you would like to learn more about collision domains and the impact of these
                            older LAN hubs, look to the companion website for Appendix K, “Analyzing Ethernet LAN
                            Designs,” to the section titled “Ethernet Collision Domains.”"
                        }
                    
                    }
                }
            }
        }

        {h3_heading("RECAP")}
        ol { class: "list-disc list-inside",
            li {
                "For any 10/100 or 10/100/1000 interfaces—that is, interfaces that can run at different
                speeds—Cisco Catalyst switches default to a setting of "
                {text_command("duplex auto", TextCommandColor::Gold)}
                " and "
                {text_command("speed auto", TextCommandColor::Gold)}
                "."
            }
            li {
                "Ethernet devices on the ends of a link must use the same standard; otherwise, they cannot correctly send data."
            }
            li {
                "IEEE autonegotiation (IEEE standard 802.3u) defines a protocol that lets the two UTP-based Ethernet nodes on a link 
                negotiate so that they each choose to use the same speed and duplex settings. Each node picks the best options that both 
                nodes support: the fastest speed and the best duplex setting (full duplex)"
            }
            li {
                "Configuring both the speed and duplex on a Cisco Catalyst switch interface disables autonegotiation."
            }
            li {
                "Cisco devices can sense the speed of the device on the other end will use IEEE autonegotiation duplex default
                based on the speed. When fails will use IEEE autonegotiation default rules."
            }
            li { "Ethernet interfaces using speeds faster than 1 Gbps always use full duplex." }
            li {
                "Half duplex uses CSMA/CD. When each node uses different duplex setting will perform poorly."
            }
            li {
                "Hubs do not react to autonegotiation messages. As a result, devices connected to a hub must use the IEEE rules 
                for choosing default settings, which often results in the devices using 10 Mbps and half duplex."
            }
        }
    }
}