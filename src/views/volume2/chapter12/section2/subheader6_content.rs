use dioxus::prelude::*;

use crate::{
    components::my_accordion::{Accordion, AccordionContent, AccordionItem, AccordionTrigger}, 
    utils::{TextCommandColor, h3_heading, h4_heading, text_command}
};

#[component]
pub fn Content() -> Element {
    rsx! {
        p { class: "mb-4",
            "SNMPv3 configuration on Cisco routers and switches has some commands in common with SNMPv2c configuration, and some 
            completely different commands."
            br {}
            "The configuration to support sending Traps and Informs, using the "
            {text_command("snmp-server host", TextCommandColor::Gold)}
            " and "
            {text_command("snmp-server enable traps", TextCommandColor::Gold)}
            " commands, works almost identically, with a few small differences."
            br {}
            "However, SNMPv3 replaces all references to communities, and as a result does not use the "
            {text_command("snmp-server community", TextCommandColor::Gold)}
            " command at all."
            br {}
            "Instead, it uses the "
            {text_command("snmp-server group", TextCommandColor::Gold)}
            " and "
            {text_command("snmp-server user", TextCommandColor::Gold)}
            " commands to configure the security features available to  SNMPv3."
        }

        p { class: "mb-1",
            "SNMPv3 has many more configuration options, and it is easy to get confused by the details."
            br {}
            "So, to get started, first look at a short SNMPv3 configuration example, as shown in Example D-24."
            br {}
            "The example highlights the values you would have to choose, but the values are either
                        text fields (names and passwords) or the IP address of the NMS."
            br {}
            "This configuration could be used to replace the SNMPv2c configuration and use username/password 
                        authentication."
            br {}
            "The requirements met in the example are"
        }
        ol { class: "list-disc list-inside mb-4",
            li { "Use SNMPv3 authentication (basically replacing SNMPv2 communities)." }
            li {
                "Use username Youdda and authentication password madeuppassword (in your network,
                            you would choose your own values)."
            }
            li { "Do not use SNMPv3 privacy (that is, message encryption)." }
            li { "Allow both read (Get) and write (Set) access." }
            li { "Send Traps to an NMS (10.1.3.3), authenticating with the same username.." }
        }

        img {
            class: "mb-4 rounded-lg",
            loading: "lazy",
            alt: "Example D-24 Configuring SNMPv3 on R1—Authentication Only",
            src: asset!("/assets/static/v2p3c12s2sh6exD-24.png", AssetOptions::image().with_avif()),
        }

        p { class: "mb-4",
            "Given the list of requirements, you could probably just read the configuration in Example D-24, compare that to the 
            list of requirements preceding the example, and correctly guess what most of the command parameters mean."
            br {}
            "However, we need to get into more detail to work through these commands and their options so that you understand the 
            entire configuration, which is exactly what the next few pages do."
        }

        Accordion { class: "mb-4",
            AccordionItem {
                AccordionTrigger { {h3_heading("SNMPv3 Groups")} }
                AccordionContent {
                    p { class: "mb-4",
                        strong { "SNMPv3 authentication uses a username/password combination." }
                        br {}
                        "When Cisco created its SNMPv3 implementation in IOS, it realized that it might be useful to have groups 
                        of users that use some of the same security settings."
                        br {}
                        "So, rather than have each "
                        {text_command("snmp-server user", TextCommandColor::Gold)}
                        " command (the command that defines a user) define every single  security parameter, Cisco put some of the 
                        security configuration settings into the "
                        {text_command("snmp-server group", TextCommandColor::Gold)}
                        " command."
                        br {}
                        "This command holds SNMPv3 security settings that are often the same between a group of
                        SNMPv3 users; each "
                        {text_command("snmp-server user", TextCommandColor::Gold)}
                        " command then refers to one SNMP group."
                        br {}
                        "This next topic explores those security parameters defined on the "
                        {text_command("snmp-server group", TextCommandColor::Gold)}
                        " command."
                    }

                    p { class: "mb-4",
                        "Figure D-13 shows the entire "
                        {text_command("snmp-server group", TextCommandColor::Gold)}
                        " command."
                        br {}
                        "The required parameters on the left include a name that the network engineer can make up; it only needs to 
                        match other commands on the local router."
                        br {}
                        "For SNMPv3 configuration, the "
                        {text_command("v3", TextCommandColor::Gold)}
                        " keyword would always be used."
                        br {}
                        "The text following this figure then details the rest of the parameters in the figure."
                    }

                    img {
                        class: "mb-4 rounded-lg",
                        loading: "lazy",
                        alt: "Figure D-13 SNMPv3 Groups—Configuration Command Parameters",
                        src: asset!("/assets/static/v2p3c12s2sh6fD-13.png", AssetOptions::image().with_avif()),
                    }

                    {h4_heading("SNMPv3 security levels")}
                    p { class: "mb-4",
                        "The next parameter in the command configures this group of users to use one of three SNMPv3 "
                        i { "security levels." }
                        br {}
                        "As you can see from the summary in Table D-5, "
                        strong {
                            "all three security levels provide message integrity for 
                            their messages, which confirms that the message has not been changed in transit"
                        }
                        "."
                        br {}
                        "The "
                        {text_command("auth", TextCommandColor::Gold)}
                        " option adds authentication to message integrity, using a username and password, with IOS storing 
                        the password with a hash and never sending the password as clear text."
                        br {}
                        "The last increase in security level, configured by using the "
                        {text_command("priv", TextCommandColor::Gold)}
                        " security level, causes the SNMP manager and agent to encrypt the entire SNMP packet for all SNMP messages 
                        sent, in addition to performing message  integrity and authentication."
                    }

                    img {
                        class: "mb-4 rounded-lg",
                        loading: "lazy",
                        alt: "Table D-5 SNMPv3 Security Levels Keywords and Their Meanings",
                        src: asset!("/assets/static/v2p3c12s2sh6tD-5.png", AssetOptions::image().with_avif()),
                    }

                    {h4_heading("Optional ACLs")}
                    p { class: "mb-4",
                        "Continuing to look at the "
                        {text_command("snmp-server group", TextCommandColor::Gold)}
                        " command in Figure D-13, notice that it ends with an optional ACL to filter packets."
                        br {}
                        "This same idea is used in SNMPv2c to reference an IPv4 or IPv6 ACL to filter incoming messages coming 
                        from the SNMP manager."
                    }

                    {h4_heading("MIB views")}
                    p { class: "mb-4",
                        "So far, the discussion has ignored one part of the "
                        {text_command("snmp-server group", TextCommandColor::Gold)}
                        " command: the idea of SNMPv3 MIB views."
                        br {}
                        "MIB views define a subset of the MIB."
                        br {}
                        strong {
                            "IOS supplies a series of MIB views for us, and you can define your own MIB views if you like."
                        }
                        br {}
                        "However, this book discusses only one predefined MIB view that goes by the name "
                        i { "v1default" }
                        ", which is a MIB  view that includes all the useful parts of the MIB."
                        br {}
                        "Instead of focusing on the depths of how you might create different views of a router or switch MIB that 
                        has literally thousands of variables, focus on how the "
                        {text_command("snmp-server group", TextCommandColor::Gold)}
                        " command uses that one MIB view that includes the majority of the MIB."
                    }

                    p { class: "mb-4",
                        "By default, each SNMPv3 group, as defined with the "
                        {text_command("snmp-server group", TextCommandColor::Gold)}
                        " command, has a read MIB view of v1default, and no write view."
                        br {}
                        "As a result, the SNMP agent will process received SNMPv3 Get requests, but not process received SNMPv3 Set 
                        requests."
                        br {}
                        "That complete lack of a write MIB view basically results in read-only behavior for the SNMP agent,
                        as shown at the top of Figure D-14."
                    }

                    img {
                        class: "mb-4 rounded-lg",
                        loading: "lazy",
                        alt: "Figure D-14 SNMPv3 Views Creating Read-Only and Read-Write Effect",
                        src: asset!("/assets/static/v2p3c12s2sh6fD-14.png", AssetOptions::image().with_avif()),
                    }

                    p { class: "mb-4",
                        "The bottom of the figure shows the concept behind configuring an SNMP group with the "
                        {text_command("write v1default", TextCommandColor::Gold)}
                        " parameters, causing the group to use the same write view of the MIB that is used for reading the MIB."
                        br {}
                        "By including "
                        {text_command("write v1default", TextCommandColor::Gold)}
                        " in the "
                        {text_command("snmp-server group", TextCommandColor::Gold)}
                        " command, you migrate from a default operation of allowing only Gets to now also allowing Sets."
                    }

                    p { class: "mb-4",
                        "To pull these ideas together, Example D-25 shows four similar SNMPv3 groups, which
                        could later be referenced by "
                        {text_command("snmp-server user", TextCommandColor::Gold)}
                        " commands."
                        br {}
                        "Two commands use the parameters "
                        {text_command("write v1default", TextCommandColor::Gold)}
                        ", and two do not, so two groups create read-write (Get and Set) support,
                        and two groups create read-only (Get only) support."
                        br {}
                        "Also, note that two groups refer to an IPv4 ACL by name (SNMPACL), and two do not."
                        br {}
                        "The ends of the lines in the example list comments about each command."
                    }

                    img {
                        class: "mb-4 rounded-lg",
                        loading: "lazy",
                        alt: "Example D-25 SNMPv3 Groups—Comparisons with Write Views and ACL Security",
                        src: asset!("/assets/static/v2p3c12s2sh6exD-25.png", AssetOptions::image().with_avif()),
                    }

                    p { class: "mb-4",
                        "Note that while all four examples use an authentication type of "
                        {text_command("noauth", TextCommandColor::Gold)}
                        ", groups could be defined that use the "
                        {text_command("auth", TextCommandColor::Gold)}
                        " and "
                        {text_command("priv", TextCommandColor::Gold)}
                        " types as well."
                        br {}
                        "Configuring groups with any one of the security levels does not change the meaning and use of the "
                        {text_command("write", TextCommandColor::Gold)}
                        " and "
                        {text_command("access", TextCommandColor::Gold)}
                        " keywords and their parameters."
                        br {}
                        "The security level simply needs to match the security level configured on the "
                        {text_command("snmp-server user", TextCommandColor::Gold)}
                        " commands that refer to the group by name, as seen in the next section."
                    }
                
                }
            }
            AccordionItem {
                AccordionTrigger { {h3_heading("SNMPv3 Users, Passwords, and Encryption Keys")} }
                AccordionContent {
                }
            }
            AccordionItem {
                AccordionTrigger { {h3_heading("Verifying SNMPv3")} }
                AccordionContent {
                }
            }
            AccordionItem {
                AccordionTrigger { {h3_heading("Implementing SNMPv3 Notifications (Traps and Informs)")} }
                AccordionContent {
                }
            }
            AccordionItem {
                AccordionTrigger { {h3_heading("Summarizing SNMPv3 Configuration")} }
                AccordionContent {
                }
            }
        }

        {h3_heading("RECAP")}
        ol { class: "list-disc list-inside",
            li { "SNMPv3 authentication uses a username/password combination." }
            li { "MIB views define a subset of the MIB." }
            li {
                "IOS supplies a series of MIB views for us, and you can define your own MIB views if you like."
            }
            li {
                "By default, each SNMPv3 group, as defined with the "
                {text_command("snmp-server group", TextCommandColor::Gold)}
                " command, has a read MIB view of v1default, and no write view."
            }
        
        }
    }
}