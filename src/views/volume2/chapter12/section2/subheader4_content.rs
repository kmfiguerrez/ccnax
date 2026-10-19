use dioxus::prelude::*;

use crate::{utils::h3_heading, components::{KeyTopic, GreenNote}};

#[component]
pub fn Content() -> Element {
    rsx! {
        p { class: "mb-4",
            "SNMP supports a few security mechanisms, depending in part on the particular version."
            br {}
            "This section works through the options."
        }

        {h3_heading("ACLs to secure SNMP")}
        p { class: "mb-4",
            "First, "
            strong {
                "one strong method to secure SNMP is to use ACLs to limit SNMP messages to those
                from known servers only."
            }
            br {}
            "SNMP agents on Cisco routers and switches support SNMP messages that flow in both IPv4 and IPv6 packets."
            br {}
            "The SNMP agent can configure an IPv4 ACL to filter incoming SNMP messages that arrive in IPv4 packets and an IPv6 ACL 
            to filter SNMP messages that arrive in IPv6 packets."
        }

        p { class: "mb-4",
            "Using an IPv4 and IPv6 ACL to secure an agent makes good sense."
            br {}
            strong {
                "The only hosts that should be sending SNMP messages to the SNMP agent in a router or switch are the NMS hosts."
            }
            br {}
            "Those NMS hosts seldom move and their IP addresses should be well known to the networking staff."
            br {}
            strong {
                "It makes good sense to configure an ACL that permits packets sourced from the IP addresses of all NMS hosts, but no 
                others."
            }
        }

        p { class: "mb-4",
            "As for the SNMP protocol messages, all versions of SNMP support a basic clear-text password mechanism, although none 
            of those versions refer to the mechanism as using a password."
            br {}
            strong { "SNMP Version 3 (SNMPv3) adds more modern security as well." }
        }

        {h3_heading("SNMPv1 communities")}
        p { class: "mb-4",
            strong { "SNMPv1 defined clear-text passwords called SNMP communities." }
            br {}
            "Basically, both the SNMP agent and the SNMP manager need prior knowledge of the same SNMP community
            value (called a community string)."
            br {}
            "The SNMP Get messages and the Set message include the appropriate community string value, in clear text."
            br {}
            "If the NMS sends a Get or Set with the correct community string, as configured on the SNMP agent, the agent processes 
            the message."
        }

        p { class: "mb-4",
            strong { "SNMPv1 defines both a read-only community and a read-write community." }
            br {}
            "The read-only (RO) community allows Get messages, and the read-write (RW) community allows both
            reads and writes (Gets and Sets)."
            br {}
            "Figure 12-12 shows the concepts."
            br {}
            "At steps 1 and 2, the agent is configured with particular RO and RW community  strings, and the NMS configures the 
            matching values."
            br {}
            "At step 3, the SNMP Get can flow with either community, but at Step 4, the Set Request must use the RW community."
        }

        KeyTopic {}
        img {
            class: "mb-4 rounded-lg",
            loading: "lazy",
            alt: "Figure 12-12 RO and RW Communities with the Get and Set Commands",
            src: asset!("/assets/static/v2p3c12s2sh4f12-12.png", AssetOptions::image().with_avif()),
        }

        {h3_heading("SNMPv2 communitiy-based")}
        p { class: "mb-4",
            "SNMPv2, and the related Community-based SNMP Version 2 (SNMPv2c), added a wrinkle in naming but basically kept the 
            same community security feature as SNMPv1 once the standards process completed."
            br {}
            "The original specifications for SNMPv2 did not include SNMPv1 communities; however, the marketplace still wanted 
            communities, so an additional RFC added the SNMPv1 communities mechanism back to SNMPv2."
            br {}
            "This updated RFC, “Community-based SNMPv2,” came to be known simply as SNMPv2c."
            br {}
            "Vendors (including Cisco) implemented SNMPv2c; however, security was still relatively weak."
        }

        {h3_heading("SNMPv3")}
        p { class: "mb-1",
            "SNMPv3 arrived with much celebration among network administrators."
            br {}
            "Finally, security had arrived with the powerful network management protocol."
            br {}
            "SNMPv3 does away with communities and replaces them with the following features:"
        }
        div { class: "flex flex-col md:flex-row  md:items-center md:gap-x-3",
            KeyTopic {}
            ol { class: "list-disc list-inside mb-4",
                li {
                    span { class: "font-bold mr-1", "Message integrity:" }
                    "This mechanism, applied to all SNMPv3 messages, confirms whether or not each message has been changed during 
                    transit."
                }
                li {
                    span { class: "font-bold mr-1", "Authentication:" }
                    "This optional feature adds authentication with both a username and password, with the password never sent as 
                    clear text."
                    " Instead, it uses a hashing method like many other modern authentication processes."
                }
                li {
                    span { class: "font-bold mr-1", "Encryption (privacy):" }
                    "This optional feature encrypts the contents of SNMPv3 messages so that attackers who intercept the messages 
                    cannot read their contents."
                }
            }
        }

        GreenNote {
            p {
                strong { "NOTE" }
                " The CCNA 200-301 exam blueprint lists SNMP in one exam topic, with that exam topic reduced to “explain SNMP,” 
                with no requirement for configuration or verification skills. However, the previous version of the CCNA R&S 
                certification did include SNMP configuration. Refer to Appendix D if interested in learning about SNMP configuration 
                and verification."
            }
        }

        {h3_heading("RECAP")}
        ol { class: "list-disc list-inside",
            li {
                "One strong method to secure SNMP is to use ACLs to limit SNMP messages to those from known servers only."
            }
            li {
                "SNMP agents on Cisco routers and switches support SNMP messages that flow in both IPv4 and IPv6 packets."
            }
            li {
                "The SNMP agent can configure an IPv4 ACL to filter incoming SNMP messages that arrive in IPv4 packets and an IPv6 
                ACL to filter SNMP messages that arrive in IPv6 packets."
            }
            li {
                "The only hosts that should be sending SNMP messages to the SNMP agent in a router or switch are the NMS hosts, but 
                no others."
            }
            li {
                "SNMPv1 defined clear-text passwords called SNMP communities. SNMPv1 defines both a read-only community and a 
                read-write community."
            }
            li {
                "Basically, both the SNMP agent and the SNMP manager need prior knowledge of the same SNMP community value 
                (called a community string)."
            }
        }
    }
}