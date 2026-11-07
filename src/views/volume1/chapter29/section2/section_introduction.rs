use dioxus::prelude::*;

use crate::{
    components::GreenNote,
    utils::h3_heading
};

#[component]
pub fn SectionIntroductionContent() -> Element {
    rsx! {
        p { class: "mb-4",
            "To connect and configure a WLC, you will need to open a web browser to the WLC's management address with either HTTP 
            or HTTPS."
            br {}
            strong {
                "This can be done only after the WLC has an initial configuration and a management IP address assigned to its 
                management interface."
            }
            br {}
            "The web-based GUI provides an effective way to monitor, configure, and troubleshoot a wireless network."
            br {}
            "You can also connect to a WLC with an SSH session, where you can use its CLI to monitor, configure, and debug activity."
        }

        p { class: "mb-4",
            strong { "Both the web-based GUI and the CLI require management users to log in." }
            br {}
            "Users can be authenticated against an internal list of local usernames or against an authentication, authorization, 
            and accounting (AAA) server, such as TACACS+ or RADIUS."
        }

        p { class: "mb-4",
            "When you first open a web browser to the management address, you will see the initial login screen."
            br {}
            "Click on the Login button, then enter your user credentials as you are prompted for them."
        }

        GreenNote {
            strong { "NOTE" }
            " The CCNA exam objectives focus on using the WLC GUI to configure a WLAN and a security suite. Therefore, the examples 
            in this section assume that someone has already entered an initial configuration to give the WLC a working IP address for management."
        }

        p { class: "mb-4",
            "When you are successfully logged in, the WLC will display a monitoring dashboard similar to the one shown in 
            Figure 29-3."
            br {}
            "You will not be able to make any configuration changes there, so you must click on the Advanced link in the 
            upper-right corner."
            br {}
            "This will bring up the full WLC GUI, as shown in Figure 29-4."
        }

        p { class: "mb-4",
            "Notice the tabs across the top of the screen in Figure 29-4."
            br {}
            "You can select categories of functions from among Monitor, WLANs, Controller, Wireless, Security, and so on."
            br {}
            "As you select one of these categories, the vertical list of functions at the left side of the screen will
            change accordingly."
            br {}
            "You can expand the list entries if needed and select one to work on."
            br {}
            "The main screen area will display all of the relevant fields and options you can edit as you make
            configuration changes."
            br {}
            "You will get a feel for which tabs and list items you should use as you work through the remainder of the chapter."
        }

        p { class: "mb-4", "See Figure 29-3, 29-4 in volume 1 on page 671." }

        {h3_heading("RECAP")}
        ol { class: "list-disc list-inside",
            li {
                "To connect and configure a WLC, you will need to open a web browser to the WLC's management address with either 
                HTTP or HTTPS."
            }
            li {
                "The web-based GUI or CLI can be used to monitor, configure, and troubleshoot a wireless network."
            }
        }
    }
}