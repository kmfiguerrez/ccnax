use dioxus::prelude::*;

use crate::utils::{TextCommandColor, h3_heading, text_command};

#[component]
pub fn Content() -> Element {
    rsx! {
        p { class: "mb-4",
            "Trunking can be configured correctly so that both switches use trunking."
            br {}
            "However, trunks can also be misconfigured, with a couple of different results: either both switches do not
            trunk, or one switch trunks and the other does not. Both results cause problems."
        }

        p { class: "mb-4",
            "The most common incorrect configuration—which results in both switches not trunking—is
            a configuration that uses the "
            {text_command("switchport mode dynamic auto", TextCommandColor::Gold)}
            " command on both switches on
            the link."
            br {}
            strong {
                "The word auto just makes us all want to think that the link would trunk automatically, but this command is 
            both automatic and passive."
            }
            br {}
            strong {
                "As a result, both switches passively wait on the other device on the link to begin negotiations."
            }
            br {}
            "Example 8-12 highlights those parts of the output from the "
            {text_command("show interfaces switchport", TextCommandColor::Gold)}
            " command that confirm both the
            configured and operational states."
            br {}
            "Note that the output lists the operational mode as “static access” rather than “trunking.” "
        }

        img {
            class: "mb-4 rounded-lg",
            alt: "Example 8-12 Operational Trunking State",
            loading: "lazy",
            src: asset!("/assets/static/v1p3c8s3sh2ex8-12.png", AssetOptions::image().with_avif()),
        }

        {h3_heading("Incorrect Trunking Configurations")}
        p { class: "mb-4",
            "A different incorrect trunking configuration has an even worse result: one switch trunks,
            sending tagged frames, while the neighboring switch does not trunk, so the neighboring
            switch discards any frames it receives that have a VLAN tag in the header."
            br {}
            "When this combination of events happens, the interface works in that the status on each end will be up/up or
            connected."
            br {}
            "Traffic in the native VLAN will actually cross the link successfully because those
            frames have no VLAN tags (headers)."
            br {}
            "However, traffic in all the rest of the VLANs will not cross the link."
        }

        p { class: "mb-4",
            "Figure 8-14 shows the incorrect configuration along with which side trunks and which does not."
            br {}
            "The side that trunks (SW1 in this case) enables trunking using the command "
            {text_command("switchport mode trunk", TextCommandColor::Gold)}
            " but also disables Dynamic Trunking Protocol (DTP) negotiations using the "
            {text_command("switchport nonegotiate ", TextCommandColor::Gold)}
            " command."
            br {}
            "SW2's configuration also helps create the problem, by using one of the two trunking options that relies on DTP."
            br {}
            "Because SW1 has disabled DTP, SW2's DTP negotiations fail, and SW2 chooses to not trunk."
        }

        img {
            class: "mb-4 rounded-lg",
            alt: "Example 8-12 Operational Trunking State",
            loading: "lazy",
            src: asset!("/assets/static/v1p3c8s3sh2f8-14.png", AssetOptions::image().with_avif()),
        }

        p { class: "mb-4",
            "The figure shows what happens when using this incorrect configuration."
            br {}
            "At Step 1, SW1 could (for example) forward a frame in VLAN 10."
            br {}
            "However, SW2 would view any frame that arrives with an 802.1Q header as illegal because the frame has an 802.1Q header, 
            and SW2 treats its G0/2 port as an access port."
            br {}
            "So, SW2 discards any 802.1Q frames received on that port."
        }

        {h3_heading("Preventing Trunking mismatch configurations")}
        p { class: "mb-4",
            "The trunking issues shown here can be easily avoided by checking the configuration and by
            checking the trunk's operational state (mode) on both sides of the trunk."
            br {}
            "The best commands to check trunking-related facts are "
            {text_command("show interfaces trunk", TextCommandColor::Gold)}
            " and "
            {text_command("show interfaces switchport", TextCommandColor::Gold)}
            "."
            br {}
            "Just be aware that the switches do not prevent you from making these configuration mistakes."
        }

        {h3_heading("RECAP")}
        ol { class: "list-disc list-inside",
            li {
                "The "
                {text_command("switchport mode dynamic auto", TextCommandColor::Gold)}
                " makes a switch  passively wait on the other device on the link to begin negotiations."
            }
            li {
                "A swtich trunk interface sends tagged frames, while a static access interface discards them."
            
            }
            li {
                " Watch out for mismatch trunking configurations on both ends because the status on both interfaces will be 
                listed as up/up or connected, but traffic in all the of the VLANs (except the native VLAN) will not cross the link."
            }
            li {
                "The best commands to check trunking-related facts are "
                {text_command("show interfaces trunk", TextCommandColor::Gold)}
                " and "
                {text_command("show interfaces switchport", TextCommandColor::Gold)}
                "."
            }
        }

    }
}