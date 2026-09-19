use dioxus::prelude::*;

use crate::{
    components::{KeyTopic, GreenNote},
    utils::h3_heading
};

#[component]
pub fn Content() -> Element {
    rsx! {
        p { class: "mb-4",
            "With STP, each nonroot switch places one port in the STP root port (RP) role."
            br {}
            "RSTP follows that same convention, with the same exact rules for choosing the RP."
            br {}
            "RSTP then takes another step beyond STP, naming other possible RPs, identifying them as alternate ports."
        }

        {h3_heading("Becoming an alternate port")}
        p { class: "mb-4",
            strong {
                "To be an alternate port, both the RP and the alternate port must receive Hellos that identify
                the same root switch."
            }
            br {}
            "For instance, in Figure 9-8, SW1 is the root."
            br {}
            "SW3 will receive Hello BPDUs on two ports: G0/1 and G0/2."
            br {}
            "Both Hellos list SW1's bridge ID (BID) as the root switch, so whichever port is not the root port meets the criteria 
            to be an alternate port."
            br {}
            "SW3 picks G0/1 as its root port in this case and then makes G0/2 an alternate port."
        }

        p {
            strong { "An alternate port basically works like the second-best option for the root port." }
            br {}
            "The alternate port can take over for the former root port, often very rapidly, without requiring a wait in
            other interim RSTP states."
        }
        p {
            "For instance, when the root port fails, or when Hellos stop arriving on the original root port, the switch changes 
            the former root port's role and state:"
        }
        ol { class: "list-disc list-inside",
            li { "(a) the role from root port to a disabled port," }
            li {
                "and (b) the state from forwarding to discarding (the equivalent of STP's blocking state)."
            }
        }
        p {
            "Then, without waiting on any timers, the switch changes roles and state for 
            the alternate port: its role changes to be the root port, with a forwarding state."
        }

        p { class: "mb-4",
            "Notably, the new root port also does not need to spend time in other states, such as learning state, instead 
            moving immediately to forwarding state."
        }

        img {
            class: "mb-4 rounded-lg",
            alt: "Figure 9-8 Example of SW3 Making G0/2 Become an Alternate Port",
            loading: "lazy",
            src: asset!("/assets/static/v1p3c9s3sh2f9-8.png", AssetOptions::image().with_avif()),
        }

        {h3_heading("RSTP convergence")}
        p { class: "mb-4",
            "Figure 9-9 shows an example of RSTP convergence."
            br {}
            "SW3's root port before the failure shown
            in this figure is SW3's G0/1, the link connected directly to SW1 (the root switch)."
            br {}
            "Then SW3's link to SW1 fails as shown in Step 1 of the figure."
        }

        img {
            class: "mb-4 rounded-lg",
            alt: "Figure 9-9 Convergence Events with SW3 G0/1 Failure",
            loading: "lazy",
            src: asset!("/assets/static/v1p3c9s3sh2f9-9.png", AssetOptions::image().with_avif()),
        }

        p { "Following the steps in Figure 9-9:" }
        ol { class: "pl-1 mb-4",
            li {
                span { class: "text-blue-500 mr-4", "Step 1." }
                " The link between SW1 and SW3 fails, so SW3's current root port (Gi0/1) fails."
            }
            li {
                span { class: "text-blue-500 mr-4", "Step 2." }
                " SW3 and SW2 exchange RSTP messages to confirm that SW3 will now transition its former alternate port (Gi0/2) to 
                be the root port. This action causes SW2 to flush the required MAC table entries."
            }
            li {
                span { class: "text-blue-500 mr-4", "Step 3." }
                " SW3 transitions Gi0/1 to the disabled role and Gi0/2 to the root port role."
            }
            li {
                span { class: "text-blue-500 mr-4", "Step 4." }
                " SW3 transitions Gi0/2 to a forwarding state immediately, without using learning
                state, because this is one case in which RSTP knows the transition will not create a loop."
            }
        }

        p { class: "mb-4",
            "As soon as SW3 realizes its Gi0/1 interface has failed, the process shown in the figure takes
            very little time."
            br {}
            "None of the processes rely on timers, so as soon as the work can be done,
            the convergence completes."
            br {}
            "(This particular convergence example takes about 1 second in a lab.)"
        }

        {h3_heading("RECAP")}
        ol { class: "list-disc list-inside",
            li {
                "To be an alternate port, both the RP and the alternate port must receive Hellos that identify the same root switch."
            }
            li { "An alternate port basically works like the second-best option for the root port." }
            li {
                "The alternate port can take over for the former root port, often very rapidly, 
                 without waiting on any timers, and without requiring a wait in other  interim RSTP states."
            }
        }
    }
}