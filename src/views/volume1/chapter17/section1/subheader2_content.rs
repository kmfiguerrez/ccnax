use dioxus::prelude::*;

use crate::utils::{TextCommandColor, h3_heading, text_command};

#[component]
pub fn Content() -> Element {
    rsx! {
        p { class: "mb-4",
            "Verifying ROAS Beyond using the "
            {text_command("show running-config", TextCommandColor::Gold)}
            " command, ROAS configuration on a router can be
            best verified with two commands: "
            {text_command("show ip route", TextCommandColor::Gold)}
            " ["
            {text_command("connected", TextCommandColor::Gold)}
            "] and "
            {text_command("show vlans", TextCommandColor::Gold)}
            "."
            br {}
            "As with any router interface, as long as the interface is in an up/up state and has an IPv4 address configured, IOS 
            will put a connected (and local) route in the IPv4 routing table."
            br {}
            strong {
                "So, a first and obvious check would be to see if all the expected connected routes exist."
            }
            br {}
            "Example 17-3 lists the connected routes per the configuration shown in Example 17-1."
        }

        p { class: "mb-4", "See Example 17-3 on volume 1 on page 399." }

        p { class: "mb-4",
            strong {
                "As for interface and subinterface state, note that the ROAS subinterface state does depend
                to some degree on the physical interface state."
            }
            br {}
            "In particular, "
            strong {
                "the subinterface state cannot be better than the state of the matching physical interface."
            }
            br {}
            "For instance, on Router B1 in the examples so far, physical interface G0/0 is in an up/up state, and the 
            subinterfaces are in an up/up state."
            br {}
            "But if you unplugged the cable from that port, the physical port would fail to a down/down state, and the 
            subinterfaces would also fail to a down/down state."
            br {}
            "Example 17-4 shows another example, with the physical interface being shut down, with the subinterfaces
            then automatically changed to an administratively down state as a result."
        }

        p { class: "mb-4", "See Example 17-4 on volume 1 on page 399." }

        p { class: "mb-4",
            "Additionally, the subinterface state can also be enabled and disabled independently from the
            physical interface, using the "
            {text_command("no shutdown", TextCommandColor::Gold)}
            " and "
            {text_command("shutdown", TextCommandColor::Gold)}
            " commands in subinterface configuration mode."
        }

        {h3_heading("The show vlans on routers")}
        p { class: "mb-4",
            "Another useful ROAS verification command, "
            {text_command("show vlans", TextCommandColor::Gold)}
            ", spells out which router trunk interfaces use which VLANs, which VLAN is the native VLAN, plus some packet 
            statistics."
            br {}
            strong {
                "The fact that the packet counters are increasing can be useful when verifying whether traffic is
                happening or not."
            }
            br {}
            "Example 17-5 shows a sample, based on the Router B1 configuration in Example 17-2 (bottom half), in which 
            native VLAN 10 is configured on subinterface G0/0.10."
            br {}
            "Note that the output identifies VLAN 1 associated with the physical interface, VLAN 10
            as the native VLAN associated with G0/0.10, and VLAN 20 associated with G0/0.20."
            br {}
            "It also lists the IP addresses assigned to each interface/subinterface."
        }

        p { class: "mb-4", "See Example 17-5 on volume 1 on page 400." }

        {h3_heading("RECAP")}
        ol { class: "list-disc list-inside",
            li {
                "ROAS configuration on a router can be
                best verified with two commands: "
                {text_command("show ip route", TextCommandColor::Gold)}
                " ["
                {text_command("connected", TextCommandColor::Gold)}
                "] and "
                {text_command("show vlans", TextCommandColor::Gold)}
                "."
            }
            li {
                "As with any router interface, as long as the interface is in an up/up state and has an IPv4 address configured, 
                IOS will put a connected (and local) route in the IPv4 routing table."
                " So, a first and obvious check would be to see if all the expected connected routes exist."
            }
            li {
                "As for interface and subinterface state, note that the ROAS subinterface state does depend
                to some degree on the physical interface state."
            }
            li {
                "Subinterfaces state can also be enabled and disabled independently from the
                physical interface, using the "
                {text_command("no shutdown", TextCommandColor::Gold)}
                " and "
                {text_command("shutdown", TextCommandColor::Gold)}
                " commands in subinterface configuration mode."
            }
            li {
                "Another useful ROAS verification command, "
                {text_command("show vlans", TextCommandColor::Gold)}
                ", spells out which router trunk interfaces use which VLANs, which VLAN is the native VLAN, plus some packet 
                statistics."
            }
        }
    }
}