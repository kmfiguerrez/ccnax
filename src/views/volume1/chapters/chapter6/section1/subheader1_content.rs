use dioxus::prelude::*;

use crate::{
    components::{green_div::GreenNote, config_checklist::ConfigChecklist, key_topic::KeyTopic}, 
    utils::{h3_heading, text_command, TextCommandColor}};

#[component]
pub fn Content() -> Element {
    rsx! {
        p { class: "mb-4",
            "By default, Cisco Catalyst switches allow full access from the console but no access via
            Telnet or SSH."
            br {}
            "Using default settings, a console user can move into user mode and then privileged mode with no passwords required; 
            however, default settings prevent remote users from accessing even user mode."
        }

        p { class: "mb-4",
            "The defaults work great for a brand new switch, but in production, you will want to secure
            access through the console as well as enable remote login via Telnet and/or SSH so you can
            sit at your desk and log in to all the switches in the LAN."
            br {}
            "Keep in mind, however, that you should not open the switch for just anyone to log in and change the configuration, 
            so some type of secure login should be used."
        }

        p { class: "mb-4",
            "Most people use a simple shared password for access to lab gear. This method uses a password only—with no 
            username—with one password for console users and a different password for Telnet users."
            br {}
            "Console users must supply the console password, as configured in console line configuration mode."
            br {}
            "Telnet users must supply the Telnet password, also called the vty password, so called because the configuration 
            sits in vty line configuration mode."
            br {}
            "Figure 6-1 summarizes these options for using shared passwords from the perspective of the
            user logging in to the switch"
        }

        img {
            class: "mb-4 rounded-lg",
            alt: "Figure 6-1 Simple Password Security Concepts",
            loading: "lazy",
            src: asset!("/assets/static/v1p2c6s1sh1f6-1.png", AssetOptions::image().with_avif()),
        }

        GreenNote {
            p {
                strong { "Note:" }
                " This section refers to several passwords as shared passwords."
                "Users share these passwords in that all users must know and use that same password."
                "In other words, each user does not have a unique username/password to use, but rather, all the appropriate 
                staff knows and uses the same password."
            }
        }

        {h3_heading("Protecting enable mode")}
        p { class: "mb-4",
            "In addition, Cisco switches protect enable mode (also called privileged mode) with yet another
            shared password called the "
            i { "enable password." }
            br {}
            "From the perspective of the network engineer connecting to the CLI of the switch, once in user mode, the user types 
            the "
            {text_command("enable", TextCommandColor::Gold)}
            " EXEC command."
            br {}
            "This command prompts the user for this enable password; if the user types the correct password, IOS moves the user 
            to enable mode"
        }

        p { class: "mb-4",
            "Example 6-1 shows an example of the user experience of logging in to a switch from the console when the shared 
            console password and the shared enable password have both been set."
            br {}
            "Note that before this example began, the user started the terminal emulator, physically connected a laptop to the 
            console cable, and then pressed the Return key to make the switch respond as shown at the top of the example."
        }

        p { class: "mb-4", "See Example 6-1 in volume 1 book page 130." }

        p { class: "mb-4",
            "Note that the example shows the password text as if typed (faith and love), along with the "
            {text_command("enable", TextCommandColor::Gold)}
            " command that moves the user from user mode to enable mode."
            br {}
            "In reality, the switch hides the passwords when typed, to prevent someone from reading over your shoulder 
            to see the passwords."
        }

        {h3_heading("Configuring passwords for console, Telnet and for enable mode")}
        p { class: "mb-4",
            "To configure the shared passwords for the console, Telnet, and for enable mode, you need to
            configure several commands."
            br {}
            "However, the parameters of the commands can be pretty intuitive."
            br {}
            "Figure 6-2 shows the configuration of all three of 
            these passwords."
        }

        p {
            "The configuration for these three passwords does not require a lot of work."
            br {}
            "First, the console and vty password configuration sets the password based on the context: console mode for the
            console ("
            span { class: "font-bold", "line con 0" }
            "), and vty line configuration mode for the Telnet password ("
            span { class: "font-bold", "line vty 0 15" }
            ")."
            br {}
            "Then inside console mode and vty mode, respectively, the two commands in each mode are as
            follows:"
        }
        ol { class: "list-disc list-inside mb-4",
            li {
                {text_command("password", TextCommandColor::Gold)}
                i { " password-value:" }
                " Defines the actual password used on the console or vty"
            }
            li {
                {text_command("login:", TextCommandColor::Gold)}
                " Tells IOS to enable the use of a simple shared password (with no username) on this
                line (console or vty), so that the switch asks the user for a password"
            }
        }

        img {
            class: "mb-4 rounded-lg",
            alt: "Figure 6-2 Simple Password Security Configuration",
            loading: "lazy",
            src: asset!("/assets/static/v1p2c6s1sh1f6-2.png", AssetOptions::image().with_avif()),
        }

        p { class: "mb-4",
            "The configured enable password, shown on the right side of the figure, applies to all users,
            no matter whether they connect to user mode via the console, Telnet, or otherwise."
            br {}
            "The command to configure the enable password is a global configuration command: "
            {text_command("enable secret", TextCommandColor::Gold)}
            i { " password-value" }
        }

        GreenNote {
            strong { "Note" }
            " Older IOS versions used the command "
            {text_command("enable password", TextCommandColor::Black)}
            i { " password-value" }
            " to set the enable password, and that command 
            still exists in IOS."
            "However, the "
            {text_command("enable secret", TextCommandColor::Black)}
            " command is much more secure."
            "In real networks, use "
            {text_command("enable secret", TextCommandColor::Black)}
            " Chapter 5, “Securing Network Devices,” in the CCNA 200-301 Official Cert Guide, Volume 2, explains more about the
            security levels of various password mechanisms, including a comparison of the "
            {text_command("enable secret", TextCommandColor::Black)}
            " and "
            {text_command("enable password", TextCommandColor::Black)}
            " commands."
        }

        p { class: "mb-4",
            "To help you follow the process, and for easier study later, use the configuration checklist before the example."
            br {}
            "The configuration checklist collects the required and optional steps to configure a feature as described in this 
            book."
            br {}
            "The configuration checklist for shared passwords for the console, Telnet, and enable passwords in the global
            configuration mode is"
        }

        ConfigChecklist {}
        ol { class: "mb-4",
            li {
                span { class: "text-sky-500 font-semibold mr-4", "Step 1." }
                "Configure the enable password with the "
                {text_command("enable secret", TextCommandColor::Gold)}
                i { " password-value" }
                " command in the config mode."
            }
            li { class: "flex flex-col md:flex-row md:gap-x-4",
                span { class: "text-sky-500 font-semibold", "Step 2." }
                div {
                    span { "Configure the console password:" }
                    ol {
                        li {
                            span { class: "text-sky-500 font-semibold uppercase mr-2",
                                "a."
                            }
                            "Use the "
                            {text_command("line con 0", TextCommandColor::Gold)}
                            " command to enter console configuration mode."
                        }
                        li {
                            span { class: "text-sky-500 font-semibold uppercase mr-2",
                                "b."
                            }
                            "Use the "
                            {text_command("password", TextCommandColor::Gold)}
                            i { " password-value" }
                            " subcommand to set the value of the console password."
                        }
                        li {
                            span { class: "text-sky-500 font-semibold uppercase mr-2",
                                "c."
                            }
                            "Use the "
                            {text_command("login", TextCommandColor::Gold)}
                            i { " password-value" }
                            " subcommand to enable console password security using a simple password."
                        }
                    }
                }
            
            }
            li { class: "flex flex-col md:flex-row md:gap-x-4",
                span { class: "text-sky-500 font-semibold", "Step 3." }
                div {
                    span { "Configure the Telnet (vty) password:" }
                    ol {
                        li {
                            span { class: "text-sky-500 font-semibold uppercase mr-2",
                                "a."
                            }
                            "Use the "
                            {text_command("line vty 0 15", TextCommandColor::Gold)}
                            " command to enter vty configuration mode for all 16 vty lines (numbered 0 through 15)."
                        }
                        li {
                            span { class: "text-sky-500 font-semibold uppercase mr-2",
                                "b."
                            }
                            "Use the "
                            {text_command("password", TextCommandColor::Gold)}
                            i { " password-value" }
                            " subcommand to set the value of the vty password."
                        }
                        li {
                            span { class: "text-sky-500 font-semibold uppercase mr-2",
                                "c."
                            }
                            "Use the "
                            {text_command("login", TextCommandColor::Gold)}
                            i { " password-value" }
                            " subcommand to enable vty password security using a simple password."
                        }
                    }
                }
            
            }
        }

        p { class: "mb-4",
            "Example 6-2 shows the configuration process as noted in the configuration checklist, along with setting the enable 
            secret password."
            br {}
            "Note that the lines which begin with a ! are comment lines; they are there to guide you through the configuration"
        }

        p { class: "mb-4", "See Example 6-2 in volume 1 book on page 132." }

        p { class: "mb-4",
            "Example 6-3 shows the resulting configuration in the switch per the "
            {text_command("show running-config", TextCommandColor::Gold)}
            " command."
            br {}
            "The gray lines highlight the new configuration."
            br {}
            "Note that many unrelated lines of
            output have been deleted from the output to keep focused on the password configuration."
        }

        p { class: "mb-4", "See Example 6-3 in volume 1 book on page 133." }

        GreenNote {
            p {
                strong { "Note" }
                " For historical reasons, the output of the "
                {text_command("show running-config", TextCommandColor::Black)}
                " command, in the
                last six lines of Example 6-3, separates the first five vty lines (0 through 4) from the rest (5
                through 15)."
            }
        }

        {h3_heading("REMEMBER")}
        ol { class: "list-disc list-inside",
            li {
                "By default, Cisco Catalyst switches allow full access from the console but no access via Telnet or SSH."
            }
            li {
                "In production, you will want to secure access through the console as well as enable remote login via Telnet and/or SSH"
            }
        }
    }
}


