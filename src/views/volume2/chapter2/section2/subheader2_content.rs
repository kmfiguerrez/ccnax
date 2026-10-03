use dioxus::prelude::*;

use crate::{
    components::{
        KeyTopic,
        GreenNote,
        my_accordion::{Accordion, AccordionContent, AccordionItem, AccordionTrigger}
    }, utils::{TextCommandColor, h3_heading, h4_heading, text_command}
};

#[component]
pub fn Content() -> Element {
    rsx! {
        p { class: "mb-4", "Standard numbered IP ACLs use the following global command:" }

        p { class: "pl-4 mb-4",
            {text_command("access-list", TextCommandColor::Gold)}
            " {{1-99 | 1300-1999}} {{"
            {text_command("permit", TextCommandColor::Gold)}
            " | "
            {text_command("deny", TextCommandColor::Gold)}
            "}} "
            i { "matching-parameters" }
        }

        p { class: "mb-4",
            "Each standard numbered ACL has one or more "
            {text_command("access-list", TextCommandColor::Gold)}
            " commands with the same number, any number from the ranges shown 
            in the preceding line of syntax."
            br {}
            "(One number is no better than the other.)"
            br {}
            strong {
                "IOS refers to each line in an ACL as an Access Control Entry (ACE), but many engineers just call them ACL statements."
            }
        }

        p { class: "mb-4",
            "Besides the ACL number, each "
            {text_command("access-list", TextCommandColor::Gold)}
            " command also lists the action ("
            {text_command("permit", TextCommandColor::Gold)}
            " or "
            {text_command("deny", TextCommandColor::Gold)}
            "), plus the matching logic."
            br {}
            "The rest of this section examines how to configure the matching parameters, which, for standard ACLs, means that you 
            can only match the source IP address or portions of the source IP address using something called an ACL wildcard mask."
        }

        Accordion { class: "mb-4",
            AccordionItem {
                AccordionTrigger { {h3_heading("Matching the Exact IP Address")} }
                AccordionContent {
                    p { class: "mb-4",
                        "To match a specific source IP address, the entire IP address, all you have to do is type that
                        IP address at the end of the command."
                        br {}
                        "For example, the previous example uses pseudocode for “permit if source = 10.1.1.1.”"
                        br {}
                        "The following command configures that logic with correct syntax using ACL number 1:"
                    }

                    p { class: "pl-4 mb-4",
                        {text_command("access-list 1 permit 10.1.1.1", TextCommandColor::Gold)}
                    }

                    p { class: "mb-4", "Matching the exact full IP address is that simple." }

                    p { class: "mb-4",
                        "In earlier IOS versions, the syntax included a "
                        {text_command("host", TextCommandColor::Gold)}
                        " keyword."
                        br {}
                        "Instead of simply typing the full IP address, you first typed the "
                        {text_command("host", TextCommandColor::Gold)}
                        " keyword and then the IP address."
                        br {}
                        "Note that in later IOS versions, if you use the "
                        {text_command("host", TextCommandColor::Gold)}
                        " keyword, IOS accepts the command but then removes 
                        the keyword."
                    }

                    p { class: "pl-4 mb-4",
                        {text_command("access-list 1 permit host 10.1.1.1", TextCommandColor::Gold)}
                    }
                }
            }
            AccordionItem {
                AccordionTrigger { {h3_heading("Matching a Subset of the Address with Wildcards")} }
                AccordionContent {
                    p { class: "mb-4",
                        "Often , the business goals you want to implement with an ACL do not match a single particular IP address, 
                        but rather a range of IP addresses."
                        br {}
                        "Maybe you want to match all IP addresses in a subnet."
                        br {}
                        "Maybe you want to match all IP addresses in a range of subnets."
                        br {}
                        "Regardless, you want to check for more than one IP address in a range of addresses."
                    }

                    {h4_heading("Wildcard Mask")}
                    p { class: "mb-1",
                        "IOS allows standard ACLs to match a range of addresses using a tool called a "
                        i { "wildcard mask." }
                        br {}
                        strong { "Note that this is not a subnet mask." }
                        br {}
                        "The wildcard mask (which this book abbreviates as WC mask) gives the engineer a way to tell IOS to ignore 
                        parts of the address when making comparisons, essentially treating those parts as wildcards, as if they 
                        already matched."
                        br {}
                        "You can think about WC masks in decimal and in binary, and both have their uses."
                        br {}
                        "To begin, think about WC masks in decimal, using these rules:"
                    }
                    KeyTopic {}
                    ol { class: "mb-4",
                        li {
                            span { class: "font-bold", "Decimal 0" }
                            ": The router must compare this octet as normal."
                        }
                        li {
                            span { class: "font-bold", "Decimal 255" }
                            ": The router ignores this octet, considering it to already match."
                        }
                    }

                    p { class: "mb-4",
                        "Keeping these two rules in mind, consider Figure 2-6, which demonstrates this logic using
                        three different but popular WC masks: one that tells the router to ignore the last octet, one
                        that tells the router to ignore the last two octets, and one that tells the router to ignore the
                        last three octets."
                    }

                    img {
                        class: "mb-4 rounded-lg",
                        loading: "lazy",
                        alt: "Figure 2-6 Logic for WC Masks 0.0.0.255, 0.0.255.255, and 0.255.255.255",
                        src: asset!("/assets/static/v2p1c2s2sh2f2-6.png", AssetOptions::image().with_avif()),
                    }

                    p { class: "mb-4",
                        "All three examples in the boxes of Figure 2-6 show two numbers that are clearly different."
                        br {}
                        strong {
                            "The WC mask causes IOS to compare only some of the octets, while ignoring other octets."
                        }
                        br {}
                        "All three examples result in a match, because each wildcard mask tells IOS to ignore some
                        octets."
                        br {}
                        "The example on the left shows WC mask 0.0.0.255, which tells the router to treat
                        the last octet as a wildcard, essentially ignoring that octet for the comparison."
                        br {}
                        "Similarly, the middle example shows WC mask 0.0.255.255, which tells the router to ignore the two
                        octets on the right."
                        br {}
                        "The rightmost case shows WC mask 0.255.255.255, telling the router to
                        ignore the last three octets when comparing values."
                    }

                    {h4_heading("Wildcard Mask in action")}
                    p { class: "mb-4",
                        "To see the WC mask in action, think back to the earlier example related to Figure 2-4 and
                        Figure 2-5."
                        br {}
                        "The pseudocode ACL in those two figures used logic that can be created using a WC mask."
                        br {}
                        "As a reminder, the logic in the pseudocode ACL in those two figures included the following:"
                    }
                    ol { class: "sm:pl-4 mb-4",
                        li {
                            span { class: "font-bold", "Line 1" }
                            ": Match and permit all packets with a source address of exactly 10.1.1.1."
                        }
                        li {
                            span { class: "font-bold", "Line 2" }
                            ": Match and deny all packets with source addresses with first three octets 10.1.1."
                        }
                        li {
                            span { class: "font-bold", "Line 3" }
                            ":  Match and permit all addresses with first single octet 10."
                        }
                    }

                    p { class: "mb-4",
                        "Figure 2-7 shows the updated version of Figure 2-4, but with the completed, correct syntax,
                        including the WC masks."
                        br {}
                        "In particular, note the use of WC mask 0.0.0.255 in the second
                        command, telling R2 to ignore the last octet of the number 10.1.1.0, and the WC mask
                        0.255.255.255 in the third command, telling R2 to ignore the last three octets in the value
                        10.0.0.0."
                    }

                    p { class: "mb-4",
                        "Finally, note that when using a WC mask, the "
                        {text_command("access-list", TextCommandColor::Gold)}
                        " command's loosely defined
                        source parameter should be a 0 in any octets where the WC mask is a 255."
                        br {}
                        strong {
                            "IOS will specify a source address to be 0 for the parts that will be ignored, even if nonzero values 
                            were configured."
                        }
                    }

                    img {
                        class: "mb-4 rounded-lg",
                        loading: "lazy",
                        alt: "Figure 2-7 Syntactically Correct ACL Replaces Pseudocode from Figure 2-4",
                        src: asset!("/assets/static/v2p1c2s2sh2f2-7.png", AssetOptions::image().with_avif()),
                    }
                
                }
            }
            AccordionItem {
                AccordionTrigger { {h3_heading("Binary Wildcard Masks")} }
                AccordionContent {
                    p { class: "mb-4",
                        strong {
                            "Wildcard masks, as dotted-decimal number (DDN) values, actually represent a 32-bit binary number."
                        }
                        br {}
                        "As a 32-bit number, the WC mask actually directs the router's logic bit by bit."
                        br {}
                        "In short, a WC mask bit of 0 means the comparison should be done as normal, but a binary 1
                        means that the bit is a wildcard and can be ignored when comparing the numbers."
                    }

                    p { class: "mb-4",
                        strong {
                            "Thankfully, for the purposes of CCNA study, and for most real-world applications, you can
                            ignore the binary WC mask."
                        }
                        br {}
                        "Why? Well, we generally want to match a range of addresses
                        that can be easily identified by a subnet number and mask, whether it be a real subnet, or
                        a summary route that groups subnets together."
                        br {}
                        "If you can describe the range of addresses with a subnet number and mask, you can find the numbers to use 
                        in your ACL with some simple decimal math, as discussed next "
                    }

                    GreenNote {
                        p {
                            strong { "NOTE" }
                            " If you really want to know the binary mask logic, take the two DDN numbers
                            the ACL will compare (one from the "
                            {text_command("access-list", TextCommandColor::Black)}
                            " command and the other from the packet
                            header) and convert both to binary. Then, also convert the WC mask to binary. Compare
                            the first two binary numbers bit by bit, but also ignore any bits for which the WC mask
                            happens to list a binary 1, because that tells you to ignore the bit. If all the bits you checked
                            are equal, it's a match!"
                        }
                    }
                }
            }
            AccordionItem {
                AccordionTrigger { {h3_heading("Finding the Right Wildcard Mask to Match a Subnet")} }
                AccordionContent {
                    p { class: "mb-1",
                        "In many cases, an ACL needs to match all hosts in a particular subnet."
                        br {}
                        "To match a subnet with an ACL, you can use the following shortcut:"
                    }
                    KeyTopic {}
                    ol { class: "list-disc list-inside mb-4",
                        li {
                            "Use the subnet number as the source value in the "
                            {text_command("access-list", TextCommandColor::Gold)}
                            " command."
                        }
                        li {
                            "Use a wildcard mask found by subtracting the subnet mask from 255.255.255.255."
                        }
                    }

                    p { class: "mb-4",
                        "For example, for subnet 172.16.8.0 255.255.252.0, use the subnet number (172.16.8.0) as
                        the address parameter, and then do the following math to find the wildcard mask:"
                    }

                    img {
                        class: "mb-4 rounded-lg",
                        loading: "lazy",
                        alt: "Wildcard mask computation sample",
                        src: asset!(
                            "/assets/static/v2p1c2s2sh2wc-formula.png", AssetOptions::image().with_avif()
                        ),
                    }

                    p { class: "mb-4",
                        "Continuing this example, a completed command for this same subnet would be as follows:"
                    }

                    p { class: "mb-4",
                        {
                            text_command("access-list 1 permit 172.16.8.0 0.0.3.255", TextCommandColor::Gold)
                        }
                    }

                    p { class: "mb-4",
                        "The section “Practice Applying Standard IP ACLs” gives you a chance to practice matching
                        subnets when configuring ACLs."
                    }
                }
            }
            AccordionItem {
                AccordionTrigger { {h3_heading("Matching Any/All Addresses")} }
                AccordionContent {
                    p { class: "mb-4",
                        "In some cases, you will want one ACL command to match any and all packets that reach 
                        that point in the ACL."
                        br {}
                        "First, you have to know the (simple) way to match all packets using the "
                        {text_command("any", TextCommandColor::Gold)}
                        " keyword."
                        br {}
                        "More importantly, you need to think about when to match any and all packets."
                    }

                    p { class: "mb-4",
                        "First, to match any and all packets with an ACL command, just use the "
                        {text_command("any", TextCommandColor::Gold)}
                        " keyword for the address."
                        br {}
                        "For example, to permit all packets:"
                    }

                    p { class: "pl-4 mb-4",
                        {text_command("access-list 1 permit any", TextCommandColor::Gold)}
                    }

                    {h4_heading("Overriding the deny any at the end")}
                    p { class: "mb-4",
                        "So, when and where should you use such a command?"
                        br {}
                        "Remember, all Cisco IP ACLs end with an implicit "
                        {text_command("deny any", TextCommandColor::Gold)}
                        " concept at the end of each ACL."
                        br {}
                        "That is, if a router compares a packet to the ACL, and the packet matches none of the configured 
                        statements, the router discards the packet."
                        br {}
                        "Want to override that default behavior?"
                        br {}
                        "Configure a "
                        {text_command("permit any", TextCommandColor::Gold)}
                        " at the end of the ACL."
                    }

                    {h4_heading("Reason to explicitly configure deny any at the end")}
                    p { class: "mb-4",
                        "You might also want to explicitly configure a command to deny all traffic (for example, "
                        {text_command("access-list 1 deny any", TextCommandColor::Gold)}
                        ") at the end of an ACL."
                        br {}
                        "Why, when the same logic already sits at the end of the ACL anyway?"
                        br {}
                        "Well, the ACL "
                        {text_command("show", TextCommandColor::Gold)}
                        " commands list counters for the number of packets matched by each command in the ACL, 
                        but there is no counter for that implicit "
                        {text_command("deny any", TextCommandColor::Gold)}
                        " concept at the end of the ACL."
                        br {}
                        "So, if you want to see counters for how many packets are matched by the "
                        {text_command("deny any", TextCommandColor::Gold)}
                        " logic at the end of the 
                        ACL, configure an explicit "
                        {text_command("deny any", TextCommandColor::Gold)}
                        "."
                    }
                }
            }
        }

        KeyTopic {}
        p { class: "mb-4",
            "In short, to filter a packet, you must enable an ACL on an interface that processes the packet, in the same direction 
            the packet flows through that interface."
        }

        {h3_heading("RECAP")}
        ol { class: "list-disc list-inside",
            li {
                "IOS refers to each line in an ACL as an Access Control Entry (ACE), but many engineers just call them ACL statements."
            }
            li {
                "IOS allows standard ACLs to match a range of addresses using a tool called a "
                i { "wildcard mask." }
            }
            li { "Note that a wildcard mask is not a subnet mask." }
            li {
                "The wildcard mask gives the engineer a way to tell IOS to ignore parts of  the address when making comparisons, 
                essentially treating those parts as wildcards, as if they already matched."
            }
            li {
                "Finally, note that when using a WC mask, the "
                {text_command("access-list", TextCommandColor::Gold)}
                " command's loosely defined source parameter should be a 0 in any octets where the WC mask is a 255."
                " IOS will specify a source address to be 0 for the parts that will be ignored, even if nonzero values were configured."
            }
            li {
                "Wildcard masks, as dotted-decimal number (DDN) values, actually represent a 32-bit binary number."
            }
            li {
                "Remember, all Cisco IP ACLs end with an implicit "
                {text_command("deny any", TextCommandColor::Gold)}
                " concept at the end of each ACL."
            }
            li {
                "Only configured commands in the ACL will have counters in the "
                {text_command("show", TextCommandColor::Gold)}
                " command for ACL. So, you might also want to explicitly configure a command to deny all traffic at the end of an 
                ACL even though there's already one in place by default."
            }
        }
    }
}