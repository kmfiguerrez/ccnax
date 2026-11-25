use dioxus::prelude::*;

use crate::{
    components::{KeyTopic, GreenNote},
    utils::h3_heading
};

#[component]
pub fn Content() -> Element {
    rsx! {
        p { class: "mb-1",
            "If your new WLAN will use a security scheme that requires a RADIUS server, such as
            WPA2-Enterprise or WPA3-Enterprise, you will need to define the server first."
            br {}
            strong { "Select Security > AAA > RADIUS > Authentication" }
            " to see a list of servers that have already been
            configured, as shown in Figure 29-8."
            br {}
            "If multiple servers are defined, the controller will try them in sequential order."
            br {}
            "Click "
            strong { "New" }
            " to create a new server."
        }
    }
}