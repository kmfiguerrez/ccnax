use dioxus::prelude::*;

use crate::utils::text_command::text_command;

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
        }
        img {
            class: "mb-4",
            src: asset!("/assets/static/v2p3c9s1sh3ex9-2.png", AssetOptions::image().with_avif()),
        }

        p { class: "mb-4",
            "First, note that the example configures the same message level at the console and for terminal 
            monitoring (level 7, or debug), and the same level for both buffered and logging to the
            syslog server (level 4, or warning)."
            br {}
            "The levels may be set using the numeric severity level or the name as shown earlier in Figure 9-3."
        }

        p {
            "The "
            {text_command("show logging")}
            " command confirms those same configuration settings and also lists the
            log messages per the logging buffered configuration."
            br {}
            "Example 9-3 shows a sample, with the
            configuration settings to match Example 9-2 highlighted in gray."
        }

    }
}