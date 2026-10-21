use dioxus::prelude::*;

use crate::{
    components::{
        KeyTopic,
        GreenNote, 
        my_accordion::{Accordion, AccordionContent, AccordionItem, AccordionTrigger}
    }, utils::{h3_heading, text_command, TextCommandColor, h4_heading}
};

#[component]
pub fn Content() -> Element {
    rsx! {
        p { class: "mb-4",
            "The IOS "
            {text_command("copy", TextCommandColor::Gold)}
            " command, when using the "
            {text_command("tftp", TextCommandColor::Gold)}
            " or "
            {text_command("ftp", TextCommandColor::Gold)}
            " keyword, makes the command act as a client."
            br {}
            "The client connects to a TFTP or FTP server and then attempts to transfer the file."
            br {}
            "In the examples from the IOS, that "
            {text_command("copy", TextCommandColor::Gold)}
            " command copied the file from the server into the client device (a router)."
        }
        p { class: "mb-4",
            "The rest of this section examines what happens behind the scenes in that process, with a
            closer look at both FTP and TFTP as protocols and tools."
        }

        Accordion { class: "mb-4",
            AccordionItem {
                AccordionTrigger { {h3_heading("FTP Protocol Basics")} }
                AccordionContent {
                    p { class: "mb-4",
                        "FTP has long been a core Internet protocol, serving as the primary file transfer protocol for
                        several decades."
                        br {}
                        "RFC 959, which standardizes FTP, dates back to 1985."
                        br {}
                        strong {
                            "FTP uses TCP as its transport protocol, relying on TCP to provide an error-free in-order deliver of data 
                            so that the FTP application knows that each file transfer creates an exact copy of the file with no
                            omissions."
                        }
                        br {}
                        "FTP uses well-known TCP port 21 and in some cases also well-known port 20."
                    }

                    p { class: "mb-4",
                        strong { "As for normal operation, FTP uses a client/server model for file transfer" }
                        ", as shown in the
                        example in Figure 12-15."
                        br {}
                        "The figure shows the major steps but not every message."
                        br {}
                        "For instance, step 1 shows host A creating a TCP connection to the server 
                        (which takes the usual three TCP messages)."
                        br {}
                        "Step 2 represents the exchange that allows the server to authenticate the client."
                        br {}
                        "Step 3 shows the idea that, once authenticated, the client and server can send FTP
                        commands over the connection to tell the other device what to do."
                    }

                    img {
                        class: "mb-4 rounded-lg",
                        loading: "lazy",
                        alt: "Figure 12-15 Major Concepts with FTP Clients and Servers",
                        src: asset!("/assets/static/v2p3c12s3sh2f12-15.png", AssetOptions::image().with_avif()),
                    }

                    {h4_heading("FTP commands")}
                    p { class: "mb-1",
                        "The commands that flow over this initial TCP connection—called the FTP control connection—define the kinds 
                        of functions supported by FTP."
                        br {}
                        "Those commands allow the client to navigate around the directory structures of the server, list files, and 
                        then transfer files from the server (FTP GET) or to the server (FTP PUT)."
                        br {}
                        "Following is a summary of some of the FTP actions: "
                    }
                    KeyTopic {}
                    ol { class: "list-disc list-inside mb-4",
                        li {
                            span { class: "font-bold mr-1", "Navigate directories:" }
                            "List the current directory, change the current directory to a new directory, go back to the home 
                            directory, all on both the server and client side of the connection."
                        }
                        li {
                            span { class: "font-bold mr-1", "Add/remove directories:" }
                            "Create new directories and remove existing directories on both the client and server."
                        }
                        li {
                            span { class: "font-bold mr-1", "List files:" }
                            "List files on both the client and server."
                        }
                        li {
                            span { class: "font-bold mr-1", "File transfer:" }
                            "Get (client gets a copy of the file from the server), Put (client takes a file
                            that exists on the client and puts a copy of the FTP server)."
                        }
                    }

                    {h4_heading("GUI FTP client")}
                    p { class: "mb-4",
                        "While many OSs support command-line FTP clients, which require you to learn the various FTP commands and 
                        use those from the command line, most users instead use an FTP client app that issues the FTP commands 
                        behind the scenes."
                        br {}
                        strong {
                            "Clients typically display files on the local system as well as the server with a user interface similar to 
                            a typical file browser on a desktop OS (for instance, Windows Explorer, macOS Finder)."
                        }
                        br {}
                        "Figure 12-16 shows a sample user interface from the Filezilla FTP client (Filezilla-project.org)."
                    }

                    p { class: "mb-4", "See Figure 12-16 in volume 1 on page 276." }

                    p { class: "mb-4",
                        "The client application in Figure 12-16 lists the client computer's local file system on the left
                        and the FTP server's file system on the right."
                        br {}
                        "The user can click on the right to change directories, much like using any app that browses a file system, 
                        with FTP performing the commands behind the scenes."
                        br {}
                        "The user can also drag and drop files from the left to the right to put a file on the server, or vice versa 
                        to get a file from the server."
                    }

                    {h4_heading("FTP server")}
                    p { class: "mb-4",
                        strong {
                            "The FTP server can be a server application installed and managed by others, or you can
                        install or enable an FTP server for your own use."
                        }
                        br {}
                        "For instance, a network engineer might install an FTP server application on her laptop for use in 
                        upgrading IOS files, while the IT staff may keep an FTP server available 24/7 for all employees of the 
                        company to use."
                        br {}
                        "A simple Internet search can show a variety of FTP server applications that run on the common desktop OSs."
                        br {}
                        "Additionally, both Windows 10 and macOS come with an FTP or FTPS (FTP Secure) server option built into the 
                        OS; all you have to do is enable it."
                        br {}
                        "(The Linux distributions all have FTP servers available via simple downloads.)"
                    }

                    p { class: "mb-4",
                        strong { "Once installed, the server can be configured with a variety of settings." }
                        br {}
                        "For instance, the server needs to specify which users can access the server, so it can use the same login 
                        credentials allowed for the host where it resides or specify other credentials."
                        br {}
                        "It can specify the directories that each user can access, and whether the user has read-only or read-write 
                        access. "
                    }
                }
            }
            AccordionItem {
                AccordionTrigger { {h3_heading("FTP Active and Passive Modes")} }
                AccordionContent {
                    p { class: "mb-4",
                        strong { "FTP can operate in either active or passive mode." }
                        br {}
                        "The choice of mode may impact whether the TCP client can or cannot connect to the server and perform 
                        normal functions."
                        br {}
                        "The user at the FTP client can choose which mode to use, so this section works through the underlying 
                        details to explain why FTP passive mode may be the more likely option to work."
                    }

                    KeyTopic {}
                    p { class: "mb-1", "First, note that FTP uses two types of TCP connections: " }
                    ol { class: "mb-4",
                        li {
                            span { class: "font-bold mr-1", "Control Connection:" }
                            "Used to exchange FTP commands"
                        }
                        li {
                            span { class: "font-bold mr-1", "Data Connection:" }
                            "Used for sending and receiving data, both for file transfers and for output to display to a user"
                        }
                    }

                    {h4_heading("Control connection using TCP port 21")}
                    p { class: "mb-4",
                        strong {
                            "Given the two roles, when a client connects to an FTP server, the client first creates the FTP
                            control connection"
                        }
                        " as shown in Figure 12-17."
                        br {}
                        "The server listens for new control connections on its well-known port 21; the client allocates any new 
                        dynamic port (49222 in this case) and creates a TCP connection to the server."
                    }

                    KeyTopic {}
                    img {
                        class: "mb-4 rounded-lg",
                        loading: "lazy",
                        alt: "Figure 12-17 FTP Client Creates an FTP Control Connection",
                        src: asset!("/assets/static/v2p3c12s3sh2f12-17.png", AssetOptions::image().with_avif()),
                    }

                    {h4_heading("Data connection using TCP port 20")}
                    p { class: "mb-4",
                        strong {
                            "After creating the TCP connection, the user authenticates to the FTP server and takes some actions."
                        }
                        br {}
                        "Some of those actions require only the control connection, but eventually the user
                        will take an action (like getting a file) that requires a data connection."
                        br {}
                        strong {
                            "When that happens, to create the FTP data connection, the client will either use active mode or passive 
                            mode"
                        }
                        ", as shown in the next two examples."
                    }

                    {h4_heading("Active mode")}
                    p { class: "mb-1",
                        "Figure 12-18 shows an example of what happens in active mode."
                        br {}
                        "Following the steps in the figure:"
                    }
                    ol { class: "list-decimal list-inside mb-4",
                        li {
                            "The FTP client allocates a currently unused dynamic port and starts listening on that port."
                        }
                        li {
                            "The client identifies that port (and its IP address) to the FTP server by sending an FTP "
                            {text_command("PORT", TextCommandColor::Gold)}
                            " command to the server."
                        }
                        li {
                            "The server, because it also operates in active mode, expects the "
                            {text_command("PORT", TextCommandColor::Gold)}
                            " command; the server reacts and initiates the FTP data connection to the client's address
                            (192.168.1.102) and port (49333)."
                        }
                    }

                    img {
                        class: "mb-4 rounded-lg",
                        loading: "lazy",
                        alt: "Figure 12-18 FTP Active Mode Process to Create the Data Connection",
                        src: asset!("/assets/static/v2p3c12s3sh2f12-18.png", AssetOptions::image().with_avif()),
                    }

                    p { class: "mb-4",
                        strong {
                            "Active mode works well with both the FTP client and server sitting inside the same enterprise network."
                        }
                        br {}
                        "When within the same network, typically no NAT function and no firewall sits between the two."
                        br {}
                        strong {
                            "However, if the FTP client sits in an enterprise network, and the FTP server resides somewhere in the 
                            Internet, an active mode connection typically fails."
                        }
                        br {}
                        "Most firewalls do not allow Internet-based hosts to initiate TCP connections to hosts inside the
                        enterprise without a specific firewall rule allowing connections to a known port, and in this
                        case, the FTP client allocates any available port number."
                        br {}
                        "For instance, in Figure 12-18, the TCP connection (step 3) would be discarded by a firewall."
                    }

                    GreenNote {
                        p {
                            strong { "NOTE" }
                            " FTP reserves two well-known ports: port 21 for control connections and port 20
                            for data connections. However, due to changes to FTP over the years, FTP often uses other
                            TCP ports for the TCP data connection, as seen in the examples in this chapter."
                        }
                    }

                    {h4_heading("Passive mode")}
                    p { class: "mb-1",
                        strong {
                            "Passive mode helps solve the firewall restrictions by having the FTP client initiate the FTP
                            data connection to the server."
                        }
                        br {}
                        "However, passive mode does not simply cause the FTP client to connect to a well-known port on the server; 
                        it requires more exchanges of port numbers to use between the server and client, as shown in Figure 12-19, 
                        with these steps:"
                    }
                    ol { class: "list-decimal list-inside mb-4",
                        li {
                            "The FTP client changes to use FTP passive mode, notifying the server using the FTP "
                            {text_command("PASV", TextCommandColor::Gold)}
                            " command."
                        }
                        li {
                            "The server chooses a port to listen on for the upcoming new TCP connection, in this case TCP port 49444."
                        }
                        li {
                            "The FTP notifies the FTP client of its IP address and chosen port with the FTP "
                            {text_command("PORT", TextCommandColor::Gold)}
                            " command."
                        }
                        li {
                            "The FTP client opens the TCP data connection to the IP address and port learned at the previous step."
                        }
                    }

                    KeyTopic {}
                    img {
                        class: "mb-4 rounded-lg",
                        loading: "lazy",
                        alt: "Figure 12-19 FTP Passive Mode Process to Create the Data Connection",
                        src: asset!("/assets/static/v2p3c12s3sh2f12-19.png", AssetOptions::image().with_avif()),
                    }
                
                }
            }
            AccordionItem {
                AccordionTrigger { {h3_heading("FTP over TLS (FTP Secure)")} }
                AccordionContent {
                    p { class: "mb-4",
                        "FTP, defined in RFC 959 back in 1985, has some shortcomings with security."
                        br {}
                        strong {
                            "As originally defined, it does include the ability to use usernames and passwords for authentication and
                            authorization; however, the username/password flows as clear text."
                        }
                        br {}
                        "Additionally, all data transfers flow as clear text."
                    }

                    {h4_heading("Improvements for FTP")}
                    p { class: "mb-4",
                        "Over the years, several RFCs defined security improvements for FTP."
                        br {}
                        strong {
                            "Those new features include using digital certificates for authentication as well as using Transport Layer 
                            Security (TLS) to encrypt all data (including usernames/passwords)."
                        }
                        br {}
                        "Fast forward to today and many of those features converge into what most FTP clients and servers support 
                        as FTP over TLS or as FTP Secure (FTPS)."
                    }

                    {h4_heading("FTPS session")}
                    p { class: "mb-1",
                        "With FTPS, the client and server still use FTP commands and still use both a control and data connection."
                        br {}
                        "However, "
                        strong {
                            "FTPS encrypts both the control and data connections with TLS, including the exchange of the 
                            usernames and passwords"
                        }
                        "."
                        br {}
                        "FTPS includes a few variations, including the FTPS explicit mode process shown in Figure 12-20:"
                    }
                    ol { class: "list-decimal list-inside mb-4",
                        li {
                            "The client creates the FTP control TCP connection to server well-known port 21."
                        }
                        li {
                            "The client initiates the use of TLS in the control connection with the "
                            {text_command("FTP AUTH", TextCommandColor::Gold)}
                            " command."
                        }
                        li {
                            "When the user takes an action that requires an FTP data connection, the client creates
                            an FTP data TCP connection to server well-known port 21."
                        }
                        li {
                            "The client initiates the use of TLS in the data connection with the "
                            {text_command("FTP AUTH", TextCommandColor::Gold)}
                            " command."
                        }
                    }

                    img {
                        class: "mb-4 rounded-lg",
                        loading: "lazy",
                        alt: "Figure 12-20 FTPS Explicit Mode Control and Data Connection Establishment",
                        src: asset!("/assets/static/v2p3c12s3sh2f12-20.png", AssetOptions::image().with_avif()),
                    }

                    p { class: "mb-4",
                        "In contrast, the implicit mode process begins with a required TLS connection, with no need
                        for an "
                        {text_command("FTP AUTH", TextCommandColor::Gold)}
                        " command, using well-known ports 990 (for the control connection) and
                        989 (for the data connection)."
                    }

                    GreenNote {
                        p {
                            strong { "NOTE" }
                            " SSH File Transfer Protocol (SFTP) is a different protocol than FTPS. SFTP uses
                            SSH to encrypt file transfers over an SSH connection. However, the acronym SFTP does not
                            refer to a secure version of FTP."
                        }
                    }
                }
            }
        }

        {h3_heading("RECAP")}
        ol { class: "list-disc list-inside",
            li {
                "RFC 959, which standardizes FTP, dates back to 1985, has long been a core Internet protocol, serving as the 
                primary file transfer protocol for several decades."
            }
            li { "FTP uses well-known TCP port 21 and in some cases also well-known port 20." }
            li {
                "FTP can operate in either active or passive mode. The choice of mode may impact whether the TCP client can or cannot connect 
                to the server and perform normal functions."
            }
            li { "FTP uses two types of TCP connections: control and data connections." }
            li {
                "When a client connects to an FTP server, the client first creates the FTP control connection, 
                after creating the TCP connection, the user authenticates to the FTP server and takes some actions. 
                Then to create the FTP data connection, the client will either use active mode or passive mode."
            }
            li {
                "Active mode works well with both the FTP client and server sitting inside the same enterprise network, 
                where there's no NAT function and no firewall sits between the two."
            }
            li {
                "Passive mode helps solve the firewall restrictions by having the FTP client initiate the FTP data connection to 
                the server."
            }
            li {
                "FTP, defined in RFC 959 back in 1985, has some shortcomings with security. Credentials for authentication and 
                data transfers flow as clear text."
            }
            li {
                "FTP over TLS or as FTP Secure (FTPS) introduced security improvements for FTP including using digital certificates 
                for authentication as well as using Transport Layer Security (TLS) to encrypt all data 
                (including usernames/passwords)."
            }
            li {
                "SSH File Transfer Protocol (SFTP) is a different protocol than FTPS. SFTP uses SSH to encrypt file transfers over 
                an SSH connection."
            }
        }
    }
}