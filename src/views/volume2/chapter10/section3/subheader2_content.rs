use dioxus::prelude::*;

use crate::{components::ConfigChecklist, utils::{TextCommandColor, h3_heading, text_command}};

#[component]
pub fn Content() -> Element {
    rsx! {
        p { class: "mb-1",
            "As you might imagine, dynamic NAT configuration differs in some ways from static NAT,
            but it has some similarities as well."
            br {}
            "Dynamic NAT still requires that each interface be identified as either an inside or outside interface, and of course 
            static mapping is no longer required."
            br {}
            "Dynamic NAT uses an access control list (ACL) to identify which inside local (private) IP addresses need to have their 
            addresses translated, and it defines a pool of registered public IP addresses to allocate."
            br {}
            "The specific steps are as follows:"
        }
        ConfigChecklist {}
        ol { class: "mb-4",
            li { class: "flex gap-x-4",
                span { class: "text-blue-500 font-bold shrink-0", "Step 1." }
                span {
                    "Use the "
                    {text_command("ip nat inside", TextCommandColor::Gold)}
                    " command in interface configuration mode to configure interfaces to be in the inside part of the NAT design 
                    (just like with static NAT)."
                }
            }
            li { class: "flex gap-x-4",
                span { class: "text-blue-500 font-bold shrink-0", "Step 2." }
                span {
                    "Use the "
                    {text_command("ip nat outside", TextCommandColor::Gold)}
                    " command in interface configuration mode to configure interfaces to be in the outside part of the NAT design 
                    (just like with static NAT)."
                }
            }
            li { class: "flex gap-x-4",
                span { class: "text-blue-500 font-bold shrink-0", "Step 3." }
                span {
                    "Configure an ACL that matches the packets entering inside interfaces for
                    which NAT should be performed."
                }
            }
            li { class: "flex gap-x-4",
                span { class: "text-blue-500 font-bold shrink-0", "Step 4." }
                span {
                    "Use the "
                    {text_command("ip nat pool", TextCommandColor::Gold)}
                    i { " name first-address last-address " }
                    {text_command("netmask", TextCommandColor::Gold)}
                    i { " subnet-mask " }
                    "command in global configuration mode to configure the pool of public registered IP addresses."
                }
            }
            li { class: "flex gap-x-4",
                span { class: "text-blue-500 font-bold shrink-0", "Step 5." }
                span {
                    "Use the "
                    {text_command("ip nat inside source list", TextCommandColor::Gold)}
                    i { " acl-number " }
                    {text_command("pool", TextCommandColor::Gold)}
                    i { " pool-name " }
                    "command in global configuration mode to enable dynamic NAT. 
                    Note the command references the ACL (step 3) and pool (step 4) per previous steps."
                }
            }
        }

        img {
            class: "mb-4 rounded-lg",
            loading: "lazy",
            alt: "Figure 10-9 Sample Network for NAT Examples, with Public Class C 200.1.1.0/24",
            src: asset!("/assets/static/v2p3c10s3sh1f10-9.png", AssetOptions::image().with_avif()),
        }

        p { class: "mb-4",
            "The next example shows a sample dynamic NAT configuration using the same network
            topology as the previous example (see Figure 10-9)."
            br {}
            "In this case, the same two inside local addresses—10.1.1.1 and 10.1.1.2—need translation."
            br {}
            "However, unlike the previous static NAT example, the configuration in Example 10-2 places the public IP addresses 
            (200.1.1.1 and 200.1.1.2) into a pool of dynamically assignable inside global addresses."
        }

        img {
            class: "mb-4 rounded-lg",
            loading: "lazy",
            alt: "Example 10-2 Dynamic NAT Configuration",
            src: asset!("/assets/static/v2p3c10s3sh2ex10-2.png", AssetOptions::image().with_avif()),
        }

        {h3_heading("Pool of public addresses")}
        p { class: "mb-4",
            "Dynamic NAT configures the pool of public (global) addresses with the "
            {text_command("ip nat pool", TextCommandColor::Gold)}
            " command listing the first and last numbers in an inclusive range of inside global addresses."
            br {}
            "For example, if the pool needed 10 addresses, the command might have listed 200.1.1.1 and
            200.1.1.10, which means that NAT can use 200.1.1.1 through 200.1.1.10."
        }

        {h3_heading("The netmask parameter")}
        p { class: "mb-4",
            "Dynamic NAT also performs a verification check on the "
            {text_command("ip nat pool", TextCommandColor::Gold)}
            " command with the required "
            {text_command("netmask", TextCommandColor::Gold)}
            " parameter."
            br {}
            "If the address range would not be in the same subnet, assuming the configured netmask was used on the addresses in the 
            configured range, then IOS will reject the "
            {text_command("ip nat pool", TextCommandColor::Gold)}
            " command."
            br {}
            "For example, as configured with the low end of 200.1.1.1, high end of 200.1.1.2, and a mask of 255.255.255.252, IOS 
            would use the following checks, to ensure that both calculations put 200.1.1.1 and 200.1.1.2 in the same subnet:"
        }

        ol { class: "list-disc list-inside mb-4",
            li {
                "200.1.1.1 with mask 255.255.255.252 implies subnet 200.1.1.0, broadcast address 200.1.1.3."
            }
            li {
                "200.1.1.2 with mask 255.255.255.252 implies subnet 200.1.1.0, broadcast address 200.1.1.3"
            }
        }

        p { class: "mb-4",
            "If the command had instead showed low and high end values of 200.1.1.1 and 200.1.1.6,
            again with mask 255.255.255.252, IOS would reject the command."
            br {}
            "IOS would do the math spelled out in the following list, realizing that the numbers were in different subnets:"
        }

        ol { class: "list-disc list-inside mb-4",
            li {
                "200.1.1.1 with mask 255.255.255.252 implies subnet 200.1.1.0, broadcast address 200.1.1.3."
            }
            li {
                "200.1.1.6 with mask 255.255.255.252 implies subnet 200.1.1.4, broadcast address 200.1.1.7."
            }
        }

        {h3_heading("Static vs Dyanamic NAT")}
        p { class: "mb-4",
            "One other big difference between the dynamic NAT and static NAT configuration in
            Example 10-1 has to do with two options in the "
            {text_command("ip nat inside source", TextCommandColor::Gold)}
            " command."
            br {}
            "The dynamic NAT version of this command refers to the name of the NAT pool it wants to use
            for inside global addresses—in this case, fred."
            br {}
            "It also refers to an IP ACL, which defines the matching logic for inside local IP addresses."
            br {}
            "So, the logic for the "
            {text_command("ip nat inside source list 1 pool fred", TextCommandColor::Gold)}
            " command in this example is as follows:"
        }

        p { class: "pl-4 mb-4",
            "Create NAT table entries that map between hosts matched by ACL 1, for packets entering any inside interface, 
            allocating an inside global address from the pool called fred."
        }

        {h3_heading("RECAP")}
        ol { class: "list-disc list-inside",
            li {
                "Dynamic NAT still requires that each interface be identified as either an inside or outside interface, and of 
                course static mapping is no longer required."
            }
            li {
                "Dynamic NAT uses an access control list (ACL) to identify which inside local (private) IP addresses need to have 
                their addresses translated, and it defines a pool of registered public IP addresses to allocate."
            }
            li {
                "Dynamic NAT also performs a verification check on the "
                {text_command("ip nat pool", TextCommandColor::Gold)}
                " command with the required "
                {text_command("netmask", TextCommandColor::Gold)}
                " parameter to make sure that the listed range of addresses checks out with the subnet mask."
            }
        }
    }
}