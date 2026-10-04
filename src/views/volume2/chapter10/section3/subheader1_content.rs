use dioxus::prelude::*;

use crate::{components::ConfigChecklist, utils::{TextCommandColor, h3_heading, text_command}};

#[component]
pub fn Content() -> Element {
    rsx! {
        p { class: "mb-1",
            "Static NAT configuration requires only a few configuration steps."
            br {}
            "Each static mapping between a local (private) address and a global (public) address must be configured."
            br {}
            "In addition, because NAT may be used on a subset of interfaces, the router must be told on which
            interfaces it should use NAT."
            br {}
            "Those same interface subcommands tell NAT whether the interface is inside or outside."
            br {}
            "The specific steps are as follows:"
        }
        ConfigChecklist {}
        ol { class: "mb-4",
            li { class: "flex gap-x-4",
                span { class: "text-blue-500 font-bold shrink-0", "Step 1." }
                span {
                    " Use the "
                    {text_command("ip nat inside", TextCommandColor::Gold)}
                    " command in interface configuration mode to configure
                    interfaces to be in the inside part of the NAT design."
                }
            }
            li { class: "flex gap-x-4",
                span { class: "text-blue-500 font-bold shrink-0", "Step 2." }
                span {
                    " Use the "
                    {text_command("ip nat outside", TextCommandColor::Gold)}
                    " command in interface configuration mode to configure
                    interfaces to be in the outside part of the NAT design."
                }
            }
            li { class: "flex gap-x-4",
                span { class: "text-blue-500 font-bold shrink-0", "Step 3." }
                span {
                    " Use the "
                    {text_command("ip nat inside source static", TextCommandColor::Gold)}
                    i { " inside-local inside-global" }
                    " command in interface configuration mode to configure
                    interfaces to be in the outside part of the NAT design."
                }
            }
        }

        p { class: "mb-4",
            "Figure 10-9 shows the familiar network used in the description of static NAT earlier in this
            chapter, which is also used for the first several configuration examples."
            br {}
            "In Figure 10-9, you can see that Certskills has obtained Class C network 200.1.1.0 as a registered network number."
            br {}
            "That entire network, with mask 255.255.255.0, is configured on the serial link between Certskills and the Internet."
            br {}
            "With a point-to-point serial link, only two of the 254 valid IP addresses in that network are consumed, leaving 252 
            addresses."
        }

        img {
            class: "mb-4 rounded-lg",
            loading: "lazy",
            alt: "Figure 10-9 Sample Network for NAT Examples, with Public Class C 200.1.1.0/24",
            src: asset!("/assets/static/v2p3c10s3sh1f10-9.png", AssetOptions::image().with_avif()),
        }

        {h3_heading("Planning a NAT configuration")}
        p { class: "mb-4",
            "When planning a NAT configuration, you must find some IP addresses to use as inside
            global IP addresses."
            br {}
            "Because these addresses must be part of some registered IP address range, it is common to use the extra addresses in 
            the subnet connecting the enterprise to the Internet—for example, the extra 252 IP addresses in network 200.1.1.0 in 
            this case."
            br {}
            strong {
                "The router can also be configured with a loopback interface and assigned an IP address that is
                part of a globally unique range of registered IP addresses."
            }
        }

        p { class: "mb-4",
            "Example 10-1 lists the NAT configuration, using 200.1.1.1 and 200.1.1.2 for the two static NAT mappings."
        }

        p { class: "mb-4", "See Example 10-1 in volume 1 on page 214." }

        {h3_heading("Static NAT command structure")}
        p {
            "The static mappings are created using the "
            {text_command(" ip nat inside source static", TextCommandColor::Gold)}
            " command."
        }
        ol { class: "list-disc list-inside",
            li {
                "The "
                {text_command("inside", TextCommandColor::Gold)}
                " keyword means that NAT translates addresses for hosts on the inside part of the network."
            }
            li {
                "The "
                {text_command("source", TextCommandColor::Gold)}
                " keyword means that NAT translates the source IP address of packets coming into its inside interfaces."
            }
            li {
                "The "
                {text_command("static", TextCommandColor::Gold)}
                "  keyword means that the parameters define a static entry,
                which should never be removed from the NAT table because of timeout."
            }
        }
        p { class: "mb-4",
            "Because the design calls for two hosts—10.1.1.1 and 10.1.1.2—to have Internet access, two "
            {text_command("ip nat inside", TextCommandColor::Gold)}
            " commands are needed."
        }

        {h3_heading("Inside vs outside interfaces")}
        p { class: "mb-4",
            "After creating the static NAT entries, the router needs to know which interfaces are “inside”
            and which are “outside.”"
            br {}
            "The "
            {text_command("ip nat inside", TextCommandColor::Gold)}
            " and "
            {text_command("ip nat outside", TextCommandColor::Gold)}
            " interface subcommands identify each interface appropriately."
        }

        {h3_heading("The NAT show commands")}
        p { class: "mb-4",
            "A couple of "
            {text_command("show", TextCommandColor::Gold)}
            " commands list the most important information about NAT."
            br {}
            "The "
            {text_command("show ip nat translations", TextCommandColor::Gold)}
            " command lists the two static NAT entries created in the configuration."
            br {}
            "The "
            {text_command("show ip nat statistics", TextCommandColor::Gold)}
            " command lists statistics, listing things such as the number of currently active translation table entries."
            br {}
            "The statistics also include the number of hits, which increments for every packet for which NAT must translate 
            addresses."
        }

        {h3_heading("RECAP")}
        ol { class: "list-disc list-inside",
            li {
                "The router can also be configured with a loopback interface and assigned an IP address that is
                 part of a globally unique range of registered IP addresses."
            }
        }
    }
}