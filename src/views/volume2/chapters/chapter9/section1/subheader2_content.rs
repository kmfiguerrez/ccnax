use dioxus::prelude::*;

use crate::utils::text_command::text_command;

#[component]
pub fn Content() -> Element {
    rsx! {
        p { class: "mb-2",
            "With logging to the console and to terminals, an event happens, IOS sends the messages to
            the console and terminal sessions, and then IOS can discard the message."
            br {}
            "However, clearly, it would be useful to keep a copy of the log messages for later review, so IOS provides two
            primary means to keep a copy."
        }
        ol { class: "list-decimal pl-4",
            li {
                "IOS can store copies of the log messages in RAM by virtue of the "
                {text_command("logging buffered")}
                " global configuration command."
                br {}
                "Then any user can come back later and see the old log messages by using the "
                {text_command("show logging")}
                " EXEC command."
            }
            li {
                "As a second option—an option used frequently in production networks—all devices store
                their log messages centrally to a syslog server."
                br {}
                "RFC 5424 defines the syslog protocol, which provides the means by which a device like a switch or router can use 
                a UDP protocol to send messages to a syslog server for storage."
                br {}
                "All devices can send their log messages to the server."
                br {}
                "Later, a user can connect to the server (typically with a graphical user interface) and browse the log messages 
                from various devices"
                br {}
                "To configure a router or switch to send log messages to a syslog server, add the "
                {text_command("logging host")}
                " {{address | hostname}} global command, referencing the IP address or host name of the syslog server."
            
            }
        }

    }
}