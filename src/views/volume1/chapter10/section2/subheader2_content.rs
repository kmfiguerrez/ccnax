use dioxus::prelude::*;

use crate::{
    components::GreenNote, utils::{h3_heading, text_command, TextCommandColor}
};

#[component]
pub fn Content() -> Element {
    rsx! {
        p { class: "mb-4",
            strong {
                "In addition to manual configuration, Cisco switches also support two different configuration
                options that then use a dynamic protocol to negotiate whether a particular link becomes
                part of an EtherChannel or not."
            }
            br {}
            "Basically, the configuration enables a protocol for a particular channel-group number."
            br {}
            "At that point, the switch can use the protocol to send messages to/from the neighboring switch and discover 
            whether their configuration settings pass all checks."
            br {}
            strong {
                "If a given physical link passes, the link is added to the EtherChannel and used; if
                not, it is placed in a down state, and not used, until the configuration inconsistency can be
                resolved."
            }
        }

        {h3_heading("Cisco and IEEE Dynamic EtherChannel protocols")}
        p { class: "mb-4",
            "Most Cisco Catalyst switches support the Cisco-proprietary Port Aggregation Protocol
            (PAgP) and the IEEE standard Link Aggregation Control Protocol (LACP), based on IEEE
            standard 802.3ad."
            br {}
            strong {
                "Although differences exist between the two, to the depth discussed here, they both accomplish the same 
                task: negotiate so that only links that pass the configuration checks are actually used in an EtherChannel."
            }
        }

        {h3_heading("LACP supports more links")}
        p { class: "mb-4",
            "One difference of note is that LACP does support more links in a channel—16—as compared to PaGP's maximum of 8."
            br {}
            strong {
                "With LACP, only 8 can be active at one time, with the others waiting to be used should any of the other links fail."
            }
        }

        {h3_heading("Configuring both protocols")}
        p { class: "mb-4",
            "To configure either protocol, a switch uses the "
            {text_command("channel-group", TextCommandColor::Gold)}
            " interface configuration subcommands on
            each switch, but with a keyword that either means “use this protocol and begin negotiations” or “use this protocol and 
            wait for the other switch to begin negotiations.”"
            br {}
            "As shown in Figure 10-7, the "
            {text_command("desirable", TextCommandColor::Gold)}
            " and "
            {text_command("auto", TextCommandColor::Gold)}
            " keywords enable PAgP, and the "
            {text_command("active", TextCommandColor::Gold)}
            " and "
            {text_command("passive", TextCommandColor::Gold)}
            " keywords enable LACP."
            br {}
            "With these options, at least one side has to begin the negotiations."
            br {}
            "In other words, with PAgP, at least one of the two sides must use "
            {text_command("desirable", TextCommandColor::Gold)}
            ", and with LACP, at least one of the two sides must use "
            {text_command("active", TextCommandColor::Gold)}
            "."
        }

        img {
            class: "mb-4 rounded-lg",
            alt: "Figure 10-7 Correct EtherChannel Configuration Combinations",
            loading: "lazy",
            src: asset!("/assets/static/v1p3c10s2sh2f10-7.png", AssetOptions::image().with_avif()),
        }

        GreenNote {
            p {
                strong { "NOTE" }
                " Do not use the "
                {text_command("on", TextCommandColor::Black)}
                " parameter on one end, and either "
                {text_command("auto", TextCommandColor::Black)}
                " or "
                {text_command("desirable", TextCommandColor::Black)}
                " (or for LACP, active or passive) on the neighboring switch."
                " The "
                {text_command("on", TextCommandColor::Black)}
                " option uses neither PAgP nor LACP, so a configuration that uses "
                {text_command("on", TextCommandColor::Black)}
                ", with PAgP or LACP options on the 
                other end, would prevent the EtherChannel from working."
            }
        }

        p { class: "mb-4",
            "For example, in the design shown in Figure 10-7, imagine both physical interfaces on both
            switches were configured with the "
            {text_command("channel-group 2 mode desirable", TextCommandColor::Gold)}
            " interface subcommand."
            br {}
            "As a result, the two switches would negotiate and create an EtherChannel."
            br {}
            "Example 10-5 shows the verification of that configuration, with the command "
            {text_command("show etherchannel 1 port-channel", TextCommandColor::Gold)}
            "."
            br {}
            "This command confirms the protocol in use (PAgP, because the "
            {text_command("desirable", TextCommandColor::Gold)}
            " keyword was configured), and the list of 
            interfaces in the channel."
        }

        p { class: "mb-4", "See Example 10-5 in volume 1 on page 251." }

        {h3_heading("RECAP")}
        ol { class: "list-disc list-inside",
            li {
                "Most Cisco Catalyst switches support the Cisco-proprietary Port Aggregation Protocol (PAgP) and 
                the IEEE standard Link Aggregation Control Protocol (LACP), based on IEEE standard 802.3ad."
            }
            li {
                "Cisco's and IEEE's protocols accomplish the same task: negotiate so that only links that pass the configuration checks 
                are actually used in an EtherChannel."
            }
            li {
                "LACP supports more links up to 16 but only 8 can be active at one time, with the others waiting to be used should 
                any of the other links fail."
            }
            li {
                "To configure either protocol, a switch uses the "
                {text_command("channel-group", TextCommandColor::Gold)}
                " interface configuration subcommands on
                each switch, but with a keyword that either means “use this protocol and begin negotiations” or “use this protocol 
                and wait for the other switch to begin negotiations.”"
            }
            li {
                "The "
                {text_command("desirable", TextCommandColor::Gold)}
                " and "
                {text_command("auto", TextCommandColor::Gold)}
                " keywords enable PAgP, and the "
                {text_command("active", TextCommandColor::Gold)}
                " and "
                {text_command("passive", TextCommandColor::Gold)}
                " keywords enable LACP."
                " With these options, at least one side has to begin the negotiations."
            }
            li {
                "With PAgP, at least one of the two sides must use "
                {text_command("desirable", TextCommandColor::Gold)}
                ", and with LACP, at least one of the two sides must use "
                {text_command("active", TextCommandColor::Gold)}
                "."
            }
            li {
                "If using either of the dynamic EtherChannel protocols, make sure to avoid using the "
                {text_command("on", TextCommandColor::Gold)}
                " keyword on one side, because it neither use PagP nor LACP, and would to result
                to EtherChannel not working."
            }
        }
    }
}