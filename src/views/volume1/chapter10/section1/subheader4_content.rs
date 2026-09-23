use dioxus::prelude::*;

use crate::utils::{h3_heading, text_command};

#[component]
pub fn Content() -> Element {
    rsx! {
        p { class: "mb-4",
            strong {
                "Cisco Catalyst switches configure the priority value using a number that represents a 16-bit
            value; however, the system ID extension exists as the low-order 12 bits of that same number."
            }
            br {}
            "This next topic works through connecting those ideas."
        }

        p { class: "mb-4",
            strong {
                "When the switch builds its BID to use for RSTP in a VLAN, it must combine the configured
                priority with the VLAN ID of that VLAN."
            }
            br {}
            strong {
                "Interestingly, the configured priority results in a 16-bit priority that always ends with 12 binary 0s."
            }
            br {}
            "That fact makes the process of combining values to create the BID a little simpler for the switch and possibly a 
            little simpler for network engineers once you understand it all."
        
        }

        p { class: "mb-4",
            "First, consider the process shown in Figure 10-5."
            br {}
            "The top shows the configured priority value (decimal 32768), in 
            16-bit binary form, with a System ID Extension of 12 zeros."
            br {}
            "Moving down the figure, you see the binary version of a VLAN ID (decimal 9)."
            br {}
            "At the last step, the switch replaces those last 12 bits of the System ID Extension with the value that
            matches the VLAN ID and uses that value as the first 16 bits of the BID."
        }

        img {
            class: "mb-4 rounded-lg",
            alt: "Figure 10-5 Configured Priority (16-Bit) and System ID Extension (12-Bit) Added",
            loading: "lazy",
            src: asset!("/assets/static/v1p3c10s1sh4f10-5.png", AssetOptions::image().with_avif()),
        }

        p {
            "As it turns out, the process shown in Figure 10-5 is just the sum of the two numbers—both
            in binary and decimal."
            br {}
            "To see an example, refer to upcoming Example 10-3, which demonstrates the following details:"
        }
        ol { class: "list-disc list-inside mb-4",
            li { "The output shows details about VLAN 9." }
            li {
                "The root switch has been configured with the "
                {
                    text_command(
                        "spanning-tree vlan 9 priority 24576",
                        text_command::TextCommandColor::Gold,
                    )
                }
                " command."
            }
            li {
                "The local switch (the switch on which the command was gathered) has been configured
                with the "
                {
                    text_command(
                        "spanning-tree vlan 9 priority 32768",
                        text_command::TextCommandColor::Gold,
                    )
                }
                " command."
            }
            li {
                " Conveniently, the decimal equivalent of the two switches' first 16 bits—the original
                16-bit priority field—can be easily calculated in decimal. In this example:"
                ol { class: "list-disc list-inside pl-5",
                    li {
                        span { class: "font-bold", "Root Switch:" }
                        " 24,576 (priority) + 9 (VLAN ID) = 24585"
                    }
                    li {
                        span { class: "font-bold", "Local Switch:" }
                        " 32,768 (priority) + 9 (VLAN ID) = 32777"
                    }
                }
            }
        }

        p { class: "mb-4",
            "The output in Example 10-3 matches this logic."
            br {}
            "The top highlight shows the priority of the root switch (24585), which is the sum of the root switch's priority 
            setting (configured as 24,576) plus 9 for the VLAN ID."
            br {}
            "The second highlight shows a value of 32,777, calculated as the local switch's priority setting of 32,768 plus 9 for 
            the VLAN ID."
        }

        img {
            class: "mb-4 rounded-lg",
            alt: "Example 10-3 Examining the 16-bit Priority as Interpreted in Cisco show Commands",
            loading: "lazy",
            src: asset!("/assets/static/v1p3c10s1sh4ex10-3.png", AssetOptions::image().with_avif()),
        }

        {h3_heading("RECAP")}
        ol { class: "list-disc list-inside",
            li {
                "Cisco Catalyst switches configure the priority value using a number that represents a 16-bit value; however, 
                the system ID extension exists as the low-order 12 bits of that same number."
            }
            li {
                "When the switch builds its BID to use for RSTP in a VLAN, it must combine the configured priority with the 
                VLAN ID of that VLAN."
            }
            li {
                "Interestingly, the list of available priority values in the command "
                {text_command("spanning-tree vlan", text_command::TextCommandColor::Gold)}
                i { " x " }
                {text_command("priority", text_command::TextCommandColor::Gold)}
                i { " value " }
                "results in a 16-bit priority that always ends with 12 binary 0s."
            }
            li {
                "A configured priority value that always ends with 12 binary 0s makes the process of combining values 
                (4-bit priority field and the 12-bit VLAN ID) to create the BID a little simpler for the switch and possibly a 
                little simpler for network engineers once you understand it all."
            }
        }
    }
}