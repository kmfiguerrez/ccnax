use dioxus::prelude::*;

use crate::{
    components::KeyTopic,
    utils::{h3_heading, text_command, TextCommandColor}
};

#[component]
pub fn Content() -> Element {
    rsx! {
        p { class: "mb-4",
            "IOS allows the definition of individual static routes using the "
            {text_command("ip route", TextCommandColor::Gold)}
            " global configuration command."
            br {}
            "Every "
            {text_command("ip route", TextCommandColor::Gold)}
            " command defines a destination that can be matched, usually with a subnet ID and mask."
            br {}
            strong {
                "The command also lists the forwarding instructions, typically listing either the outgoing interface or the next-hop 
                router's IP address."
            }
            br {}
            "IOS then takes that information and adds that route to the IP routing table."
        }

        {h3_heading("Network vs default vs host route")}
        p { class: "mb-4",
            "The static route is considered a network route when the destination listed in the ip route
            command defines a subnet, or an entire Class A, B, or C network."
            br {}
            "In contrast, a default route matches all destination IP addresses, while a host route matches a single IP address 
            (that is, an address of one host.)"
        }

        p { class: "mb-4",
            "As an example of a network route, Figure 16-11 shows a subset of the figure used throughout this chapter so far, 
            with some unrelated details removed."
            br {}
            "The figure shows only the details related to a static network route on R1, for destination subnet 172.16.2.0/24, 
            which sits on the far right."
            br {}
            "To create that static network route on R1, R1 will configure the subnet ID and mask, and either R1's outgoing 
            interface (S0/0/0) or R2 as the next-hop router IP address (172.16.4.2)."
        }

        KeyTopic {}
        img {
            class: "mb-4 rounded-lg",
            alt: "Figure 16-11 Static Route Configuration Concept",
            loading: "lazy",
            src: asset!("/assets/static/v1p5c16s3sh1f16-11.png", AssetOptions::image().with_avif()),
        }

        p { class: "mb-4",
            "Example 16-3 shows the configuration of a couple of sample static routes."
            br {}
            "In particular, it shows routes on Router R1 in Figure 16-12, for the two subnets on the right side of the
            figure."
        }

        img {
            class: "mb-4 rounded-lg",
            alt: "Figure 16-12 Sample Network Used in Static Route Configuration Examples",
            loading: "lazy",
            src: asset!("/assets/static/v1p5c16s3sh1f16-12.png", AssetOptions::image().with_avif()),
        }

        img {
            class: "mb-4 rounded-lg",
            alt: "Example 16-3 Static Routes Added to R1",
            loading: "lazy",
            src: asset!("/assets/static/v1p5c16s3sh1f16-3.png", AssetOptions::image().with_avif()),
        }

        {h3_heading("Forwarding instructions using outgoing interface")}
        p { class: "mb-4",
            "The two example "
            {text_command("ip route", TextCommandColor::Gold)}
            " commands show the two different styles of forwarding instructions."
            br {}
            "The first command shows subnet 172.16.2.0, mask 255.255.255.0, which sits on a LAN near Router R2."
            br {}
            "That same first command lists R1's S0/0/0 interface as the outgoing interface."
            br {}
            strong {
                "This route basically states: To send packets to the subnet off Router R2, send them out my
                own local S0/0/0 interface (which happens to connect to R2)."
            }
        }

        {h3_heading("Forwarding instructions using next hop router")}
        p { class: "mb-4",
            "The second route has the same kind of logic, except for using different forwarding instructions."
            br {}
            strong {
                "Instead of referencing R1's outgoing interface, it instead lists the neighboring router's IP
                address on the WAN link as the next-hop router."
            }
            br {}
            "This route basically says this: To send packets to the subnet off Router R3, send them to R3—specifically, R3's WAN IP 
            address next."
        }

        {h3_heading("Note when the outgoing interface is used")}
        p { class: "mb-4",
            "The routes created by these two "
            {text_command("ip route", TextCommandColor::Gold)}
            " commands actually look a little different in the IP routing table compared to each other."
            br {}
            "Both are static routes."
            br {}
            strong {
                "However, the route that used the outgoing interface configuration is also noted as a connected route"
            }
            "; this is just a quirk of the output of the "
            {text_command("show ip route", TextCommandColor::Gold)}
            " command."
        }

        {h3_heading("The show ip route static command")}
        p { class: "mb-4",
            "Example 16-4 lists these two routes using the "
            {text_command("show ip route static", TextCommandColor::Gold)}
            " command."
            br {}
            "This command lists the details of static routes only, but it also lists a few statistics about all IPv4 routes."
            br {}
            "For example, the example shows two lines, for the two static routes configured in Example 16-4,
            but statistics state that this router has routes for eight subnets."
        }

        p { class: "mb-4", "See Example 16-4 in volume 1 on page 380." }

        p { class: "mb-4",
            strong {
                "IOS adds and removes these static routes dynamically over time, based on whether the outgoing interface is working or not."
            }
            br {}
            "For example, in this case, if R1's S0/0/0 interface fails, R1 removes the static route to 172.16.2.0/24 from the IPv4 
            routing table."
            br {}
            "Later, when the interface comes up again, IOS adds the route back to the routing table."
        }

        {h3_heading("Reason to use the next hop router in forwarding intructions")}
        p { class: "mb-4",
            "Finally, note that static routes that will send packets out an Ethernet interface—LAN or WAN—should use the next-hop 
            IP address option on the "
            {text_command("ip route", TextCommandColor::Gold)}
            " command, as shown in Example 16-4."
            br {}
            "Routers expect their Ethernet interfaces to be able to reach any number of
            other IP addresses in the connected subnet."
            br {}
            strong {
                "Referencing the next-hop router identifies the specific device in the connected subnet, while referencing the local 
                router's outgoing interface does not identify the specific neighboring router."
            }
        }

        {h3_heading("RECAP")}
        ol { class: "list-disc list-inside",
            li {
                "The static route is considered a network route when the destination listed in the ip route command defines a 
                subnet, or an entire Class A, B, or C network."
            }
            li {
                "A default route matches all destination IP addresses, while a host route matches a single IP address 
                (that is, an address of one host.)"
            }
            li {
                "When the outgoing interface is used in the forwarding instructions, it is also noted as a connected route
                in the output of the "
                {text_command("show ip route static", TextCommandColor::Gold)}
                " command."
            }
            li {
                "IOS adds and removes these static routes dynamically over time, based on whether the outgoing interface is working 
                or not."
            }
            li {
                "Note that most sites use a dynamic routing protocol to learn all the routes to remote subnets
                rather than using static routes"
            }
            li {
                "Referencing the next-hop router in the "
                {text_command("ip route", TextCommandColor::Gold)}
                " global command identifies the specific device in the connected subnet, while referencing the local router's 
                outgoing interface does not identify the specific neighboring router."
            }
        }
    }
}