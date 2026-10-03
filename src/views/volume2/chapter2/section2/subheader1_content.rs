use dioxus::prelude::*;

use crate::{components::KeyTopic, utils::{h3_heading, TextCommandColor, text_command}};

#[component]
pub fn Content() -> Element {
    rsx! {
        p { class: "mb-4",
            strong {
                "A single ACL is both a single entity and, at the same time, a list of one or more configuration commands."
            }
            br {}
            "As a single entity, the configuration enables the entire ACL on an interface,
            in a specific direction, as shown earlier in Figure 2-1."
            br {}
            "As a list of commands, each command has different matching logic that the router must apply to each packet when 
            filtering using that ACL."
        }

        KeyTopic {}
        p { class: "mb-4",
            "When doing ACL processing, the router processes the packet, compared to the ACL, as follows:"
            br {}
            span { class: "pl-4 font-bold",
                "ACLs use first-match logic. Once a packet matches one line in the ACL, the router takes
                the action listed in that line of the ACL and stops looking further in the ACL."
            }
        }

        p { class: "mb-4",
            "To see exactly what that means, consider the example built around Figure 2-4."
            br {}
            "The figure shows an example ACL 1 with three lines of pseudocode."
            br {}
            "This example applies ACL 1 on R2's S0/0/1 interface, inbound (the same location as in earlier Figure 2-2)."
        }

        img {
            class: "mb-4 rounded-lg",
            loading: "lazy",
            alt: "Figure 2-4 Backdrop for Discussion of List Process with IP ACLs",
            src: asset!("/assets/static/v2p3c2s2sh1f2-4.png", AssetOptions::image().with_avif()),
        }

        p { class: "mb-4",
            "Consider the first-match ACL logic for a packet sent by host A to server S1."
            br {}
            "The source IP address will be 10.1.1.1, and it will be routed so that it enters R2's S0/0/1 interface, driving
            R2's ACL 1 logic."
            br {}
            "R2 compares this packet to the ACL, matching the first item in the list with a permit action."
            br {}
            "So this packet should be allowed through, as shown in Figure 2-5, on the left."
        }

        img {
            class: "mb-4 rounded-lg",
            loading: "lazy",
            alt: "Figure 2-5 ACL Items Compared for Packets from Hosts A, B, and C in Figure 2-4",
            src: asset!("/assets/static/v2p3c2s2sh1f2-5.png", AssetOptions::image().with_avif()),
        }

        p { class: "mb-4",
            "Next , consider a packet sent by host B, source IP address 10.1.1.2."
            br {}
            "When the packet enters R2's S0/0/1 interface, R2 compares the packet to ACL 1's first statement and does not make
            a match (10.1.1.1 is not equal to 10.1.1.2)."
            br {}
            "R2 then moves to the second statement, which requires some clarification."
            br {}
            "The ACL pseudocode, back in Figure 2-4, shows 10.1.1.x, which
            is meant to be shorthand that any value can exist in the last octet."
            br {}
            "Comparing only the first three octets, R2 decides that this latest packet does have a source IP address that begins
            with the first three octets 10.1.1, so R2 considers that to be a match on the second statement."
            br {}
            "R2 takes the listed action (deny), discarding the packet. R2 also stops ACL processing
            on the packet, ignoring the third line in the ACL."
        }

        p { class: "mb-4",
            "Finally, consider a packet sent by host C, again to server S1."
            br {}
            "The packet has source IP address 10.3.3.3, so when it enters R2's S0/0/1 interface and drives ACL processing on
            R2, R2 looks at the first command in ACL 1."
            br {}
            "R2 does not match the first ACL command (10.1.1.1 in the command is not equal to the packet's 10.3.3.3)."
            br {}
            "R2 looks at the second command, compares the first three octets (10.1.1) to the packet source IP address (10.3.3), and
            still finds no match."
            br {}
            "R2 then looks at the third command."
            br {}
            "In this case, the wildcard means ignore the last three octets and just compare the first octet (10), so the packet 
            matches."
            br {}
            "R2 then takes the listed action (permit), allowing the packet to keep going."
        }

        p { class: "mb-4",
            strong {
                "This sequence of processing an ACL as a list happens for any type of IOS ACL: IP, other
            protocols, standard or extended, named or numbered."
            }
        }

        {h3_heading("The implicit deny all statement")}
        p { class: "mb-4",
            "Finally, if a packet does not match any of the items in the ACL, the packet is discarded."
            br {}
            strong {
                "The reason is that every IP ACL has a deny all statement implied at the end of the ACL."
            }
            br {}
            "It does not exist in the configuration, but if a router keeps searching the list, and no match is made
            by the end of the list, IOS considers the packet to have matched an entry that has a "
            {text_command("deny", TextCommandColor::Gold)}
            " action."
        }

        {h3_heading("RECAP")}
        ol { class: "list-disc list-inside",
            li {
                "A single ACL is both a single entity and, at the same time, a list of one or more configuration commands."
            }
            li {
                "As a single entity, the configuration enables the entire ACL on an interface, in a specific direction."
            }
            li {
                "As a list of commands, each command has different matching logic that the router must apply to each packet when 
                filtering using that ACL."
            }
            li {
                "ACLs use first-match logic. Once a packet matches one line in the ACL, the router takes the action listed in that 
                line of the ACL and stops looking further in the ACL."
            }
            li {
                "This sequence of processing an ACL as a list happens for any type of IOS ACL: IP, other protocols, standard or 
                extended, named or numbered."
            }
            li {
                "By default every IP ACL has an implicit "
                {text_command("deny all", TextCommandColor::Gold)}
                " statement implied at the end of the ACL."
            }
        
        }
    }
}