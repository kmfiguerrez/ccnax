use dioxus::prelude::*;

use crate::{components::RedNote, utils::{text_command, TextCommandColor, h3_heading}};

#[component]
pub fn Content() -> Element {
    rsx! {
        p { class: "mb-3",
            "With the information in Table 9-2 ( "
            strong { "Table 9-2 can be viewed in the previous content" }
            " ), configuring syslog in a Cisco IOS 
            router or switch should be relatively straightforward."
            br {}
            "Example 9-2 shows a sample, based on Figure 9-4."
            br {}
            "The figure shows a syslog server at IP address 172.16.3.9."
            br {}
            "Both switches and both routers will use the same configuration shown in Example 9-2, 
            although the example shows the configuration process on a single device, router R1."
        }

        img {
            class: "mb-3",
            src: asset!("/assets/static/v2p3c9s1sh3f9-4.png", AssetOptions::image().with_avif()),
            alt: "Figurez 9-4: Sample Network Used in Logging Examples",
        
        }
        img {
            class: "mb-4",
            src: asset!("/assets/static/v2p3c9s1sh3ex9-2.png", AssetOptions::image().with_avif()),
            alt: "Example 9-2: Syslog configuration on Router 1",
        
        }

        RedNote {
            p {
                strong { "Note in example 9-2:" }
                " On Cisco IOS version 03.16.05.S in Packet Tracer"
            }
            ul { class: "list-disc pl-4 mb-4",
                li {
                    "You can't specify the the severity level using both the name and number in the commands for "
                    span { class: "font-bold", "logging console" }
                    ", "
                    span { class: "font-bold", "logging monitor" }
                    " and "
                    span { class: "font-bold", "logging buffered" }
                    " as they default to receive messages of all levels."
                
                }
                li {
                    "For the command: "
                    span { class: "font-bold", "logging buffered" }
                    ", you have to specify the amount of memory (buffer size)."
                }
                li {
                    "For the command: "
                    span { class: "font-bold", "logging trap" }
                    ", you can only set the debug level option which means will also receive messages of all levels."
                }
            }
        }

        p { class: "mb-4",
            "First, note that the example configures the same message level at the console and for terminal 
            monitoring (level 7, or debug), and the same level for both buffered and logging to the
            syslog server (level 4, or warning)."
            br {}
            "The levels may be set using the numeric severity level or the name as shown earlier in Figure 9-3."
        }

        p { class: "mb-3",
            "The "
            {text_command("show logging", TextCommandColor::Gold)}
            " command confirms those same configuration settings and also lists the
            log messages per the logging buffered configuration."
            br {}
            "Example 9-3 shows a sample, with the
            configuration settings to match Example 9-2 highlighted in gray."
        }
        figure { class: "mb-4",
            figcaption {
                span { class: "font-bold", "Example 9-3: " }
                "Viewing the Configured Log Settings per the Earlier Example"
            }
            img {
                class: "mb-4",
                src: asset!("/assets/static/v2p3c9s1sh3ex9-3a.png", AssetOptions::image().with_avif()),
                alt: "Example 9-3: Ouput of the show logging command",
            }
        }

        p { class: "mb-4",
            strong {
                "You might notice by now that knowing the names of all eight log message levels can be handy if you want to understand 
                the output of the commands."
            }
            br {}
            " Most of the show commands list the log message levels by name, not by number."
            br {}
            " As you can see in the green highlights in this example, two levels list “debug,” and two list “warning,” even though 
            some of the configuration commands referred to those levels by number."
        }

        p { class: "mb-4",
            "Also, you cannot know this from the output, but in Example 9-3, router R1 has no buffered log messages."
            br {}
            " (Note the counter value of 0 for buffered logging messages.)"
            br {}
            " If any log messages had been buffered, the actual log messages would be listed at the end of the command."
            br {}
            "In this case, the router just booted, and no messages has been buffered yet."
            br {}
            "(You could also clear out the old messages from the log with the "
            {text_command("clear logging", TextCommandColor::Gold)}
            " EXEC command.)"
        }

        RedNote {
            p {
                strong { "Note:" }
                " On Cisco IOS version 03.16.05.S in Packet Tracer, the "
                span { class: "font-bold", "clear logging" }
                " is not available!"
            }
        }

        p { class: "mb-4",
            "The next example shows the difference between the current severity levels."
            br {}
            "This example shows the user disabling interface G0/1 on R1 with the "
            {text_command("shutdown", TextCommandColor::Gold)}
            " command and then reenabling it with the "
            {text_command("no shutdown", TextCommandColor::Gold)}
            " command."
            br {}
            "If you look closely at the highlighted messages, you will see several severity 5 messages and one severity 3 message."
            br {}
            "The "
            {text_command("logging buffered 4", TextCommandColor::Gold)}
            " global configuration command on R1 (see Example 9-2) means that R1 will not buffer the
            severity level 5 log messages, but it will buffer the severity level 3 message and more severe messages."
            br {}
            "Example 9-4 ends by showing that log message at the end of the output of the "
            {text_command("show logging", TextCommandColor::Gold)}
            " command."
        }

        figure { class: "mb-4",
            figcaption {
                span { class: "font-bold", "Example 9-4: " }
                "Seeing Severity 3 and 5 Messages at the Console, and Severity 3 Only in the Buffer"
            }
            img {
                class: "mb-4",
                src: asset!("/assets/static/v2p3c9s1sh3ex9-4.png", AssetOptions::image().with_avif()),
                alt: "Example 9-4: Ouput of the configuration and show logging commands",
            }
        }

        RedNote {
            p {
                strong { "Note in Example 9-4:" }
                " On Cisco IOS version 03.16.05.S in Packet Tracer, logging by buffer doesn't work!"
            }
        }

        {h3_heading("RECAP")}
        ul { class: "list-disc pl-4",
            li {
                "It's best to know all the eight log messages to understand the output of the commands."
            }
            li { "Most of the show commands list the log message levels by name, not by number." }
            li {
                "You could also clear out the old messages from the log with the "
                {text_command("clear logging", TextCommandColor::Gold)}
                " EXEC command."
            }
        }
    }
}