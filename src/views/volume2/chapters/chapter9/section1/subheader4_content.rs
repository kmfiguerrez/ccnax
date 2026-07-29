use dioxus::prelude::*;

use crate::utils::text_command::text_command;

#[component]
pub fn Content() -> Element {
    rsx! {
        p { class: "mb-3",
            "Log messages may just tell you about some mundane event, or they may tell you of some
            critical event."
            br {}
            strong {
                "To help you make sense of the importance of each message, IOS assigns each
                message a severity level (as noted in the same messages in the preceding page or so)."
            }
            br {}
            "Figure 9-3 shows the severity levels: the lower the number, the more severe the event that caused
            the message."
            br {}
            strong { "(Note that the values on the left and center are used in IOS commands.)" }
        }

        img {
            class: "mb-4",
            src: asset!("/assets/static/v2p3c9s1sh3f9-3.png", AssetOptions::image().with_avif()),
        }

        p { class: "mb-4",
            "Figure 9-3 breaks the eight severity levels into four sections just to make a little more sense
            of the meaning ."
        }

        div { class: "mb-4",

            h3 { class: "font-semibold", "The severe section" }
            p {
                "The two top levels in the figure are the most severe."
                br {}
                "Messages from this level mean a serious and immediate issue exists."
            }
        }

        div { class: "mb-4",

            h3 { class: "font-semibold", "The Impactful section" }
            p {
                "The next three levels, called Critical, Error, and Warning, also tell about events that impact the device, but they 
            are not as immediate and severe."
                br {}
                "For instance, one common log message about an interface failing to a physically down state shows as a severity level 
            3 message."
            }
        }

        div { class: "mb-4",

            h3 { class: "font-semibold", "The Normal section" }
            p {
                "Continuing down the figure, IOS uses the next two levels (5 and 6) for messages that are more about notifying 
                the user rather than identifying errors."
            }
        }

        div { class: "mb-4",

            h3 { class: "font-semibold", "The Normal section" }
            p {
                " Finally, the last level in the figure is used for messages requested by the "
                {text_command("debug")}
                " command, as shown in an example later in this chapter."
            }
        }

        h3 { class: "font-semibold", "The Configuration logging commands" }
        p { class: "mb-3",
            " Table 9-2 summarizes the configuration commands used to enable logging and to set the severity level for each type."
            br {}
            strong {
                "When the severity level is set, IOS will send messages of that severity level and more severe ones 
                (lower severity numbers) to the service identified in the command."
            }
            br {}
            "For example, the command "
            {text_command("logging console 4")}
            " causes IOS to send severity level 0-4 messages to the console."
            br {}
            "Also, note that the command to disable each service is the "
            span { class: "font-semibold", "no" }
            " version of the command, with "
            i { "no" }
            " in front of the command ( "
            {text_command("no logging console")}
            ", "
            {text_command("no logging monitor")}
            " and so on )."
        }
        img {
            class: "mb-4",
            src: asset!("/assets/static/v2p3c9s1sh3t9-2.png", AssetOptions::image().with_avif()),
        }

        h3 { class: "font-semibold mb-1", "REMEMBER" }
        ul { class: "list-disc pl-4",
            li {
                "By default, on cisco IOS version 03.16.05.S, in Packet Tracer, both the console and Telnet & SSH users receive messages from levels 0-7"
            }
            li {
                "Storing syslog messages in RAM and Syslog Server is disabled by default based on the said IOS version."
            }
        }
    }
}