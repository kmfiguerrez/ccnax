use dioxus::prelude::*;

use crate::{components::{KeyTopic,ConfigChecklist}, utils::{TextCommandColor, h3_heading, text_command}};

#[component]
pub fn Content() -> Element {
    rsx! {
        p { class: "mb-4",
            "The static and dynamic NAT configurations matter, but the NAT overload (PAT) configuration in this section matters more."
            br {}
            "This is the feature that saves public IPv4 addresses and prolonged IPv4's life."
        }

        p { class: "mb-4",
            "NAT overload, as mentioned earlier, allows NAT to support many inside local IP addresses with only one or a few inside 
            global IP addresses."
            br {}
            "By essentially translating the private IP address and port number to a single inside global address, but with a unique 
            port number, NAT can support many (more than 65,000) private hosts with only a single public, global address."
        }

        {h3_heading("Two variations of PAT")}
        p { class: "mb-1",
            strong { "Two variations of PAT configuration exist in IOS." }
        
        }
        ol { class: "list-disc list-inside mb-1",
            li {
                "If PAT uses a pool of inside global addresses, the configuration looks exactly like dynamic NAT, except the "
                {text_command("ip nat inside source list", TextCommandColor::Gold)}
                " global command has an "
                {text_command("overload", TextCommandColor::Gold)}
                " keyword added to the end."
            }
            li {
                "If PAT just needs to use one inside global IP address, the router can use one of its interface IP addresses."
            }
        }
        p { class: "mb-4",
            "Because NAT can support over 65,000 concurrent flows with a single inside global address,
            a single public IP address can support an entire organization's NAT needs."
        }

        KeyTopic {}
        p { class: "mb-4",
            "The following statement details the configuration difference between NAT overload and 1:1 NAT when using a NAT pool:"
        }
        p { class: "pl-5 mb-4",
            "Use the same steps for configuring dynamic NAT, as outlined in the previous section, but
            include the "
            {text_command("overload", TextCommandColor::Gold)}
            " keyword at the end of the "
            {text_command("ip nat inside source list", TextCommandColor::Gold)}
            " global command."
        }

        ConfigChecklist {}
        p { class: "mb-1",
            "The following checklist details the configuration when using an interface IP address as the
            sole inside global IP address:"
        }

        ol { class: "mb-4",
            li { class: "flex gap-x-4",
                span { class: "text-blue-500 font-bold shrink-0", "Step 1." }
                span {
                    "As with dynamic and static NAT, configure the "
                    {text_command("ip nat inside", TextCommandColor::Gold)}
                    "  interface subcommand to identify inside interfaces."
                }
            }
            li { class: "flex gap-x-4",
                span { class: "text-blue-500 font-bold shrink-0", "Step 2." }
                span {
                    "As with dynamic and static NAT, configure the "
                    {text_command("ip nat outside", TextCommandColor::Gold)}
                    " interface subcommand to identify outside interfaces."
                }
            }
            li { class: "flex gap-x-4",
                span { class: "text-blue-500 font-bold shrink-0", "Step 3." }
                span {
                    "As with dynamic NAT, configure an ACL that matches the packets entering inside interfaces."
                }
            }
            li { class: "flex gap-x-4",
                span { class: "text-blue-500 font-bold shrink-0", "Step 4." }
                span {
                    "Configure the "
                    {text_command("ip nat inside source list", TextCommandColor::Gold)}
                    i { " acl-number " }
                    {text_command("interface", TextCommandColor::Gold)}
                    i { " type/number " }
                    {text_command("overload", TextCommandColor::Gold)}
                    " global configuration command, referring to the ACL created in step 3 and to the interface whose IP address 
                    will be used for translations."
                }
            }
        }

        {h3_heading("Converting Dynamic NAT into PAT")}
        p { class: "mb-4",
            "Example 10-2 demonstrated a dynamic NAT configuration."
            br {}
            "To convert it to a PAT configuration, you would use the "
            {
                text_command(
                    "ip nat inside source list 1 pool fred overload",
                    TextCommandColor::Gold,
                )
            }
            " command instead, simply adding the "
            {text_command("overload", TextCommandColor::Gold)}
            " keyword."
        }

        {h3_heading("Examining PAT configuration")}
        p { class: "mb-4",
            "The next example shows PAT configuration using a single interface IP address."
            br {}
            "Figure 10-10 shows the same familiar network, with a few changes."
            br {}
            "In this case, the ISP has given Certskills a subset of network 200.1.1.0: CIDR subnet 200.1.1.248/30."
            br {}
            "In other words, this subnet has two usable addresses: 200.1.1.249 and 200.1.1.250."
            br {}
            "These addresses are used on either end of the serial link between Certskills and its ISP."
            br {}
            "The NAT feature on the Certskills router translates all NAT addresses to its serial IP address, 200.1.1.249."
        }

        img {
            class: "mb-4 rounded-lg",
            loading: "lazy",
            alt: "Figure 10-10 NAT Overload and PAT",
            src: asset!("/assets/static/v2p3c10s3sh4f10-10.png", AssetOptions::image().with_avif()),
        }

        p { class: "mb-4",
            "In Example 10-6, which shows the NAT overload configuration, NAT translates using inside
            global address 200.1.1.249 only, so the NAT pool is not required."
            br {}
            "In the example, host 10.1.1.2 creates two Telnet connections, and host 10.1.1.1 creates one Telnet connection,
            causing three dynamic NAT entries, each using inside global address 200.1.1.249, but each
            with a unique port number."
        }

        p { class: "mb-4", "See Example 10-6 in volume 1 on page 221." }

        {h3_heading("PAT command structure")}
        p { class: "mb-4",
            "The "
            {
                text_command(
                    "ip nat inside source list 1 interface serial 0/0/0 overload",
                    TextCommandColor::Gold,
                )
            }
            " command has several parameters, but if you understand the dynamic NAT configuration, the new parameters shouldn't be 
            too hard to grasp."
            br {}
            "The "
            {text_command("list 1", TextCommandColor::Gold)}
            " parameter means the same thing as it does for dynamic NAT: inside local IP addresses matching ACL 1 have 
            their addresses translated."
            br {}
            "The "
            {text_command("interface serial 0/0/0", TextCommandColor::Gold)}
            " parameter means that the only inside global IP address available is the
            IP address of the NAT router's interface serial 0/0/0."
            br {}
            "Finally, the "
            {text_command("overload", TextCommandColor::Gold)}
            " parameter means that overload is enabled."
            br {}
            "Without this parameter, the router does not perform overload, just dynamic NAT."
        }

        {h3_heading("NAT show commands")}
        p { class: "mb-4",
            "As you can see in the output of the "
            {text_command("show ip nat translations", TextCommandColor::Gold)}
            " command, three translations have been added to the NAT table."
            br {}
            "Before this command, host 10.1.1.1 creates one Telnet connection to 170.1.1.1, and host 10.1.1.2 creates two Telnet 
            connections."
            br {}
            "The router creates one NAT table entry for each unique combination of inside local IP address and port."
        }

        {h3_heading("RECAP")}
        ol { class: "list-disc list-inside",
            li {
                "NAT overload, translates the private IP addresses and port numbers to a single inside global address, but with a 
                unique port number, NAT can support many (more than 65,000) private hosts with only a single public, 
                global address."
            }
            li {
                "Because NAT can support over 65,000 concurrent flows with a single inside global address, a single public IP 
                address can support an entire organization's NAT needs."
            }
            li {
                "Without the "
                {text_command("overload", TextCommandColor::Gold)}
                " parameter in the PAT command, the router does not perform overload, just dynamic NAT."
            }
        }
    }
}