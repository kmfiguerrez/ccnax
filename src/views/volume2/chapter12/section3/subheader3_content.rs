use dioxus::prelude::*;

use crate::{utils::h3_heading, components::KeyTopic};

#[component]
pub fn Content() -> Element {
    rsx! {
        p { class: "mb-4",
            "FTP has a role as a general file transfer tool for any user, with a good number of FTP client application options 
            available."
            br {}
            "TFTP plays a much smaller role as a tool for the average user, but it does play a more useful role for IT support staff."
        }

        p { class: "mb-4",
            "For the basics, Trivial File Transfer Protocol uses UDP well-known port 69."
            br {}
            strong {
                "Because it uses UDP, TFTP adds a feature to check each file for transmission errors by using a checksum
            process on each file after the transfer completes."
            }
        }

        div { class: "flex gap-x-3 items-center mb-4",
            KeyTopic {}
            p {
                "The word "
                i { "trivial" }
                " in the name refers to its relatively small number of features, meant to be
                an advantage by making the tool lightweight."
                br {}
                "For instance, it supports far fewer commands than FTP (fewer functions), meaning that the code requires less space 
                to install, which can be useful for devices with limited memory."
                br {}
                "TFTP can Get and Put files, but it includes no commands to change directories, create/remove directories, or even 
                to list files on the  server."
                br {}
                "TFTP does not support even simple clear-text authentication."
                br {}
                "In effect, if a TFTP server is running, it should accept requests from any TFTP client."
            }
        }

        p { class: "mb-1",
            strong {
                "Ideally, TFTP has its best use as a temporary tool for quick file transfers in a controlled
                environment, particularly when the data itself does not have to be secure."
            }
            br {}
            "For instance, imagine this scenario:"
        }
        ol { class: "list-decimal list-inside mb-4",
            li { "A network engineer keeps all router and switch IOS images in a folder." }
            li {
                "The engineer enables a TFTP server on her laptop as needed; otherwise, the TFTP
                server remains disabled."
            }
            li {
                "The engineer connects her laptop to a LAN and enables the TFTP server long enough
                to transfer IOS images into or out of a few devices."
            }
            li {
                "If the engineer forgets to disable TFTP, the only risk is that someone may copy an
                IOS image—an image that is already available from Cisco.com to any customer ."
            }
        }

        {h3_heading("RECAP")}
        ol { class: "list-disc list-inside",
            li { "Trivial File Transfer Protocol uses UDP well-known port 69." }
            li {
                "Because it uses UDP, TFTP adds a feature to check each file for transmission errors by using a checksum process 
                on each file after the transfer completes."
            }
            li {
                "The word "
                i { "trivial" }
                " in the name refers to its relatively small number of features, meant to be
                an advantage by making the tool lightweight suitable for devices with limited memory."
            }
            li {
                "Ideally, TFTP has its best use as a temporary tool for quick file transfers in a controlled environment, 
                particularly when the data itself does not have to be secure."
            }
        }
    }
}