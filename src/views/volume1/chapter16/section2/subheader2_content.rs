use dioxus::prelude::*;

use crate::utils::{h3_heading, TextCommandColor, text_command};

#[component]
pub fn Content() -> Element {
    rsx! {
        p { class: "mb-4",
            "After a router has added connected routes, the router can route IPv4 packets between those subnets."
            br {}
            "To do so, the router makes use of its IP ARP table."
        }

        p { class: "mb-4",
            strong {
                "The IPv4 ARP table lists the IPv4 address and matching MAC address of hosts connected to
                the same subnet as the router."
            }
            br {}
            "When forwarding a packet to a host on the same subnet, the router encapsulates the packet, with a destination MAC 
            address as found in the ARP table."
            br {}
            "If the router wants to forward a packet to an IP address on the same subnet as the router but does not find an ARP 
            table entry for that IP address, the router will use ARP messages to learn that device's MAC address."
        }

        p { class: "mb-4",
            "Example 16-2 shows R1's ARP table based on the previous example."
            br {}
            "The output lists R1's own IP address of 172.16.1.1, with an age of -, meaning that this entry does not time out."
            br {}
            "Dynamically learned ARP table entries have an upward counter, like the 35-minute value for the ARP table
            entry for IP address 172.16.1.9."
            br {}
            strong {
                "By default, IOS will time out (remove) an ARP table entry after 240 minutes in which the entry is not used."
            }
            br {}
            "("
            strong { "IOS resets the timer to 0 when an ARP table entry is used" }
            ".)"
            br {}
            "Note that to experiment in the lab, you might want to empty all dynamic entries (or a single entry for one IP address) 
            using the "
            {text_command("clear ip arp", TextCommandColor::Gold)}
            " ["
            i { "ip-address" }
            "] EXEC command."
        }

        img {
            class: "mb-4 rounded-lg",
            alt: "Example 16-2 Displaying a Router's IP ARP Table",
            loading: "lazy",
            src: asset!("/assets/static/v1p5c16s2sh2ex16-2.png", AssetOptions::image().with_avif()),
        }

        p { class: "mb-1",
            "Thinking about how Router R1 forwards a packet to host A (172.16.1.9), over that final subnet, R1 does the following:"
        }
        ol { class: "list-decimal list-inside mb-4",
            li { "R1 looks in its ARP table for an entry for 172.16.1.9." }
            li {
                "R1 encapsulates the IP packet in an Ethernet frame, adding destination 0200.3333.3333 to the Ethernet header 
                (as taken from the ARP table)."
            }
            li { "R1 transmits the frame out interface G0/0." }
        }

        {h3_heading("RECAP")}
        ol { class: "list-disc list-inside",
            li {
                "The IPv4 ARP table lists the IPv4 address and matching MAC address of hosts connected to the same subnet as the router."
            }
            li {
                "Dynamically learned ARP table entries have an upward counter value ( not - ) for the age column."
            }
            li { "ARP entry with an age of - does not time out (expire.)" }
            li {
                "By default, IOS will time out (remove) an ARP table entry after 240 minutes in which the entry is not used.
                (IOS resets the timer to 0 when an ARP table entry is used.)"
            }
        }
    }
}