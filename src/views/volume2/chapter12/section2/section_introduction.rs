use dioxus::prelude::*;

use crate::utils::h3_heading;

#[component]
pub fn SectionIntroductionContent() -> Element {
    rsx! {
        p { class: "mb-4",
            "In 1988, RFC 1065, “Structure and Identification of Management Information for
            TCP/IP-based Internets,” was published."
            br {}
            strong {
                "The idea behind this document was the fact that information about devices on a TCP/IP-based network—configuration 
                settings, status information, counters, and so on—could be broken down into a database of variables."
            }
            br {}
            "Those variables could then be collected by management software to monitor and manage the IP-based network."
            br {}
            strong { "After all, the elements of any IP-based machines would have commonalities." }
            br {}
            "For example, a PC, a network printer, and a router would all have commonalities such as interfaces, IP addresses, and 
            buffers."
            br {}
            "Why not create a standardized database of these variables and a simple system for monitoring and managing them?"
            br {}
            "This idea was brilliant, caught on, and became what we know today as Simple Network Management Protocol (SNMP)."
        }

        p { class: "mb-4",
            "This second of three major sections of the chapter now turns our attention to SNMP by
            looking at the major concepts along with the two common versions used today: SNMPv2c
            and SNMPv3."
        }

        {h3_heading("SNMP")}
        p { class: "mb-4",
            strong {
                "SNMP is an application layer protocol that provides a message format for communication
                between what are termed managers and agents."
            }
            br {}
            "An SNMP manager is a network management application running on a PC or server, with that host typically being called a 
            Network Management Station (NMS)."
            br {}
            "Many SNMP agents exist in the network, one per device that is managed."
            br {}
            "The SNMP agent is software running inside each device (router, switch, and so on), with knowledge of all the variables 
            on that device that describe the device's configuration, status, and counters."
            br {}
            strong { "The SNMP manager uses SNMP protocols to communicate with each SNMP agent." }
        }

        {h3_heading("Management Information Base (MIB)")}
        p { class: "mb-4",
            strong {
                "Each agent keeps a database of variables that make up the parameters, status, and counters
                for the operations of the device."
            }
            br {}
            "This database, called the Management Information Base (MIB), has some core elements in common across most networking 
            devices."
            br {}
            "It also has a large number of variables unique to that type of device—for instance, router MIBs will
            include variables not needed on switch MIBs, and vice versa."
            br {}
            "(For perspective, I did a quick check on a router when writing this section and found a little over 7000 MIB 
            variables on a router."
        }

        p { class: "mb-4",
            "Figure 12-8 connects a few of these ideas and terms together."
            br {}
            "First, many companies sell SNMP management products—for example, the Cisco Prime series of management products 
            (www.cisco.com/go/prime) use SNMP (and other protocols) to manage networks."
            br {}
            strong {
                "IOS on routers and switches include an SNMP agent, with built-in MIB, that can be enabled with the configuration"
            }
            " shown later in this chapter."
        }

        img {
            class: "mb-4 rounded-lg",
            loading: "lazy",
            alt: "Figure 12-3 Removing All Single Points of Failure from the Network Design",
            src: asset!("/assets/static/v2p3c12s2f12-8.png", AssetOptions::image().with_avif()),
        }

        p { class: "mb-4",
            strong { "NOTE" }
            " I will include the implementations of SNMPv1, SNMPv2 and SNMPv3 found in appendix D in volume 2 on page 651, 
            in this section."
        }

        {h3_heading("RECAP")}
        ol { class: "list-disc list-inside",
            li {
                "In 1988, RFC 1065, defines that information about devices on a TCP/IP-based network—configuration settings, status 
                information, counters, and so on—could be broken down into a database of variables. Those variables could then be 
                collected by management software to monitor and manage the IP-based network."
            }
            li {
                "SNMP is an application layer protocol that provides a message format for communication between what are termed 
                managers and agents."
            }
            li {
                "An SNMP manager is a network management application running on a PC or server, with that host typically being 
                called a Network Management Station (NMS)."
            }
            li {
                "The SNMP agent is software running inside each device (router, switch, and so on), with knowledge of all the variables on that device that describe the device's configuration, status, and counters."
            }
            li {
                "Each agent keeps a database (called Management Information Base) of variables that make up the parameters, status, and 
                counters for the operations of the device."
            }
        }
    }
}