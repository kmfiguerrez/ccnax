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
            strong {
                "IOS exists as a file—a single file—that the router then loads into RAM to use as its operating system."
            }
            br {}
            "To better understand the process, you must understand a few more details about how IOS works."
            br {}
            strong {
                "In particular, you need to understand the IO file system (IFS), which defines how IOS stores files (including the IOS 
            file)."
            }
            br {}
            "The IOS image upgrade process occurs by copying new IOS files into the router and then booting the router with that 
            new IOS."
        }

        Accordion { class: "mb-4",
            AccordionItem {
                AccordionTrigger { {h3_heading("The IOS File System")} }
                AccordionContent {
                    p { class: "mb-4",
                        strong { "Every OS creates file systems to store files." }
                        br {}
                        "A computer needs some type of permanent storage, but it needs more than just a place to store bytes."
                        br {}
                        strong {
                            "The OS organizes the storage into a file system, which includes directories, structure, and filenames, with 
                        the associated rules."
                        }
                        br {}
                        "By using a file system, the OS can keep data organized so the user and the applications can
                        find the data later."
                    }

                    {h4_heading("File systems conventions")}
                    p { class: "mb-4",
                        strong { "Every OS defines its own file system conventions." }
                        br {}
                        "Windows OSs, for instance, use a leftleaning slash (\\) in 
                        directory structures, like \\Desktop\\Applications."
                        br {}
                        "Linux and macOS use a right-leaning slash, for example, /Desktop."
                        br {}
                        "Each OS refers to physical disks slightly differently as well, and IOS is no different."
                    }

                    {h4_heading("Flash memory over HDD")}
                    p { class: "mb-4",
                        "As for the physical storage, Cisco routers typically use flash memory, with no hard disk drive."
                        br {}
                        strong {
                            "Flash memory is rewriteable, permanent storage, which is ideal for storing files that
                            need to be retained when the router loses power."
                        }
                        br {}
                        "Cisco purposefully uses flash memory rather than hard disk drives in its products because there are no 
                        moving parts in flash memory, so there is a smaller chance of failure as compared with disk drives."
                        br {}
                        strong { "Some routers have flash memory on the motherboard." }
                        br {}
                        "Others have flash memory slots that allow easy removal and replacement of the flash card, but with the 
                        intent that the card remain in the device most of the time."
                        br {}
                        strong { "Also, many devices have USB slots that support USB flash drives." }
                    }

                    {h4_heading("Output of the show file systems command")}
                    p { class: "mb-4",
                        strong {
                            "For each physical memory device in the router, IOS creates a simple IOS file system and gives that device 
                            a name."
                        }
                        br {}
                        "Example 12-1 lists the surprisingly long list of IOS file systems."
                        br {}
                        strong {
                            "Note that entries of type disk and usbflash are the physical storage devices in that router."
                        }
                        br {}
                        "In this case, the router has one of two of the 2901's compact flash slots populated with a 256-MB flash 
                        card and one of the two USB flash slots populated with an 8-GB USB flash drive."
                        br {}
                        "Look at the size column and prefixes column in the output to find these devices, based on their types as 
                        disk and usbflash "
                    }

                    p { class: "mb-4", "See Example 12-1 in volume 2 on page 369." }

                    {h4_heading("Types of IOS file systems")}
                    p { class: "mb-1",
                        "The example lists 20 different IOS file systems in this case, but the router does not have 20
                        different physical storage devices."
                        br {}
                        "Instead, IOS uses these file systems for other purposes as well, with these types "
                    }
                    ol { class: "list-disc list-inside mb-4",
                        li {
                            span { class: "font-bold mr-1", "Opaque:" }
                            "To represent logical internal file systems for the convenience of internal functions and commands"
                        }
                        li {
                            span { class: "font-bold mr-1", "Network:" }
                            "To represent external file systems found on different types of servers for the convenience of 
                            reference in different IOS commands"
                        }
                        li {
                            span { class: "font-bold mr-1", "Disk:" }
                            "For flash"
                        }
                        li {
                            span { class: "font-bold mr-1", "Usbflash:" }
                            "For USB flash"
                        }
                        li {
                            span { class: "font-bold mr-1", "NVRAM:" }
                            "A special type for NVRAM memory, the default location of the startup-config file"
                        }
                    }

                    {h4_heading("Formal vs Informal filenames")}
                    p { class: "mb-1",
                        "Many IOS commands refer to files in an IFS, but only some commands refer directly to the files by their 
                        formal names."
                        br {}
                        strong {
                            "The formal names use the prefix as seen in the far right column of Example 12-1."
                        }
                        br {}
                        "For instance, the command "
                        {text_command("more flash0:/wotemp/fred", TextCommandColor::Gold)}
                        " would display the contents of file "
                        i { "fred" }
                        " in directory "
                        i { "/wotemp" }
                        " in the first flash memory slot in the router."
                        br {}
                        "(The "
                        {text_command("more", TextCommandColor::Gold)}
                        " command itself displays the contents of a file.)"
                        br {}
                        "However, many commands use a keyword that indirectly refers to a formal filename, to reduce typing."
                        br {}
                        "For example:"
                    }
                    ol { class: "list-disc list-inside mb-4",
                        li {
                            span { class: "font-bold mr-1", "show running-config" }
                            "command: Refers to file system:running-config"
                        }
                        li {
                            span { class: "font-bold mr-1", "show startup-config" }
                            "command: Refers to file nvram:startup-config"
                        }
                        li {
                            span { class: "font-bold mr-1", "show flash" }
                            "command: Refers to default flash IFS (usually flash0:)"
                        }
                    
                    }
                }
            }
            AccordionItem {
                AccordionTrigger { {h3_heading("Upgrading IOS Images")} }
                AccordionContent {
                    p { class: "mb-4",
                        strong {
                            "One of the first steps to upgrade a router's IOS to a new version is to obtain the new IOS
                            image and put it in the right location."
                        }
                        br {}
                        "Typically, Cisco routers have their IOS in one of the local physical file systems, most often in permanent 
                        flash."
                        br {}
                        "The only requirement is that the IOS be in some reachable file system—even if the file sits on an external 
                        server and the device loads the OS over the network."
                        br {}
                        strong {
                            "However, the best practice is to store each device's IOS file in flash that will remain with the device 
                            permanently."
                        }
                    }

                    p { class: "mb-1",
                        "Figure 12-13 illustrates the process to upgrade an IOS image into flash memory, using the following steps:"
                    }
                    ol { class: "mb-4",
                        li { class: "flex gap-x-4",
                            span { class: "font-bold text-blue-500 shrink-0", "Step 1." }
                            span {
                                "Obtain the IOS image from Cisco, usually by downloading the IOS image from Cisco.com using HTTP or 
                                FTP."
                            }
                        }
                        li { class: "flex gap-x-4",
                            span { class: "font-bold text-blue-500 shrink-0", "Step 2." }
                            span {
                                "Place the IOS image someplace that the router can reach. Locations include TFTP or FTP servers in 
                                the network or a USB flash drive that is then inserted into the router."
                            }
                        }
                        li { class: "flex gap-x-4",
                            span { class: "font-bold text-blue-500 shrink-0", "Step 3." }
                            span {
                                "Issue the "
                                {text_command("copy", TextCommandColor::Gold)}
                                " command from the router, copying the file into the flash memory that usually remains with the 
                                router on a permanent basis. ("
                                strong {
                                    "Routers usually cannot boot from the IOS image in a USB flash drive"
                                }
                                ".)"
                            }
                        }
                    }

                    KeyTopic {}
                    img {
                        class: "mb-4 rounded-lg",
                        loading: "lazy",
                        alt: "Figure 12-13 Copying an IOS Image as Part of the Cisco IOS Software Upgrade Process",
                        src: asset!("/assets/static/v2p3c12s3sh1f12-13.png", AssetOptions::image().with_avif()),
                    }
                }
            }
            AccordionItem {
                AccordionTrigger { {h3_heading("Copying a New IOS Image to a Local IOS File System Using TFTP")} }
                AccordionContent {
                    p { class: "mb-4",
                        "Example 12-2 provides an example of step 3 from Figure 12-13, copying the IOS image into
                        flash memory."
                        br {}
                        "In this case, router R2, a 2901, copies an IOS image from a TFTP server at IP address 2.2.2.1."
                    }

                    KeyTopic {}
                    p { class: "mb-4", "See Example 12-2 in volume 1 on page 271." }

                    {h4_heading("The copy command")}
                    p { class: "mb-1",
                        "The "
                        {text_command("copy", TextCommandColor::Gold)}
                        " command does a simple task—copy a file—but the command also has several small items to check."
                        br {}
                        "It needs a few pieces of information from the user, so the command prompts the user for that information 
                        by showing the user some text and waiting for the user's input."
                        br {}
                        "The bold items in the example show the user's input."
                        br {}
                        "The router then has to check to make sure the copy will work."
                        br {}
                        "The command works through these kinds of questions:"
                    }
                    ol { class: "list-decimal list-inside pl-3 mb-4",
                        li { "What is the IP address or host name of the TFTP server?" }
                        li { "What is the name of the file?" }
                        li {
                            "Ask the server to learn the size of the file, and then check the local router's flash to
                            ask whether enough space is available for this file in flash memory."
                        }
                        li { "Does the server actually have a file by that name?" }
                        li { "Do you want the router to erase any old files in flash?" }
                    }

                    p { class: "mb-4",
                        "The router prompts you for answers to some of these questions, as necessary."
                        br {}
                        "For each question, you should either type an answer or press Enter if the default answer (shown in
                        square brackets at the end of the question) is acceptable."
                        br {}
                        strong {
                            "Afterward, the router erases flash memory if directed, copies the file, and then verifies that the 
                            checksum for the file shows that no errors occurred in transmission."
                        }
                    }

                    GreenNote {
                        p {
                            strong { "NOTE" }
                            "Most people use the IOS filenames that Cisco supplies because these names embed information about the 
                            IOS image, like the version. Also, if you want to use the same destination filename as the source, avoid 
                            the mistake of typing “y” or “yes” to confirm the selection; instead, you would be setting the 
                            destination filename to “y” or “yes.” Simply press Enter to confirm the selection listed in brackets."
                        }
                    }

                    {h4_heading("Viewing flash drives content")}
                    p { class: "mb-4",
                        "You can view the contents of the flash file system to see the IOS file that was just copied
                        by using a couple of commands."
                        br {}
                        "The "
                        {text_command("show flash", TextCommandColor::Gold)}
                        " command shows the files in the default flash file system (flash0:), as seen at the top of Example 12-3."
                        br {}
                        "Below it, the more general "
                        {text_command("dir flash0:", TextCommandColor::Gold)}
                        " command lists the contents of that same file system, with similar information."
                        br {}
                        "(You can use the "
                        {text_command("dir", TextCommandColor::Gold)}
                        " command to display the contents of any local IFS.)"
                    }

                    p { class: "mb-4", "See Example 12-3 in volume 1 on page 272." }

                    p { class: "mb-4",
                        "Pay close attention to the memory usage per file and for the IFS as shown in the example."
                        br {}
                        "The output lists the size in bytes for each file."
                        br {}
                        "Note that the IOS file is about 104 MB."
                        br {}
                        "Note that the size of the IOS file matches the size shown earlier in the TFTP transfer in Example 12-2."
                        br {}
                        strong {
                            "The end of each of the commands then lists the amount of space available for new files to be added to 
                            flash (one lists it as “bytes available”; the other as “bytes free”)."
                        }
                        br {}
                        "However, that same ending line of each command shows slightly different information
                        about usage: "
                        {text_command("show flash", TextCommandColor::Gold)}
                        " lists the bytes used, whereas the "
                        {text_command("dir", TextCommandColor::Gold)}
                        " command lists the total bytes (bytes used plus bytes free)."
                        br {}
                        "Play around with the numbers in this example to make sure you know which command lists which particular 
                        total."
                    }
                
                }
            
            }
            AccordionItem {
                AccordionTrigger { {h3_heading("Verifying IOS Code Integrity with MD5")} }
                AccordionContent {}
            }
            AccordionItem {
                AccordionTrigger { {h3_heading("Copying Images with FTP")} }
                AccordionContent {}
            }
        }

        {h3_heading("RECAP")}
        ol { class: "list-disc list-inside",
            li {
                "IOS exists as a file—a single file—that the router then loads into RAM to use as its operating system."
            }
            li {
                "IO file system (IFS) defines how IOS stores files (including the IOS file). By using a file system, the OS can keep data 
                organized so the user and the applications can find the data later."
            }
            li {
                "The IOS image upgrade process occurs by copying new IOS files into the router and then booting the router with that new IOS."
            }
            li { "Cisco uses flash memory over hard disk drive for its permanent physical storage." }
            li {
                "Some cisco devices have flash memory on the motherboard, others have flash memory slots that allow easy removal 
                and replacement of the flash card, but with the intent that the card remain in the device most of the time."
            }
            li {
                "For each physical memory device in the router, IOS creates a simple IOS file system and gives that device a name."
            }
            li {
                "For each physical memory device in the router, IOS creates a simple IOS file system and gives that device a name."
            }
            li { "IOS commands can reference filenames with their formal or informal names." }
        }
    }
}