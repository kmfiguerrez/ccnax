use dioxus::prelude::*;

use crate::{components::KeyTopic, utils::{TextCommandColor, h3_heading, text_command}};

#[component]
pub fn Content() -> Element {
    rsx! {
        p { class: "mb-4",
            "As you might imagine, network engineers need a way to bring down an interface without
            having to travel to the switch and remove a cable."
            br {}
            "In short, we need to be able to decide which ports should be enabled and which should be disabled."
        }

        {h3_heading("Administratively enabling and disabling an interface")}
        p { class: "mb-4",
            "In an odd turn of phrase, Cisco uses two interface subcommands to configure the idea of
            administratively enabling and disabling an interface: the "
            {text_command("shutdown", TextCommandColor::Gold)}
            " command (to disable) and
            the "
            {text_command("no shutdown", TextCommandColor::Gold)}
            " command (to enable)."
            br {}
            "While the "
            {text_command("no shutdown", TextCommandColor::Gold)}
            " command might seem like an odd command to enable an interface at first, 
            you will use this command a lot in the lab, and it will become second nature. (Most people, in fact, 
            use the abbreviations "
            {text_command("shut", TextCommandColor::Gold)}
            " and "
            {text_command("no shut", TextCommandColor::Gold)}
            ".)"
        }

        {h3_heading("Disabling an interface")}
        p { class: "mb-4",
            "Example 7-4 shows an example of disabling an interface using the "
            {text_command("shutdown", TextCommandColor::Gold)}
            " interface subcommand."
            br {}
            "In this case, switch SW1 has a working interface F0/1."
            br {}
            "The user connects at the console and disables the interface."
            br {}
            "IOS generates a log message each time an interface fails or recovers, and log messages appear at 
            the console, as shown in the example."
        }

        KeyTopic {}
        img {
            class: "mb-4 rounded-lg",
            alt: "Example 7-4 Administratively Disabling an Interface with shutdown",
            loading: "lazy",
            src: asset!("/assets/static/v1p2c7s1sh3ex7-4.png", AssetOptions::image().with_avif()),
        }

        {h3_heading("Enabling an interface")}
        p { class: "mb-4",
            "To bring the interface back up again, all you have to do is follow the same process but use
            the "
            {text_command("no shutdown", TextCommandColor::Gold)}
            " command instead."
        }

        {h3_heading("Listing the status of an interface")}
        p { class: "mb-4",
            "Before leaving the simple but oddly named "
            {text_command("shutdown/no shutdown", TextCommandColor::Gold)}
            " commands, take a look at two important show commands that list the status of a shutdown interface."
            br {}
            "The "
            {text_command("show interfaces status", TextCommandColor::Gold)}
            " command lists one line of output per interface, and when shut down,
            lists the interface status as “disabled.”"
            br {}
            "That makes logical sense to most people."
            br {}
            "The "
            {text_command("show interfaces", TextCommandColor::Gold)}
            " command (without the "
            {text_command("status", TextCommandColor::Gold)}
            " keyword) lists many lines of output per interface,
            giving a much more detailed picture of interface status and statistics."
            br {}
            "With that command, the interface status comes in two parts, with one part using the phrase “administratively
            down,” matching the highlighted log message in Example 7-4."
        }

        p { class: "mb-4",
            "Example 7-5 shows an example of each of these commands."
            br {}
            "Note that both examples also use the F0/1 parameter (short for Fast Ethernet0/1), which limits the output to 
            the messages about F0/1 only."
            br {}
            "Also note that F0/1 is still shut down at this point."
        }

        p { class: "mb-4", "See Example 7-5 in volume 1 on page 156" }

        {h3_heading("RECAP")}
        ol { class: "list-disc list-inside",
            li {
                "Cisco uses two interface subcommands to configure the idea of
                administratively enabling and disabling an interface: the "
                {text_command("shutdown", TextCommandColor::Gold)}
                " command (to disable) and
                the "
                {text_command("no shutdown", TextCommandColor::Gold)}
                " command (to enable)."
            }
            li {
                "The "
                {text_command("show interfaces status", TextCommandColor::Gold)}
                " command lists one line of output per interface, and when shut down,
                lists the interface status as “disabled.”"
            }
            li {
                "The "
                {text_command("show interfaces", TextCommandColor::Gold)}
                " command (without the "
                {text_command("status", TextCommandColor::Gold)}
                " keyword) lists many lines of output per interface,
                giving a much more detailed picture of interface status and statistics.
                The interface status comes in two parts, with one part using the phrase “administratively down,”"
            }
            li {
                "You can limit the output of both the "
                {text_command("show interfaces status/show interfaces", TextCommandColor::Gold)}
                " commands about one  interface only by passing an "
                i { "interface" }
                " parameter after the "
                {text_command("interfaces", TextCommandColor::Gold)}
                " keyword"
            }
            li {
                "You can also use the "
                {text_command("show ip interface brief", TextCommandColor::Gold)}
                " command to check the status of interfaces that
                also uses the phrase “administratively down”."
            }
        }
    }
}