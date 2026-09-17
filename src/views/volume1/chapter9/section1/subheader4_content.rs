use dioxus::prelude::*;

use crate::{
    components::KeyTopic, utils::{h3_heading, text_command}
};

#[component]
pub fn Content() -> Element {
    rsx! {
        p { class: "mb-4",
            strong {
                "STP/RSTP works by default on Cisco switches, so all the settings needed by a switch have
                a useful default."
            }
            br {}
            "Switches have a default BID, based on a default priority value and adding a
            universal MAC address that comes with the switch hardware."
            br {}
            "Additionally, switch interfaces have default STP/RSTP costs based on the current operating speed of the 
            switch interfaces."
        }

        {h3_heading("Available STP/RSTP settings that can be configured")}
        p {
            "Network engineers often want to change the STP/RSTP settings to then change the choices
            STP/RSTP makes in a given LAN."
            br {}
            "Two main tools available to the engineer are to configure
            the bridge ID and to change STP/RSTP port costs."
        }
        ul { class: "list-disc list-inside mb-4",
            li {
                "First, to change the BID, the engineer can set the priority used by the switch, while continuing to use the 
                universal MAC address as the final 48 bits of the BID."
                br {}
                strong {
                    "For instance, giving a switch the lowest priority value among all switches will cause that switch to win the root
                    election."
                }
            }
            li {
                "Port costs also have default values, per port, per VLAN."
                br {}
                "You can configure these port costs,
                which will in turn impact many switch's calculations of the root cost."
                br {}
                strong {
                    "For instance, to favor one link, give the ports on that link a lower cost, or to avoid a link, give the ports 
                    a higher cost."
                }
            }
        }

        {h3_heading("Default Port Costs According to IEEE")}
        p { class: "mb-4",
            "Of course, it helps to know the default cost values so you can then choose alternative
            values as needed."
            br {}
            "Table 9-6 lists the default port costs suggested by IEEE."
            br {}
            "IOS on Cisco switches has long used the default settings as defined as far back as the 1998 version of the
            IEEE 802.1D standard."
            br {}
            "The latest IEEE standard to suggest RSTP default costs (as of the publication of this book), 
            the 2018 publication of the 802.1Q standard, suggests values that
            are more useful when using links faster than 10 Gbps."
        }

        KeyTopic {}
        img {
            class: "mb-4 rounded-lg",
            alt: "Table 9-6 Default Port Costs According to IEEE",
            loading: "lazy",
            src: asset!("/assets/static/v1p3c9s1sh4t9-6.png", AssetOptions::image().with_avif()),
        }

        p { class: "mb-4",
            strong {
                "Of note in regards to these defaults, the cost defaults based on the operating speed of the
                link, not the maximum speed."
            }
            br {}
            "That is, if a 10/100/1000 port runs at 10 Mbps for some reason, its default STP cost on a Cisco switch is 100, 
            the default cost for an interface running at 10 Mbps."
            br {}
            "Also, if you prefer the defaults in the right-side column of Table 9-6, note that
            Cisco Catalyst switches can be configured to use those values as defaults with a single global
            configuration command on each switch ("
            {
                text_command(
                    "spanning-tree pathcost method long",
                    text_command::TextCommandColor::Gold,
                )
            }
            ")."
        }

        {h3_heading("RECAP")}
        ol { class: "list-disc list-inside",
            li {
                "STP/RSTP works by default on Cisco switches, so all the settings needed by a switch have a useful default."
            }
            li {
                "Switches have a default BID, based on a default priority value and adding a universal MAC address that comes 
                with the switch hardware."
            }
            li {
                "Switch interfaces have default STP/RSTP costs based on the current operating speed of the switch interfaces."
            }
            li {
                "Network engineers often want to change the STP/RSTP settings to then change the choices STP/RSTP makes in a 
                given LAN. Two main tools available to the engineer are to configure the bridge ID and to change STP/RSTP port costs."
            }
            li {
                "Giving a switch the lowest priority value among all switches will cause that switch to win the root election."
            }
            li { "Port costs also have default values, per port, per VLAN." }
            li {
                "To favor one link, give the ports on that link a lower cost, or to avoid a link, give the ports a higher cost."
            }
            li {
                "Note that the cost defaults are based on the current operating speed of the link, not the maximum speed."
            }
        }
    }

}