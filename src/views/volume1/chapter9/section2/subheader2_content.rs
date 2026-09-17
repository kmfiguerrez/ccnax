use dioxus::prelude::*;

use crate::{
    components::KeyTopic,
    utils::h3_heading
};

#[component]
pub fn Content() -> Element {
    rsx! {
        p { class: "mb-4",
            "For various reasons, the STP convergence process requires the use of three timers, listed in Table 9-7."
            br {}
            strong {
                "Note that all switches use the timers as dictated by the root switch, which the root
                lists in its periodic Hello BPDU messages."
            }
        }

        KeyTopic {}
        img {
            class: "mb-4 rounded-lg",
            alt: "Table 9-7 STP Timers",
            loading: "lazy",
            src: asset!("/assets/static/v1p3c9s2sh1t9-7.png", AssetOptions::image().with_avif()),
        }

        {h3_heading("Absence of Hellos BPDUs")}
        p { class: "mb-4",
            "If a switch does not get an expected Hello BPDU within the Hello time, the switch continues as normal."
            br {}
            "However, if the Hellos do not show up again within MaxAge time, the switch reacts by taking steps to change the 
            STP topology."
            br {}
            "With default settings, MaxAge is 20 seconds (10 times the default Hello timer of 2 seconds)."
            br {}
            "So, a switch would go 20 seconds without hearing a Hello before reacting."
        }

        p { class: "mb-4",
            "The best way to describe STP convergence is to show an example using the same familiar topology."
            br {}
            "Figure 9-7 shows the same familiar figure, with SW3's Gi0/2 in a blocking state,
            but SW1's Gi0/2 interface has just failed."
        }

        img {
            class: "mb-4 rounded-lg",
            alt: "Figure 9-7 Initial STP State Before SW1-SW3 Link Fails",
            loading: "lazy",
            src: asset!("/assets/static/v1p3c9s2sh1f9-7.png", AssetOptions::image().with_avif()),
        }

        p { class: "mb-4",
            "In the scenario shown in the figure, SW3 reacts to the change because SW3 fails to receive
            its expected Hellos on its Gi0/1 interface."
            br {}
            "However, SW2 does not need to react because SW2 continues to receive its periodic Hellos in its Gi0/2 interface."
            br {}
            "In this case, SW3 reacts either when MaxAge time passes without hearing the Hellos, or as soon as SW3 notices that
            interface Gi0/1 has failed."
            br {}
            "(If the interface fails, the switch can assume that the Hellos will not be arriving in that interface anymore.)"
        }

        {h3_heading("STP convergence in action on sw3")}
        p { class: "mb-4",
            "Now that SW3 can act, it begins by reevaluating the choice of root switch."
            br {}
            "SW3 still receives the Hellos from SW2, as forwarded from the root (SW1)."
            br {}
            "SW1 still has a lower BID than SW3; otherwise, SW1 would not have already been the root."
            br {}
            "So, SW3 decides that SW1 wins the root election and that SW3 is not the root."
        }

        p { class: "mb-4",
            "Next, SW3 reevaluates its choice of RP."
            br {}
            "At this point, SW3 is receiving Hellos on only one interface: Gi0/2."
            br {}
            "Whatever the calculated root cost, Gi0/2 becomes SW3's new RP."
            br {}
            "(The cost would be 8, assuming the STP costs had no changes since Figures 9-5 and 9-6."
        }

        p { class: "mb-4",
            "SW3 then reevaluates its role as DP on any other interfaces."
            br {}
            "In this example, no real work needs to be done."
            br {}
            "SW3 was already DP on interface Fa0/13, and it continues to be the DP
            because no other switches connect to that port."
        }

        {h3_heading("RECAP")}
        ol { class: "list-disc list-inside",
            li { "For various reasons, the STP convergence process requires the use of three timers." }
            li {
                "Note that all switches use the timers as dictated by the root switch, which the root lists in its 
                periodic Hello BPDU messages."
            }
            li {
                "If a switch does not get an expected Hello BPDU within MaxAge time, the switch reacts by taking steps to 
                change the STP topology."
            }
        }
    }
}