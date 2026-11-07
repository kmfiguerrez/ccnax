use dioxus::prelude::*;

#[component]
pub fn SectionIntroductionContent() -> Element {
    rsx! {
        p { class: "mb-4",
            "Connecting a Cisco wireless LAN controller to the network is not quite as straightforward
            because it has several different types of connections."
            br {}
            strong {
                "From your work with Cisco routers and switches, you probably know that the terms interface and port are usually 
                interchangeable."
            }
            br {}
            "For example, switches can come in 48-port models, and you apply configuration changes to the corresponding interfaces."
            br {}
            strong {
                "Cisco wireless controllers differ a bit; ports and interfaces refer to different concepts."
            }
        }

        p { class: "mb-4",
            "Controller ports are physical connections made to an external wired or switched network, whereas interfaces are 
            logical connections made internally within the controller."
            br {}
            "The following sections explain each connection type in more detail."
            br {}
            "You will learn more about configuring ports and interfaces in the “Configuring a WLAN” section later in the chapter."
        }
    }
}