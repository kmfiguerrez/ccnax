use dioxus::prelude::*;

use crate::{
    components::{ConfigChecklist, GreenNote}, utils::{TextCommandColor, h3_heading, text_command}
};

#[component]
pub fn Content() -> Element {
    rsx! {
        p { class: "mb-4",
            "Cisco switches support two other login security methods that both use per-user username/
            password pairs instead of a shared password with no username."
            br {}
            "One method, referred to as local usernames and passwords, configures the username/password pairs locally—that is,
            in the switch's configuration."
            br {}
            "Switches support this local username/password option for the console, for Telnet, and even for SSH, but do not 
            replace the enable password used to reach enable mode."
        }

        p { class: " mb-4",
            "The configuration to migrate from using the simple shared passwords to instead using local
            usernames/passwords requires only some small configuration changes, as shown in Figure 6-3."
        }

        img {
            class: "mb-4 rounded-lg",
            alt: "Figure 6-3 Local Usernames and Passwords",
            loading: "lazy",
            src: asset!("/assets/static/v1p2c6s1sh2f6-3.png", AssetOptions::image().with_avif()),
        }

        p { class: " mb-4",
            "Working through the configuration in the figure, first, the switch of course needs to know
            the list of username/password pairs."
            br {}
            "To create these, repeatedly use the "
            {text_command("username", TextCommandColor::Gold)}
            i { " name " }
            {text_command("secret", TextCommandColor::Gold)}
            i { " password" }
            " global configuration command."
            br {}
            "Then, to enable this different type of console or Telnet security, simply enable this login security method 
            with the "
            {text_command("login local", TextCommandColor::Gold)}
            " line."
            br {}
            "Basically, this command means “use the local list of usernames for login.”"
            br {}
            "You can also use the "
            {text_command("no password", TextCommandColor::Gold)}
            " command (without even typing in the password) to clean up any remaining
            password subcommands from console or vty mode because these commands are not needed
            when using local usernames and passwords."
        }

        {h3_heading("Configuration Checklist")}
        p {
            "The following checklist details the commands to configure local username login, mainly as a
            method for easier study and review:"
        }
        ConfigChecklist {}
        ol { class: "mb-4",
            // Step 1
            li {
                span { class: "text-sky-500 font-semibold mr-4", "Step 1." }
                "Use the "
                {text_command("username", TextCommandColor::Gold)}
                i { " name " }
                {text_command("secret", TextCommandColor::Gold)}
                i { " password" }
                " global configuration command to
                add one or more username/password pairs on the local switch."
            }
            // Step 2
            li { class: "flex flex-col md:flex-row md:gap-x-4",
                span { class: "text-sky-500 font-semibold shrink-0", "Step 2." }
                div {
                    span { "Configure the console to use locally configured username/password pairs:" }
                    ol {
                        // Step A
                        li {
                            span { class: "text-sky-500 font-semibold uppercase mr-2",
                                "a."
                            }
                            "Use the "
                            {text_command("line con 0", TextCommandColor::Gold)}
                            " command to enter console configuration mode."
                        }
                        // Step B
                        li {
                            span { class: "text-sky-500 font-semibold uppercase mr-2",
                                "b."
                            }
                            "Use the "
                            {text_command("login local", TextCommandColor::Gold)}
                            " subcommand to enable the console to prompt for both username and password, checked versus the list 
                            of local usernames/passwords."
                        }
                        // Step C
                        li {
                            span { class: "text-sky-500 font-semibold uppercase mr-2",
                                "c."
                            }
                            "(Optional) Use the "
                            {text_command("no password", TextCommandColor::Gold)}
                            " subcommand to remove any existing simple shared passwords, just for good housekeeping of the 
                            configuration file."
                        }
                    }
                }
            
            }
            // Step 3
            li { class: "flex flex-col md:flex-row md:gap-x-4",
                span { class: "text-sky-500 font-semibold shrink-0", "Step 3." }
                div {
                    span { "Configure Telnet (vty) to use locally configured username/password pairs." }
                    ol {
                        // Step A
                        li {
                            span { class: "text-sky-500 font-semibold uppercase mr-2",
                                "a."
                            }
                            "Use the "
                            {text_command("line vty 0 15", TextCommandColor::Gold)}
                            " command to enter vty configuration mode for all 16 vty lines (numbered 0 through 15)."
                        }
                        // Step B
                        li {
                            span { class: "text-sky-500 font-semibold uppercase mr-2",
                                "b."
                            }
                            "Use the "
                            {text_command("login local", TextCommandColor::Gold)}
                            " subcommand to enable the switch to prompt for both
                            username and password for all inbound Telnet users, checked versus the list
                            of local usernames/passwords."
                        }
                        // Step C
                        li {
                            span { class: "text-sky-500 font-semibold uppercase mr-2",
                                "c."
                            }
                            "(Optional) Use the "
                            {text_command("no password", TextCommandColor::Gold)}
                            " subcommand to remove any existing simple shared passwords, just for good housekeeping of the 
                            configuration file."
                        }
                    }
                }
            
            }
        }

        p { class: "mb-4",
            "When a Telnet user connects to the switch configured as shown in Figure 6-3, the user will
            be prompted first for a username and then for a password, as shown in Example 6-4."
            br {}
            "The username/password pair must be from the list of local usernames; otherwise, the login is
            rejected. "
        }

        img {
            class: "mb-4 rounded-lg",
            alt: "Example 6-4 Telnet Login Process After Applying Configuration in Figure 6-3",
            loading: "lazy",
            src: asset!("/assets/static/v1p2c6s1sh2ex6-4.png", AssetOptions::image().with_avif()),
        }

        GreenNote {
            strong { "NOTE" }
            " Example 6-4 does not show the password value as having been typed because Cisco
            switches do not display the typed password for security reasons."
        }

        p { class: "mb-4",
            "The end of Example 6-4 points out one of the many security improvements when requiring
            each user to log in with their own username."
            br {}
            "The end of the example shows the user entering configuration mode (configure terminal) and then immediately leaving (end). Note that
            when a user exits configuration mode, the switch generates a log message."
            br {}
            "If the user logged in with a username, the log message identifies that username; note the “wendell” in the log
            message."
        }

        {h3_heading("RECAP")}
        ol { class: "list-disc list-inside",
            li { "Cisco also uses local username/passwords for securing user mode access." }
        }
    }
}