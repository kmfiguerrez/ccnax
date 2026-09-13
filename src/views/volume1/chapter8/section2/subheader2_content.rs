use dioxus::prelude::*;

use crate::{
    components::{
        GreenNote,
        RedNote
    }, 
    utils::{TextCommandColor, h3_heading, text_command}
};

#[component]
pub fn Content() -> Element {
    rsx! {
        p { class: "mb-4",
            "Before showing more configuration examples, you also need to know something about a
            Cisco protocol and tool called the VLAN Trunking Protocol (VTP)."
            br {}
            "VTP is a Cisco proprietary  tool on Cisco switches that advertises each VLAN configured in one switch 
            (with the "
            {text_command("vlan", TextCommandColor::Gold)}
            i { " number" }
            " command) so that all the other switches in the campus learn about that VLAN."
        }

        {h3_heading("Enterprises disables VTP")}
        p { class: "mb-4",
            "This book does not discuss VTP as an end to itself for a few different reasons."
            br {}
            "First, the current CCNA 200-301 exam blueprint ignores VTP, as do the CCNP Enterprise Core and CCNP
            Enterprise Advanced Routing blueprints."
            br {}
            "Additionally, many enterprises choose to disable VTP. "
        }

        p { class: "mb-4",
            "However, VTP has some small impact on how every Cisco Catalyst switch works, even if you
            do not try to use VTP."
            br {}
            "This brief section introduces enough details of VTP so that you can
            see these small differences in VTP that cannot be avoided."
        }

        {h3_heading("The vtp mode transparent/off commands")}
        p { class: "mb-4",
            "First, all examples in this book (and in Volume 2) use switches that disable VTP in some way."
            br {}
            "Interestingly, for much of VTP's decades of existence, most switches did not allow VTP to
            be disabled completely; on those switches, to effectively disable VTP, the engineer would set
            the switch to use VTP transparent mode (with the "
            {text_command("vtp mode transparent", TextCommandColor::Gold)}
            " global command)."
            br {}
            "Some switches now have an option to disable VTP completely with the "
            {text_command("vtp mode off", TextCommandColor::Gold)}
            " global command."
        }

        p { class: "mb-4",
            "Note that both transparent and off modes prevent VTP from learning and advertising about
            VLAN configuration."
            br {}
            "Those modes allow a switch to configure all VLANs, including standard- and extended-range VLANs."
            br {}
            "Additionally, switches using transparent or off modes list the vlan configuration commands in the running-config file"
        }

        RedNote {
            p {
                strong { "NOTE" }
                " In Packet Tracer version 8.2.2.0400."
                br {}
                " The "
                {text_command("vtp mode off", TextCommandColor::Black)}
                " command is not availabe on switches with IOS version C2960-LANBASEK9-M."
            }
        }

        {h3_heading("The show vtp status command")}
        p {
            "Finally, on a practical note, if you happen to do lab exercises with real switches or with simulators, and you see 
            unusual results with VLANs, check the VTP status with the "
            {text_command("show vtp status", TextCommandColor::Gold)}
            " command."
            br {}
            "If your switch uses VTP server or client mode, you will find"
        }
        ol { class: "list-disc list-inside mb-4",
            li { "The server switches can configure VLANs in the standard range only (1-1005)." }
            li { "The client switches cannot configure VLANs." }
            li {
                "Both servers and clients may be learning new VLANs from other switches and seeing
                their VLANs deleted by other switches because of VTP."
            }
            li {
                "The "
                {text_command("show running-config", TextCommandColor::Gold)}
                " command does not list any "
                {text_command("vlan", TextCommandColor::Gold)}
                " commands; you must use
                other "
                {text_command("show", TextCommandColor::Gold)}
                " commands to find out about the configured VLANs."
            }
        }

        GreenNote {
            p {
                strong { "NOTE" }
                " Do not change VTP settings on any switch that also connects to the production
                network until you know how VTP works and you talk with experienced colleagues."
                " Doing so can cause real harm to your LAN."
                " For example, if the switch you configure connects to other switches, which in turn connect to switches used in 
                the production LAN, you could accidentally change the VLAN configuration in other switches with serious impact 
                to the operation of the network."
                " You could delete VLANs and cause outages."
                " Be careful and never experiment with VTP settings on a switch unless it and the other switches connected to it
                have absolutely no physical links connected to the production LAN."
            }
        }

        {h3_heading("RECAP")}
        ol { class: "list-disc list-inside",
            li {
                "VTP is a Cisco proprietary  tool on Cisco switches that advertises each VLAN configured in one switch 
                (with the "
                {text_command("vlan", TextCommandColor::Gold)}
                i { " number" }
                " command) so that all the other switches in the campus learn about that VLAN."
            }
            li {
                "The "
                {text_command("vtp mode transparent/off", TextCommandColor::Gold)}
                " command disables VTP - prevent VTP from learning and advertising about
                VLAN configuration."
            }
            li {
                "If see unusual results with VLANs, check the VTP status with the "
                {text_command("show vtp status", TextCommandColor::Gold)}
                " command."
            }
        }
    }
}