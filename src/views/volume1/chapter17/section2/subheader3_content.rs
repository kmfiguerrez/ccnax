use dioxus::prelude::*;

use crate::{
    components::KeyTopic, utils::{TextCommandColor, h3_heading, text_command}
};

#[component]
pub fn Content() -> Element {
    rsx! {
        p { class: "mb-1",
            "There are two big topics to investigate when troubleshooting routing over LANs with SVIs."
        }
        ol { class: "list-disc list-inside mb-4",
            li { "First, you have to make sure the switch has been enabled to support IP routing." }
            li {
                "Second, the VLAN associated with each VLAN interface must be known and active on the local switch;
                otherwise, the VLAN interfaces do not come up."
            }
        }

        {h3_heading("First check: Layer 3 switching")}
        p { class: "mb-4",
            "First, about enabling IP routing, "
            strong {
                "note that some models of Cisco switches default to enable Layer 3 switching, and some do not."
            }
            br {}
            "So, to make sure your switch supports Layer 3 routing, look to those first few configuration commands listed in 
            the configuration checklist found in the earlier section “Configuring Routing Using Switch SVIs.”"
            br {}
            "Those commands are "
            {text_command("sdm prefer", TextCommandColor::Gold)}
            " (followed by a "
            {text_command("reload", TextCommandColor::Gold)}
            ") and then ip routing (after the "
            {text_command("reload", TextCommandColor::Gold)}
            ")."
        }

        p { class: "mb-4",
            "The "
            {text_command("sdm prefer", TextCommandColor::Gold)}
            " command changes how the switch forwarding chips allocate memory for
            different forwarding tables, and changes to those tables require a reload of the switch."
            br {}
            strong {
                "By default, many access switches that support Layer 3 switching still have an SDM default that
            does not allocate space for an IP routing table."
            }
            br {}
            "Once changed and reloaded, the "
            {text_command("ip routing", TextCommandColor::Gold)}
            " command then enables IPv4 routing in IOS software."
            br {}
            "Both are necessary before some Cisco switches will act as a Layer 3 switch."
        }

        p { class: "mb-4",
            "Example 17-8 shows some symptoms on a router for which Layer 3 switching had not yet
            been enabled by the "
            {text_command("sdm prefer", TextCommandColor::Gold)}
            " command."
            br {}
            "As you can see, both the "
            {text_command("show ip route", TextCommandColor::Gold)}
            " EXEC command and the "
            {text_command("ip routing config", TextCommandColor::Gold)}
            " command are rejected because they do not exist to IOS until the "
            {text_command("sdm prefer", TextCommandColor::Gold)}
            " command has been used (followed by a reload of the switch)."
        }

        img {
            class: "mb-4 rounded-lg",
            alt: "Example 17-8 Evidence That a Switch Has Not Yet Enabled IPv4 Routing",
            loading: "lazy",
            src: asset!("/assets/static/v1p3c17s2sh3ex17-8.png", AssetOptions::image().with_avif()),
        }

        {h3_heading("Second check: SVI state")}
        p { class: "mb-1",
            "The second big area to investigate when troubleshooting SVIs relates to the SVI state, a state
            that ties to the state of the associated VLANs."
            br {}
            "Each VLAN interface has a matching VLAN of the same number, and the VLAN interface's state is tied to the state of 
            the VLAN in certain ways."
            br {}
            "In particular, for a VLAN interface to be in an up/up state:"
        }
        KeyTopic {}
        ol { class: "pl-1 mb-4",
            // Step 1
            li {
                span { class: "text-blue-500 font-bold mr-4", "Step 1." }
                "The VLAN must be defined on the local switch (either explicitly or learned with VTP)."
            }
            // Step 2
            li {
                span { class: "text-blue-500 font-bold mr-4", "Step 2." }
                "The switch must have at least one up/up interface using the VLAN, either/both:"
                ol { class: "sm:pl-17",
                    li {
                        span { class: "uppercase text-blue-500 font-semibold", "a." }
                        " An up/up access interface assigned to that VLAN"
                    }
                    li {
                        span { class: "uppercase text-blue-500 font-semibold", "b." }
                        " A trunk interface for which the VLAN is in the allowed list, is STP forwarding, and is not VTP pruned"
                    }
                }
            
            }
            // Step 3
            li {
                span { class: "text-blue-500 font-bold mr-4", "Step 3." }
                " The VLAN (not the VLAN interface) must be administratively enabled (that is, not "
                {text_command("shutdown", TextCommandColor::Gold)}
                ")."
            }
            // Step 4
            li {
                span { class: "text-blue-500 font-bold mr-4", "Step 4." }
                " The VLAN interface (not the VLAN) must be administratively enabled (that is, not "
                {text_command("shutdown", TextCommandColor::Gold)}
                ")."
            }
        }

        p { class: "mb-4",
            strong {
                "When working through the steps in the list, keep in mind that the VLAN and the VLAN
                interface are related but separate ideas, and the configuration items are separate in the CLI."
            }
            br {}
            "The VLAN interface is a switch's Layer 3 interface connected to the VLAN."
            br {}
            "If you want to route packets for the subnets on VLANs 11, 12, and 13, the matching VLAN interfaces must
            be numbered 11, 12, and 13."
            br {}
            "And both the VLANs and the VLAN interfaces can be disabled and enabled with the "
            {text_command("shutdown", TextCommandColor::Gold)}
            "shutdown"
            " and "
            {text_command("no shutdown", TextCommandColor::Gold)}
            " commands (as mentioned in Steps 3 and 4 in the previous list), so you have to check for both."
        }

        p { class: "mb-1",
            "Example 17-9 shows three scenarios, each of which leads to one of the VLAN interfaces in
            the previous configuration example (Figure 17-3, Example 17-6) to fail."
            br {}
            "At the beginning of the example, all three VLAN interfaces are up/up. VLANs 10, 20, and 30 each have at least
            one access interface up and working. The example works through three scenarios:"
        }
        ol { class: "list-disc list-inside mb-4",
            li {
                span { class: "font-bold", "Scenario 1:" }
                "  The last access interface in VLAN 10 is shut down (F0/1), so IOS shuts down the VLAN 10 interface."
            }
            li {
                span { class: "font-bold", "Scenario 2:" }
                "  VLAN 20 (not VLAN interface 20, but VLAN 20) is deleted, which results in
                IOS then bringing down (not shutting down) the VLAN 20 interface."
            }
            li {
                span { class: "font-bold", "Scenario 3:" }
                "  VLAN 30 (not VLAN interface 30, but VLAN 30) is shut down, which results
                in IOS then bringing down (not shutting down) the VLAN 30 interface."
            }
        }

        p { class: "mb-4", "See Example 17-9 in volume 1 on page 405." }

        {h3_heading("RECAP")}
        ol { class: "list-disc list-inside",
            li {
                "To troubleshoot routing over LANs with SVIs, check two things. First, make sure layer 3 routing is enabled
                on the switch. Second, the VLAN associated with each VLAN interface must be known and active on the local 
                switch; otherwise, the VLAN interfaces do not come up."
            }
            li {
                "Note that some models of Cisco switches default to enable Layer 3 switching, and some do not."
            }
            li {
                "The "
                {text_command("sdm prefer", TextCommandColor::Gold)}
                " command changes how the switch forwarding chips allocate memory for
                different forwarding tables, and changes to those tables require a reload of the switch."
            }
            li {
                "By default, many access switches that support Layer 3 switching still have an SDM default that does not 
                allocate space for an IP routing table. If the "
                {text_command("show ip routing", TextCommandColor::Gold)}
                " EXEC command is issued, the codes for the routing table will not be even shown."
            }
            li {
                "Keep in mind that the VLAN and the VLAN interface are related but separate ideas, and the configuration items 
                are separate in the CLI."
            }
            li { "The VLAN interface is a switch's Layer 3 interface connected to the VLAN." }
        }

    }
}