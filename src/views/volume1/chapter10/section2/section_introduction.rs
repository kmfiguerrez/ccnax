use dioxus::prelude::*;

use crate::{
    components::GreenNote,
    utils::h3_heading
};

#[component]
pub fn SectionIntroductionContent() -> Element {
    rsx! {
        p { class: "mb-4",
            "As introduced in Chapter 9, two neighboring switches can treat multiple parallel links
            between each other as a single logical link called an EtherChannel."
            br {}
            strong {
                "Without EtherChannel, a switch treats each physical port as an independent port, applying MAC learning, 
                forwarding, and STP logic per physical port."
            }
            br {}
            "With EtherChannel, the switch applies all those same processes to a group of physical ports as one 
            entity: the EtherChannel."
            br {}
            "Without EtherChannel, with parallel links between two switches, STP/RSTP would block all 
            links except one, but with EtherChannel, the switch can use all the links, load balancing the traffic over the links."
        }

        GreenNote {
            p {
                strong { "NOTE" }
                " All references to EtherChannel in this chapter refer to Layer 2 EtherChannels, not
                to Layer 3 EtherChannels (as discussed in Chapter 17, “IP Routing in the LAN”). CCNA
                200-301 exam topics include both Layer 2 and Layer 3 EtherChannels."
            }
        }

        {h3_heading("EtherChannel requirements")}
        p { class: "mb-4",
            strong { "EtherChannel may be one of the most challenging switch features to make work." }
            br {}
            "First, the configuration has several options, so you have to remember the details of which options
            work together."
            br {}
            "Second, the switches also require a variety of other interface settings to
            match among all the links in the channel, so you have to know those settings as well."
        }

        {h3_heading("EtherChannel configurations")}
        p { "This section shows how to configure a Layer 2 EtherChannel:" }
        ol { class: "list-disc list-inside mb-4",
            li { "first through manual (static) configuration," }
            li { "and then by allowing dynamic protocols to create the channel." }
        }

        p { class: "mb-4",
            "This section closes with some information about some common configuration issues that occur with
            Layer 2 EtherChannels."
        }

        {h3_heading("RECAP")}
        ol { class: "list-disc list-inside",
            li {
                "EtherChannel is a group of parallel links of the same type and interface configuration settings between
                two switches treated as a single logical link."
            }
            li {
                "Without EtherChannel, with parallel links between two switches, STP/RSTP would block all links except one."
            }
            li {
                "With EtherChannel, the switch can use all the links, load balancing the traffic over the links."
            }
        }
    }
}