use dioxus::prelude::*;

use crate::utils::{TextCommandColor, h3_heading, text_command};

#[component]
pub fn Content() -> Element {
    rsx! {
        p { class: "mb-4",
            "These next three configuration commands have little in common, other than the fact that
            they can be useful settings to reduce your frustration when using the console of a switch or
            router."
        }

        p { class: "mb-4",
            "The console automatically receives copies of all unsolicited syslog messages on a switch."
            br {}
            "The idea is that if the switch needs to tell the network administrator some important and possibly
            urgent information, the administrator might be at the console and might notice the message."
        }

        p { class: "mb-4",
            "Unfortunately, IOS (by default) displays these syslog messages on the console's screen at
            any time—including right in the middle of a command you are entering, or in the middle of
            the output of a show command."
            br {}
            "Having a bunch of text show up unexpectedly can be a bit annoying"
        }

        {h3_heading("The logging console command")}
        p { class: "mb-4",
            "You could simply disable the feature that sends these messages to the console and then reenable the feature 
            later using the "
            {text_command("no logging console", TextCommandColor::Gold)}
            " and "
            {text_command("logging console", TextCommandColor::Gold)}
            " global configuration commands."
            br {}
            "For example, when working from the console, if you want to temporarily not be bothered by log messages, 
            you can disable the display of these messages with the "
            {text_command("no logging console", TextCommandColor::Gold)}
            " global configuration command, and then when finished, enable them again."
        }

        {h3_heading("The logging synchronous command")}
        p { class: "mb-4",
            "However, IOS supplies a reasonable compromise, telling the switch to display syslog messages
            only at more convenient times, such as at the end of output from a "
            {text_command("show", TextCommandColor::Gold)}
            " command."
            br {}
            "To do so, just configure the "
            {text_command("logging synchronous", TextCommandColor::Gold)}
            " console line subcommand, which basically tells IOS to
            synchronize the syslog message display with the messages requested using "
            {text_command("show", TextCommandColor::Gold)}
            " commands."
        }

        {h3_heading("The exec-timeout command")}
        p { class: "mb-4",
            "Another way to improve the user experience at the console is to control timeouts of the
            login session from the console or when using Telnet or SSH."
            br {}
            "By default, the switch automatically disconnects console and vty (Telnet and SSH) users after 
            5 minutes of inactivity."
            br {}
            "The "
            {text_command("exec-timeout", TextCommandColor::Gold)}
            i { " minutes seconds" }
            " line subcommand enables you to set the length of that inactivity timer."
            br {}
            "In the lab (but not in production), you might want to use the special value of 0
            minutes and 0 seconds meaning “never time out.”"
        }

        {h3_heading("The no ip domain-lookup command")}
        p { class: "mb-4",
            "Finally, IOS has an interesting combination of features that can make you wait for a minute
            or so when you mistype a command."
            br {}
            "First, IOS tries to use DNS name resolution on IP hostnames—a generally useful feature."
            br {}
            "If you mistype a command, however, IOS thinks you want to telnet to a host by that name."
            br {}
            "With all default settings in the switch, the switch tries to resolve the hostname, cannot find a DNS server, 
            and takes about a minute to time out and give you control of the CLI again."
        }

        p { class: "mb-4",
            "To avoid this problem, configure the "
            {text_command("no ip domain-lookup", TextCommandColor::Gold)}
            " global configuration command,
            which disables IOS's attempt to resolve the hostname into an IP address."
        }

        p { class: "mb-4",
            "Example 6-10 collects all these commands into a single example, as a template for some
            good settings to add in a lab switch to make you more productive"
        }

        img {
            class: "mb-4 rounded-lg",
            alt: "Table 6-2 Commands Related to the History Buffer",
            loading: "lazy",
            src: asset!("/assets/static/v1p2c6s3sh2ex6-10.png", AssetOptions::image().with_avif()),
        }

        {h3_heading("RECAP")}
        ol { class: "list-disc list-inside",
            li {
                "If you want to temporarily not be bothered by log messages, 
            you can disable the display of these messages with the "
                {text_command("no logging console", TextCommandColor::Gold)}
                " global configuration command, and then when finished, enable them again."
            }
            li {
                "You can use the "
                {text_command("logging synchronous", TextCommandColor::Gold)}
                " console line subcommand to tell the IOS to
                display syslog messages only at more convenient times, such as at the end of output from a "
                {text_command("show", TextCommandColor::Gold)}
                " command."
            }
            li {
                "The "
                {text_command("exec-timeout", TextCommandColor::Gold)}
                i { " minutes seconds" }
                " line subcommand enables you to set the length of the inactivity timer."
            }
            li {
                "By default, the switch automatically disconnects console and vty (Telnet and SSH) users after 5 minutes 
                of inactivity."
            }
            li {
                "The "
                {text_command("no ip domain-lookup", TextCommandColor::Gold)}
                " global configuration command disables IOS's attempt to resolve the hostname into an IP address."
            }
        }
    }
}