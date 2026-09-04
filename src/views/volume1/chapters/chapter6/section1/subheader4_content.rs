use dioxus::prelude::*;

use crate::{components::{KeyTopic, ConfigChecklist, GreenNote}, utils::{TextCommandColor, h3_heading, text_command}};

#[component]
pub fn Content() -> Element {
    rsx! {
        p { class: "mb-4",
            "So far, this chapter has focused on the console and on Telnet, mostly ignoring SSH."
            br {}
            "Telnet has one serious disadvantage: all data in the Telnet session flows as clear text, including the
            password exchanges."
            br {}
            "So, anyone that can capture the messages between the user and the switch (in what is called a man-in-the-middle 
            attack) can see the passwords."
            br {}
            "SSH encrypts all data transmitted between the SSH client and server, protecting the data and passwords."
        }

        p { class: "mb-4",
            "SSH can use the same local login authentication method as Telnet, with the locally configured username and password."
            br {}
            "(SSH cannot rely on authentication methods that do not include a username, like shared passwords.)"
            br {}
            "So, the configuration to support local usernames for Telnet, as shown previously in Figure 6-3, also enables local 
            username authentication for incoming SSH connections"
        }

        {h3_heading("SSH Configuration")}
        p { class: "mb-4",
            "Figure 6-5 shows one example configuration of what is required to support SSH."
            br {}
            "The figure repeats the local username configuration as shown earlier in Figure 6-3, as used for Telnet."
            br {}
            "Figure 6-5 shows three additional commands required to complete the configuration of SSH
            on the switch."
        }

        KeyTopic {}
        img {
            class: "mb-4 rounded-lg",
            alt: "Figure 6-5 Adding SSH Configuration to Local Username Configuration",
            loading: "lazy",
            src: asset!("/assets/static/v1p2c6s1sh4f6-5.png", AssetOptions::image().with_avif()),
        }

        p { class: "mb-4",
            strong {
                "IOS uses the three SSH-specific configuration commands in the figure to create the SSH
            encryption keys."
            }
            br {}
            "The SSH server uses the fully qualified domain name (FQDN) of the switch as input to create that key."
            br {}
            "The switch creates the FQDN from the hostname and domain name of the switch."
            br {}
            "Figure 6-5 begins by setting both values (just in case they are not
            already configured)."
            br {}
            "Then the third command, the crypto key generate rsa command, generates the SSH encryption keys."
        }

        p { class: "mb-4",
            "The configuration in Figure 6-5 relies on two default settings that the figure therefore conveniently ignored."
            br {}
            strong { "IOS runs an SSH server by default." }
            br {}
            "In addition, IOS allows SSH connections into the vty lines by default."
        }

        p { class: "mb-4",
            "Seeing the configuration happen in configuration mode, step by step, can be particularly
            helpful with SSH configuration."
            br {}
            "Note in particular that in this example, the "
            {text_command("crypto key", TextCommandColor::Gold)}
            " command prompts the user for the key modulus; you could also 
            add the parameters "
            {text_command("modulus", TextCommandColor::Gold)}
            i { " modulus-value" }
            " to the end of the "
            {text_command("crypto key", TextCommandColor::Gold)}
            " command to add this setting on the command."
            br {}
            "Example 6-5 shows the commands in Figure 6-5 being configured, with the encryption key
            as the final step."
        }

        p { class: "mb-4",
            "See Example 6-5: SSH Configuration Process to Match Figure 6-5, in volume 1 book on page 137"
        }

        {h3_heading("Telnet should be disabled")}
        p { class: "mb-4",
            "Earlier, I mentioned that one useful default was that the switch defaults to support both SSH
            and Telnet on the vty lines."
            br {}
            "However, because Telnet is a security risk, you could disable Telnet to enforce a tighter security policy."
            br {}
            "(For that matter, you can disable SSH support and
            allow Telnet on the vty lines as well.)"
        }

        {h3_heading("Configuring VTY protocols")}
        p {
            "To control which protocols a switch supports on its vty lines, use the "
            {
                text_command(
                    "transport input {{all | none | telnet | ssh}}",
                    TextCommandColor::Gold,
                )
            }
            " vty subcommand in vty mode, with the following options:"
        }
        ul { class: "list-disc list-inside mb-4",
            li {
                {text_command("transport input all", TextCommandColor::Gold)}
                " or "
                {text_command("transport input telnet ssh:", TextCommandColor::Gold)}
                " Support both Telnet and SSH"
            }
            li {
                {text_command("transport input none:", TextCommandColor::Gold)}
                " Support neither"
            }
            li {
                {text_command("transport input telnet:", TextCommandColor::Gold)}
                " Support only Telnet"
            }
            li {
                {text_command("transport input ssh:", TextCommandColor::Gold)}
                " Support only SSH"
            }
        }

        {h3_heading("Configuration Checklist")}
        p {
            "To complete this section about SSH, the following configuration checklist details the steps
            for one method to configure a Cisco switch to support SSH using local usernames."
            br {}
            "(SSH support in IOS can be configured in several ways; this checklist shows one simple way to
            configure it.)"
            br {}
            "The process shown here ends with a comment to configure local username support on vty lines, as was discussed earlier 
            in the section titled “Securing User Mode Access with Local Usernames and Passwords.”"
        }
        ConfigChecklist {}
        ol { class: "mb-4",
            // Step 1
            li { class: "flex flex-col md:flex-row md:gap-x-4",
                span { class: "text-sky-500 font-semibold shrink-0", "Step 1." }
                div {
                    span {
                        "Configure the switch to generate a matched public and private key pair to use
                    for encryption:"
                    }
                    ol {
                        // Step A
                        li {
                            span { class: "text-sky-500 font-semibold uppercase mr-2",
                                "a."
                            }
                            "If not already configured, use the "
                            {text_command("hostname ", TextCommandColor::Gold)}
                            i { "name" }
                            " in global configuration mode to configure a hostname for this switch."
                        }
                        // Step B
                        li {
                            span { class: "text-sky-500 font-semibold uppercase mr-2",
                                "b."
                            }
                            "If not already configured, use the "
                            {text_command(" ip domain-name", TextCommandColor::Gold)}
                            i { " name" }
                            " in global configuration mode to configure a domain name for the switch, completing the
                            switch's FQDN."
                        }
                        // Step C
                        li {
                            span { class: "text-sky-500 font-semibold uppercase mr-2",
                                "c."
                            }
                            "Use the "
                            {text_command("crypto key generate rsa", TextCommandColor::Gold)}
                            " command in global configuration mode (or the "
                            {text_command("crypto key generate rsa modulus", TextCommandColor::Gold)}
                            i { " modulus-value" }
                            " command to avoid being prompted for the key modulus) to generate the keys. (Use at
                            least a 768-bit key to support SSH version 2.)"
                        }
                    }
                }
            
            }
            // Step 2
            li {
                span { class: "text-sky-500 font-semibold mr-4", "Step 2." }
                "(Optional) Use the "
                {text_command("ip ssh version 2", TextCommandColor::Gold)}
                " command in global configuration mode to override the default of supporting both versions 1 and 2, 
                so that only SSHv2 connections are allowed."
            }
            // Step 3
            li { class: "flex flex-col md:flex-row md:gap-x-4",
                span { class: "text-sky-500 font-semibold shrink-0", "Step 3." }
                div {
                    span {
                        "(Optional) If not already configured with the setting you want, configure the
                        vty lines to accept SSH and whether to also allow Telnet:"
                    }
                    ol {
                        // Step A
                        li {
                            span { class: "text-sky-500 font-semibold uppercase mr-2",
                                "a."
                            }
                            "Use the "
                            {text_command("transport input ssh", TextCommandColor::Gold)}
                            " command in vty line configuration mode to allow SSH only."
                        }
                        // Step B
                        li {
                            span { class: "text-sky-500 font-semibold uppercase mr-2",
                                "b."
                            }
                            "Use the "
                            {text_command("transport input all", TextCommandColor::Gold)}
                            " command (default) or "
                            {text_command("transport input telnet ssh", TextCommandColor::Gold)}
                            " command in vty line configuration mode to allow both SSH and Telnet."
                        }
                    }
                }
            
            }
            // Step 4
            li {
                span { class: "text-sky-500 font-semibold mr-4", "Step 4." }
                "Use various commands in vty line configuration mode to configure local username login authentication as discussed earlier 
                in this chapter."
            }
        }

        GreenNote {
            p {
                strong { "NOTE" }
                " Cisco routers often default to "
                {text_command("transport input none", TextCommandColor::Black)}
                ", so you must add the "
                {text_command("transport input", TextCommandColor::Black)}
                " line subcommand to enable Telnet and/or SSH into a router."
            }
        }

        {h3_heading("Displaying SSH Status")}
        p { "Two key commands give some information about the status of SSH on the switch." }
        ul { class: "list-disc list-inside",
            li {
                "First, the "
                {text_command("show ip ssh", TextCommandColor::Gold)}
                " command lists status information about the SSH server itself"
            }
            li {
                "The "
                {text_command("show ssh", TextCommandColor::Gold)}
                " command then lists information about each SSH client currently connected into the switch."
            }
        }
        p { class: "mb-4",
            "Example 6-6 shows samples of each, with user wendell currently connected to the switch."
        }

        img {
            class: "mb-4 rounded-lg",
            alt: "Example 6-6 Displaying SSH Status",
            loading: "lazy",
            src: asset!("/assets/static/v1p2c6s1sh4ex6-6.png", AssetOptions::image().with_avif()),
        }

        {h3_heading("RECAP")}
        ol { class: "list-disc list-inside",
            li {
                "IOS by default runs SSH and Telnet servers and allows both connections into the vty lines."
            }
            li {
                "Telnet has one serious disadvantage: all data in the Telnet session flows as clear text, including the 
                password exchanges."
            }
            li {
                "SSH encrypts all data transmitted between the SSH client and server, protecting the data and passwords."
            }
            li {
                "SSH cannot rely on authentication methods that do not include a username, like shared passwords."
            }
            li {
                "The switch creates the FQDN from the hostname and domain name of the switch and it is used as input
                to create the SSH encryption key."
            }
        }
    }
}