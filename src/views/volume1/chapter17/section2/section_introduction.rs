use dioxus::prelude::*;

#[component]
pub fn SectionIntroductionContent() -> Element {
    rsx! {
        p { class: "mb-4",
            "Using a router with ROAS to route packets makes sense in some cases, particularly at small remote sites."
            br {}
            strong {
                "In sites with a larger LAN, network designers choose to use Layer 3 switches for most inter-VLAN routing."
            }
        }

        p { class: "mb-1",
            "A Layer 3 switch (also called a multilayer switch) is one device, but it executes logic at two
            layers:"
        }
        ul { class: "list-disc list-inside mb-1",
            li { "Layer 2 LAN switching" }
            li { "and Layer 3 IP routing." }
        }
        p { class: "mb-4",
            "The Layer 2 switch function forwards frames inside each VLAN, but it will not forward frames between VLANs."
            br {}
            strong { "The Layer 3 forwarding (routing) logic forwards IP packets between VLANs." }
        }

        p { class: "mb-4",
            "Layer 3 switches typically support two configuration options to enable IPv4 routing inside the
            switch, specifically to enable IPv4 on switch interfaces."
            br {}
            "This section explains one option, an option that uses switched virtual interfaces (SVI)."
            br {}
            "The final major section of the chapter deals with the other option for configuring IPv4 addresses on 
            Layer 3 switches: routed interfaces."
        }

    }
}