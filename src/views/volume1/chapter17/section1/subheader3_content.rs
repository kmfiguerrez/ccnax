use dioxus::prelude::*;

use crate::{
    components::KeyTopic, utils::{TextCommandColor, h3_heading, text_command}
};

#[component]
pub fn Content() -> Element {
    rsx! {
        p { class: "mb-4",
            strong {
                "The biggest challenge when troubleshooting ROAS has to do with the fact that if you misconfigure only the router or 
                misconfigure only the switch, the other device on the trunk has no way to know that the other side is misconfigured."
            }
            br {}
            "That is, if you check the "
            {text_command("show ip route", TextCommandColor::Gold)}
            " and "
            {text_command("show vlans", TextCommandColor::Gold)}
            " commands on a router, and the output looks like it matches the  intended configuration, and the connected routes 
            for the correct subinterfaces show up, routing may still fail because of problems on the attached switch."
            br {}
            "So, troubleshooting ROAS often begins with checking the configuration on both the router and switch because there is 
            no status output on either device that tells you where the problem might be."
        }

        KeyTopic {}
        p { class: "mb-1",
            "First, to check ROAS on the router, you need to start with the intended configuration and
            ask questions about the configuration:"
        }
        ol { class: "list-decimal list-inside mb-4",
            li {
                "Is each non-native VLAN configured on the router with an "
                {text_command("encapsulation dot1q", TextCommandColor::Gold)}
                i { " vlan-id" }
                " command on a subinterface?"
            }
            li {
                "Do those same VLANs exist on the trunk on the neighboring switch ("
                {text_command("show interfaces trunk", TextCommandColor::Gold)}
                "), and are they in the allowed list, not VTP pruned, and not STP blocked?"
            }
            li {
                "Does each router ROAS subinterface have an IP address/mask configured per the
                planned configuration?"
            }
            li {
                "If using the native VLAN, is it configured correctly on the router either on a subinterface (with an "
                {text_command("encapsulation dot1q", TextCommandColor::Gold)}
                i { " vlan-id " }
                {text_command("native", TextCommandColor::Gold)}
                " command) or implied on the physical interface?"
            }
            li {
                "Is the same native VLAN configured on the neighboring switch's trunk in comparison
                to the native VLAN configured on the router?"
            }
            li {
                "Are the router physical or ROAS subinterfaces configured with a "
                {text_command("shutdown", TextCommandColor::Gold)}
                " command?"
            }
        }

        {h3_heading("VLAN Trunking issues on the switch")}
        p { class: "mb-4",
            "For some of these steps, you need to be ready to investigate possible VLAN trunking issues on the LAN switch."
            br {}
            "The reason is that on many Cisco routers, router interfaces do not negotiate trunking."
            br {}
            "As a result, ROAS relies on static trunk configuration on both the router and switch."
            br {}
            strong {
                "If the switch has any problems with VLANs or the VLAN trunking configuration on
                its side of the trunk, the router has no way to realize that the problem exists."
            }
        }

        p { class: "mb-4",
            "For example, imagine you configured ROAS on a router just like in Example 17-1 or Example 17-2."
            br {}
            "However, the switch on the other end of the link had no matching configuration."
            br {}
            "For instance, maybe the switch did not even define VLANs 10 and 20."
            br {}
            "Maybe the switch did not configure trunking on the port connected to the router."
            br {}
            "Even with blatant misconfiguration or missing configuration on the switch, the router still shows up/up ROAS interfaces 
            and subinterfaces, IP routes in the output of "
            {text_command("show ip route", TextCommandColor::Gold)}
            ", and meaningful configuration information in the output of the "
            {text_command("show vlans", TextCommandColor::Gold)}
            " command."
        }

        {h3_heading("RECAP")}
        ol { class: "list-disc list-inside",
            li {
                "The biggest challenge when troubleshooting ROAS has to do with the fact that if you misconfigure only the router 
                or misconfigure only the switch, the other device on the trunk has no way to know that the other side is 
                misconfigured."
            }
            li {
                "That is, if you check the "
                {text_command("show ip route", TextCommandColor::Gold)}
                " and "
                {text_command("show vlans", TextCommandColor::Gold)}
                " commands on a router, and the output looks like it matches the  intended configuration, and the connected routes 
                for the correct subinterfaces show up, routing may still fail because of problems on the attached switch."
            }
            li {
                "So, troubleshooting ROAS often begins with checking the configuration on both the router and switch because 
                there is  no status output on either device that tells you where the problem might be."
            }
            li {
                "Many Cisco routers, router interfaces do not negotiate trunking."
                " As a result, ROAS relies on static trunk configuration on both the router and switch."
            }
            li {
                "If the switch has any problems with VLANs or the VLAN trunking configuration on its side of the trunk, the 
                router has no way to realize that the problem exists."
            }
            li {
                "Even with blatant misconfiguration or missing configuration on the switch, the router still shows up/up ROAS interfaces 
                and subinterfaces, IP routes in the output of "
                {text_command("show ip route", TextCommandColor::Gold)}
                ", and meaningful configuration information in the output of the "
                {text_command("show vlans", TextCommandColor::Gold)}
                " command."
            }
        }
    }
}