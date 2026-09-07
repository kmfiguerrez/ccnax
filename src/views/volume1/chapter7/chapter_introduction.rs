use dioxus::prelude::*;

#[component]
pub fn ChapterIntroductionContent() -> Element {
    rsx! {
        div {
            p { class: "mb-4",
                "So far in this part, you have learned the skills to navigate the command-line interface (CLI)
                and use commands that configure and verify switch features."
                br {}
                "You learned about "
                strong { "the primary purpose of a switch—forwarding Ethernet frames—" }
                "and learned how to see that process
                in action by looking at the switch MAC address table."
                br {}
                "After learning about the switch data plane in Chapter 5, “Analyzing Ethernet LAN Switching,” you learned a 
                few management plane features in Chapter 6, “Configuring Basic Switch Management,” like how to configure
                the switch to support Telnet and Secure Shell (SSH) by configuring IP address and login
                security."
            }

            p {
                "This chapter focuses on switch interfaces in two major sections."
                br {}
                "The first section shows how you can configure and change the operation of switch interfaces: how to change 
                the speed, duplex, or even disable the interface."
                br {}
                "The second half then focuses on how to use show commands on a switch to verify switch interface status and 
                how to interpret the output to find some of the more common issues with switch interfaces."
            }
        }
    }
}