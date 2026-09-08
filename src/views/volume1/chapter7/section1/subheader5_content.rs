use dioxus::prelude::*;

use crate::{components::GreenNote, utils::{TextCommandColor, h3_heading, text_command}};

#[component]
pub fn Content() -> Element {
    rsx! {
        p { class: "mb-4",
            "For any 10/100 or 10/100/1000 interfaces—that is, interfaces that can run at different
            speeds—Cisco Catalyst switches default to a setting of "
            {text_command("duplex auto", TextCommandColor::Gold)}
            " and "
            {text_command("speed auto", TextCommandColor::Gold)}
            "."
            br {}
            "As a result, those interfaces attempt to automatically determine the speed and duplex setting to use."
            br {}
            " Alternatively, you can configure most devices, switch interfaces included, to use a specific speed and/or duplex."
        }

        p { class: "mb-4",
            "In practice, using autonegotiation is easy: just leave the speed and duplex at the default setting, and let the 
            switch port negotiate what settings to use on each port."
            br {}
            "However, problems can occur due to unfortunate combinations of configuration."
            br {}
            "Therefore, this next topic walks through more detail about the concepts behind autonegotiation, so you know better 
            how to interpret the meaning of the switch "
            {text_command("show", TextCommandColor::Gold)}
            " commands and when to choose to use a particular
            configuration setting."
        }

        {h3_heading("Autonegotiation Under Working Conditions")}
        p { class: "mb-4",
            "Ethernet devices on the ends of a link must use the same standard; otherwise, they cannot correctly send data."
            br {}
            "For example, a NIC cannot use 100BASE-T, which uses a two-pair UTP cable with a 100-Mbps speed, while the switch 
            port on the other end of the link uses 1000BASE-T."
            br {}
            "Even if you used a cable that works with Gigabit Ethernet, the link would not
            work with one end trying to send at 100 Mbps while the other tried to receive the data at
            1000 Mbps."
        }

        p { class: "mb-4",
            "Upgrading to new and faster Ethernet standards becomes a problem because both ends have
            to use the same standard."
            br {}
            "For example, if you replace an old PC with a new one, the old one might have been using 100BASE-T while the new 
            one uses 1000BASE-T."
            br {}
            "The switch port on the other end of the link needs to now use 1000BASE-T, so you upgrade the switch."
            br {}
            "If that switch had ports that would use only 1000BASE-T, you would need to upgrade all the other
            PCs connected to the switch."
            br {}
            "So, having both PC network interface cards (NIC) and switch ports that support multiple standards/speeds makes 
            it much easier to migrate to the next better standard."
        }

        p { class: "mb-4",
            "The IEEE autonegotiation protocol helps makes it much easier to operate a LAN when NICs
            and switch ports support multiple speeds."
            br {}
            "IEEE autonegotiation (IEEE standard 802.3u) defines a protocol that lets the two UTP-based Ethernet nodes 
            on a link negotiate so that they each choose to use the same speed and duplex settings."
            br {}
            "The protocol messages flow outside the normal Ethernet electrical frequencies as out-of-band signals 
            over the UTP cable."
            br {}
            strong {
                "Basically, each node states what it can do, and then each node picks the best options that
                both nodes support: the fastest speed and the best duplex setting, with full duplex being
                better than half duplex."
            }
        }

        {h3_heading("RECAP")}
        ol { class: "list-disc list-inside",
            li {
                "For any 10/100 or 10/100/1000 interfaces—that is, interfaces that can run at different
                speeds—Cisco Catalyst switches default to a setting of "
                {text_command("duplex auto", TextCommandColor::Gold)}
                " and "
                {text_command("speed auto", TextCommandColor::Gold)}
                "."
            }
            li {
                "Ethernet devices on the ends of a link must use the same standard; otherwise, they cannot correctly send data."
            }
            li {
                "IEEE autonegotiation (IEEE standard 802.3u) defines a protocol that lets the two UTP-based Ethernet nodes on a link 
                negotiate so that they each choose to use the same speed and duplex settings."
            }
        }
    }
}