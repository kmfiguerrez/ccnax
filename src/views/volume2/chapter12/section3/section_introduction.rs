use dioxus::prelude::*;

use crate::utils::h3_heading;

#[component]
pub fn SectionIntroductionContent() -> Element {
    rsx! {
        p { class: "mb-4",
            "This final of three major sections of the chapter focuses on two topics: File Transfer
            Protocol (FTP) and Trivial File Transfer Protocol (TFTP)."
            br {}
            "Both exist as TCP/IP protocols defined in RFCs."
            br {}
            strong {
                "Both use a client and server model, in which the client connects to a server and then the client can copy files to 
                the server or from the server."
            }
            br {}
            "Both exist as a myriad of implementations of both client and server code, from command-line clients to apps with
            graphical interfaces, using the respective FTP or TFTP protocols behind the scenes."
        }

        p { class: "mb-1", "This section discusses FTP and TFTP with two branches." }
        ol { class: "list-disc list-inside mb-4",
            li {
                "The first section takes a practical view of the most common use of TFTP and FTP by network engineers while on the 
                job: the job of updating IOS images. The process can make use of TFTP and FTP, so this section provides the basics."
            }
            li {
                "The second branch of this final major section then moves on to talk about FTP and TFTP in a much broader sense, with 
                details about each protocol, their capabilities, and what capabilities each provides to any user."
            }
        }
    }
}