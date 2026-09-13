use dioxus::prelude::*;

use crate::{
    components::{
        KeyTopic,
        RedNote
    }, 
    utils::{TextCommandColor, h3_heading, text_command}
};

#[component]
pub fn Content() -> Element {
    rsx! {
        p { class: "mb-4",
            "Trunking configuration between two Cisco switches can be very simple if you just statically
            configure trunking."
            br {}
            "For example, most Cisco Catalyst switches today support only 802.1Q
            and not ISL."
            br {}
            "You could literally add one interface subcommand for the switch interface on
            each side of the link ("
            {text_command("switchport mode trunk", TextCommandColor::Gold)}
            "), and you would create a VLAN trunk that
            supported all the VLANs known to each switch."
        }

        p {
            "However, trunking configuration on Cisco switches includes many more options, including
            several options for dynamically negotiating various trunking settings."
            br {}
            "The configuration can either predefine different settings or tell the switch to negotiate the settings, as follows:"
        }
        ul { class: "list-disc list-inside mb-4",
            li {
                span { class: "font-bold", "The type of trunking:" }
                " IEEE 802.1Q, ISL, or negotiate which one to use, on switches that
                support both types of trunking."
            }
            li {
                span { class: "font-bold", "The administrative mode:" }
                " Whether to always trunk, always not trunk, or negotiate whether to trunk or not."
            }
        }

        {h3_heading("Dynamic Trunking Protocol")}
        p { class: "mb-4",
            "First, consider the type of trunking."
            br {}
            "Cisco switches that support ISL and 802.1Q can negotiate which type to use, using the Dynamic Trunking Protocol (DTP)."
            br {}
            "If both switches support both protocols, they use ISL; otherwise, they use the protocol that both support."
            br {}
            "Today, many Cisco switches do not support the older ISL trunking protocol."
            br {}
            "Switches that support both types of trunking use the "
            {text_command("switchport trunk encapsulation", TextCommandColor::Gold)}
            " {{"
            {text_command("dot1q", TextCommandColor::Gold)}
            " | "
            {text_command("isl", TextCommandColor::Gold)}
            " | "
            {text_command("negotiate", TextCommandColor::Gold)}
            "}}"
            " interface subcommand to either configure the type or allow DTP to negotiate the type."
        }

        p { class: "mb-4",
            "DTP can also negotiate whether the two devices on the link agree to trunk at all, as guided
            by the local switch port's administrative mode."
            br {}
            "The "
            strong { "administrative mode" }
            " refers to the configuration setting for whether trunking should be used."
            br {}
            "Each interface also has an "
            strong { "operational mode" }
            ", which refers to what is currently happening on the interface and 
            might have been chosen by DTP's negotiation with the other device."
            br {}
            "Cisco switches use the "
            {text_command("switchport mode", TextCommandColor::Gold)}
            " interface subcommand to define the administrative trunking mode, as 
            listed in Table 8-2."
        }

        KeyTopic {}
        img {
            class: "mb-4 rounded-lg",
            alt: "Table 8-2 Trunking Administrative Mode Options with the switchport mode Command",
            loading: "lazy",
            src: asset!("/assets/static/v1p3c8s2sh3t8-2.png", AssetOptions::image().with_avif()),
        }

        p { class: "mb-4",
            "For example, consider the two switches shown in Figure 8-10."
            br {}
            "The two switches use a Gigabit Ethernet link for the trunk."
            br {}
            strong {
                "In this case, the trunk does not dynamically form by default because both (2960) switches
            default to an administrative mode of dynamic auto, meaning that neither switch initiates the
            trunk negotiation process."
            }
            br {}
            "When one switch is changed to use dynamic desirable mode,
            which does initiate the negotiation, the switches negotiate to use trunking, specifically
            802.1Q because the 2960s support only 802.1Q."
        }

        img {
            class: "mb-4 rounded-lg",
            alt: "Table 8-2 Trunking Administrative Mode Options with the switchport mode Command",
            loading: "lazy",
            src: asset!("/assets/static/v1p3c8s2sh3f8-10.png", AssetOptions::image().with_avif()),
        }

        p { class: "mb-4",
            "Example 8-5 begins with SW1 configuration, SW1
            has two ports each assigned to VLANs 1, 2, and 3."
            br {}
            "However, both SW1 and SW2 currently have all default settings on the interfaces that connect the two switches."
            br {}
            "With the default setting of switchport mode dynamic auto, the two switches do not trunk."
        }

        p { class: "mb-4", "See Example 8-5 in volume 1 on page 192." }

        {h3_heading("The output of show interfaces switport command")}
        p {
            "First, focus on the highlighted items from the output of the "
            {text_command("show interfaces switchport", TextCommandColor::Gold)}
            " command at the beginning of Example 8-5."
        }
        ol { class: "list-disc list-inside mb-4",
            li {
                span { class: "font-bold", "First shaded line (Administrative mode):" }
                " The output lists the default administrative mode setting of dynamic auto."
            }
            li {
                span { class: "font-bold", "Second shaded line (Operational mode):" }
                " Because SW2 also defaults to dynamic auto, the command lists SW1's operational status as “access,” meaning that it is 
                not trunking. (“Dynamic auto” tells both switches to sit there and wait on the other switch to start the negotiations.)"
            }
            li {
                span { class: "font-bold", "Third shaded line (Administrative Trunking Encapsulation):" }
                " The third shaded line points out the only supported type of trunking (802.1Q). (On a switch that supports 
                both ISL and 802.1Q, this value would by default list “negotiate,” to mean that the type of encapsulation is negotiated.)"
            }
            li {
                span { class: "font-bold", "Fourth shaded line (Operational Trunking Encapsulation):" }
                " Finally, the operational trunking type is listed as “native,” which is a reference to the 802.1Q native VLAN."
                " When the operational mode is trunk, this value will be listed as “dot1q”."
            }
        }

        {h3_heading("The show interfaces trunk")}
        p { class: "mb-4",
            "The end example 8-5 shows the output of the "
            {text_command("show interfaces trunk", TextCommandColor::Gold)}
            " command, but with
            no output."
            br {}
            "This command lists information about all interfaces that currently operationally
            trunk; that is, it lists interfaces that currently use VLAN trunking."
            br {}
            "With no interfaces listed, this command also confirms that the link between switches is not trunking."
        }

        p { class: "mb-4",
            "See Example 8-6,8-7 in volume 1 on page 193 and subsequent pages."
            br {}
            "The examples show new configuration that enables trunking."
        }

        {h3_heading("Interpreting output of the show interfaces switchport")}
        p { class: "mb-4",
            strong { "For the exams, you should be ready to interpret the output of the " }
            {text_command("show interfaces switchport", TextCommandColor::Gold)}
            " command, realize the administrative mode implied by the output, and know
            whether the link should operationally trunk based on those settings."
            br {}
            "Table 8-3 lists the combinations of the trunking administrative modes and the expected operational mode
            (trunk or access) resulting from the configured settings."
            br {}
            "The table lists the administrative mode used on one end of the link on the left, and the administrative mode on the 
            switch on the other end of the link across the top of the table."
        }

        KeyTopic {}
        img {
            class: "mb-4 rounded-lg",
            alt: "Table 8-3 Expected Trunking Operational Mode Based on the Configured Administrative Modes",
            loading: "lazy",
            src: asset!("/assets/static/v1p3c8s2sh3t8-3.png", AssetOptions::image().with_avif()),
        }

        RedNote {
            p {
                strong { "NOTE" }
                " When two switches configure a mode of “access” on one end and “trunk” on the other, problems occur.
                Avoid this combination."
            }
        }

        {h3_heading("Disabling DTP")}
        p { class: "mb-4",
            "Finally, before leaving the discussion of configuring trunks, "
            strong { "Cisco recommends disabling trunk negotiation on most ports for better security." }
            br {}
            "The majority of switch ports on most switches will be used to connect to users and configured with the command "
            {text_command("switchport mode access", TextCommandColor::Gold)}
            "—which also disables DTP."
            br {}
            "For ports without the "
            {text_command("switchport mode access", TextCommandColor::Gold)}
            " command—for instance, ports statically configured to trunk with 
            the "
            {text_command("switchport mode trunk", TextCommandColor::Gold)}
            " command—DTP still operates, but you can disable DTP negotiations altogether using
            the "
            {text_command("switchport nonegotiate", TextCommandColor::Gold)}
            " interface subcommand."
        }

        {h3_heading("RECAP")}
        ol { class: "list-disc list-inside",
            li {
                "You can make a trunk link between switches with just one interface subcommand "
                {text_command("switchport mode trunk", TextCommandColor::Gold)}
                "."
            }
            li { "Most Cisco Catalyst switches today support only 802.1Q and not ISL." }
            li {
                "Cisco switches that support ISL and 802.1Q can negotiate which type to use, using the Dynamic Trunking Protocol (DTP)."
            }
            li {
                "If both switches support both ISL and 802.1Q protocols, they use ISL; otherwise, they use the protocol that both support."
            }
            li {
                "Switches that support both types of trunking ( ISL and 802.1Q) use the "
                {text_command("switchport trunk encapsulation", TextCommandColor::Gold)}
                " {{"
                {text_command("dot1q", TextCommandColor::Gold)}
                " | "
                {text_command("isl", TextCommandColor::Gold)}
                " | "
                {text_command("negotiate", TextCommandColor::Gold)}
                "}}"
                " interface subcommand to either configure the type or allow DTP to negotiate the type."
            }
            li {
                "The "
                strong { "administrative mode" }
                " refers to the configuration setting for whether trunking should be used."
            }
            li {
                "Cisco switches use the "
                {text_command("switchport mode", TextCommandColor::Gold)}
                " interface subcommand to define the administrative trunking mode."
            }
            li {
                "Each interface also has an "
                strong { "operational mode" }
                ", which refers to whether an interface acts as static access or trunk interface--manually 
                configured or dynamically negotiated."
            }
            li {
                "By default, Cisco switches interfaces default to administrative mode of dynamic auto, meaning they
                do not initiate the trunk negotiation process."
            }
            li {
                "The output of the "
                {text_command("show interfaces trunk", TextCommandColor::Gold)}
                " command  lists information about all interfaces that currently 
                operationally trunk; that is, it lists interfaces that currently use VLAN trunking."
            }
            li {
                "When an interface transitions from access to trunk mode (vice versa), a log message appears
                showing  that the interface goes down and then back up again."
            }
            li { "Cisco recommends disabling trunk negotiation on most ports for better security." }
            li {
                "The "
                {
                    text_command(
                        "switchport mode access/switchport nonegotiate",
                        TextCommandColor::Gold,
                    )
                }
                " interface subcommand disables DTP."
            }
            li {
                "The "
                {text_command("switchport mode trunk", TextCommandColor::Gold)}
                " doesn't disables DTP so make sure to pair it with the "
                {text_command("switchport nonegotiate", TextCommandColor::Gold)}
                " interface subcommand."
            }
        }
    }
}