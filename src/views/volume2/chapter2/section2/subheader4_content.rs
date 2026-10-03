use dioxus::prelude::*;

use crate::utils::{h3_heading, TextCommandColor, text_command};

#[component]
pub fn Content() -> Element {
    rsx! {
        p { class: "mb-4",
            "Troubleshooting IPv4 ACLs requires some attention to detail."
            br {}
            "In particular, you have to be ready to look at the address and wildcard mask and confidently predict the addresses
            matched by those two combined parameters."
            br {}
            "The upcoming practice problems a little later in this chapter can help prepare you for that part of the work."
            br {}
            "But a few other tips can help you verify and troubleshoot ACL problems on the exams as well."
        }

        {h3_heading("The log Keyword")}
        p { class: "mb-4",
            "First, you can tell if the router is matching packets or not with a couple of tools."
            br {}
            "Example 2-2 already showed that IOS keeps statistics about the packets matched by each line of an ACL."
            br {}
            "In addition, if you add the "
            {text_command("log", TextCommandColor::Gold)}
            " keyword to the end of an "
            {text_command("access-list", TextCommandColor::Gold)}
            " command, IOS then issues log messages with occasionalw statistics about matches of that particular line of
            the ACL."
            br {}
            "Both the statistics and the log messages can be helpful in deciding which line in
            the ACL is being matched by a packet."
        }

        p { class: "mb-4",
            "For example, Example 2-4 shows an updated version of ACL 2 from Example 2-3, this time with the "
            {text_command("log", TextCommandColor::Gold)}
            " keyword added."
            br {}
            "The bottom of the example then shows a typical log message, this one showing the resulting match based on a packet 
            with source IP address 10.2.2.1 (as matched with the ACL), to destination address 10.1.1.1 ."
        }

        p { class: "mb-4", "See Example 2-4 in volume 2 on page 38." }

        {h3_heading("Verify the correct interface and direction of packer flow")}
        p { class: "mb-4",
            "When you troubleshoot an ACL for the first time, before getting into the details of the
            matching logic, take the time to think about both the interface on which the ACL is enabled
            and the direction of packet flow."
            br {}
            "Sometimes, the matching logic is perfect—but the ACL has been enabled on the wrong interface, or for the wrong 
            direction, to match the packets as configured for the ACL."
        }

        p { class: "mb-4",
            "For example, Figure 2-9 repeats the same ACL shown earlier in Figure 2-7."
            br {}
            "The first line of that ACL matches the specific host address 10.1.1.1."
            br {}
            "If that ACL exists on Router R2, placing that ACL as an inbound ACL on R2's S0/0/1 interface can work, because packets 
            sent by host 10.1.1.1—on the left side of the figure—can enter R2's S0/0/1 interface."
            br {}
            "However, if R2 enables ACL 1 on its F0/0 interface, for inbound packets, the ACL will never match
            a packet with source IP address 10.1.1.1, because packets sent by host 10.1.1.1 will never
            enter that interface."
            br {}
            "Packets sent by 10.1.1.1 will exit R2's F0/0 interface, but never enter it, just because of the network topology."
        }

        img {
            class: "mb-4 rounded-lg",
            loading: "lazy",
            alt: "Figure 2-9 Example of Checking the Interface and Direction for an ACL",
            src: asset!("/assets/static/v2p1c2s2sh4f2-9.png", AssetOptions::image().with_avif()),
        }
    }
}