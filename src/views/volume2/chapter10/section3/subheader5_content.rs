use dioxus::prelude::*;

use crate::utils::{h3_heading, TextCommandColor, text_command};

#[component]
pub fn Content() -> Element {
    rsx! {
        p { class: "mb-4",
            "The majority of NAT troubleshooting issues relate to getting the configuration correct."
            br {}
            "Source NAT has several configuration options—static, dynamic, PAT—with several configuration commands for each."
            br {}
            "You should work hard at building skills with the configuration so that you can quickly recognize configuration 
            mistakes."
            br {}
            "The following troubleshooting checklist summarizes the most common source NAT issues, most of which relate to 
            incorrect configuration."
        }
        ol { class: "list-disc list-inside flex flex-col gap-y-1 mb-4",
            li {
                span { class: "font-bold", "Reversed inside and outside:" }
                " Ensure that the configuration includes the "
                {text_command("ip nat inside", TextCommandColor::Gold)}
                " and "
                {text_command("ip nat outside", TextCommandColor::Gold)}
                " interface subcommands and that the commands are not reversed (the "
                {text_command("ip nat inside", TextCommandColor::Gold)}
                " command on outside interfaces, and vice versa). With source NAT, only the
                inside interface triggers IOS to add new translations, so designating the correct inside
                interfaces is particularly important."
            }
            li {
                span { class: "font-bold", "Static NAT:" }
                " Check the "
                {text_command("ip nat inside source static", TextCommandColor::Gold)}
                " command to ensure it lists the inside local address first and the inside global IP address second."
            }
            li {
                span { class: "font-bold", "Dynamic NAT (ACL):" }
                " Ensure that the ACL configured to match packets sent by the inside hosts match that host's packets before any 
                NAT translation has occurred. For example, if an inside local address of 10.1.1.1 should be translated to 
                200.1.1.1, ensure that the ACL matches source address 10.1.1.1, not 200.1.1.1."
            }
            li {
                span { class: "font-bold", "Dynamic NAT (pool):" }
                " For dynamic NAT without PAT, ensure that the pool has enough IP addresses."
                " When not using PAT, each inside host consumes one IP address from the pool."
                " A large or growing value in the second misses counter in the "
                {text_command("show ip nat statistics", TextCommandColor::Gold)}
                " command output can indicate this problem."
                " Also, compare the configured pool to the list of addresses in the NAT translation table 
                ( "
                {text_command("show ip nat translations", TextCommandColor::Gold)}
                " )."
                " Finally, if the pool is small, the problem may be that the configuration intended to use PAT and is
                missing the "
                {text_command("overload", TextCommandColor::Gold)}
                " keyword (see the next item)."
            }
            li {
                span { class: "font-bold", "PAT:" }
                " It is easy to forget to add the "
                {text_command("overload", TextCommandColor::Gold)}
                " option on the end of the "
                {text_command("ip nat inside source list", TextCommandColor::Gold)}
                " command. PAT configuration is identical to a valid dynamic NAT configuration except that PAT requires the "
                {text_command("overload", TextCommandColor::Gold)}
                " keyword. Without it, dynamic NAT works, but the pool of addresses is typically consumed very quickly. The NAT 
                router will not translate nor forward traffic for hosts if there is not an available pool IP address for their
                traffic, so some hosts experience an outage."
            }
            li {
                span { class: "font-bold", "ACL:" }
                " As mentioned in Chapter 3, “Advanced IPv4 Access Control Lists,” you can always
                add a check for ACLs that cause a problem. Perhaps NAT has been configured correctly,
                but an ACL exists on one of the interfaces, discarding the packets."
                strong { " Note that the order of operations inside the router matters in this case." }
                strong { " For packets entering an interface, IOS processes ACLs before NAT." }
                br {}
                "For packets exiting an interface, IOS processes any outbound ACL after translating the addresses with NAT ."
            }
            li {
                span { class: "font-bold", "User traffic required:" }
                " NAT reacts to user traffic. If you configure NAT in a lab, NAT
                does not act to create translations ( "
                {text_command("show ip nat translations", TextCommandColor::Gold)}
                " ) until some user traffic enters the NAT router on an inside interface, triggering NAT to do a translation. 
                The NAT configuration can be perfect, but if no inbound traffic occurs that matches the
                NAT configuration, NAT does nothing."
            }
            li {
                span { class: "font-bold", "IPv4 routing:" }
                " IPv4 routing could prevent packets from arriving on either side of the
                NAT router. Note that the routing must work for the destination IP addresses used in the
                packets."
            }
        }

        p { class: "mb-4",
            "With source NAT, the user sits at some user device like a PC."
            br {}
            "She attempts to connect to some server, using that server's DNS name."
            br {}
            "After DNS resolution, the client (the inside host) sends an IP packet with a destination address of the server."
            br {}
            "For instance, as shown in Figure 10-11, PC1 sends an IP packet with destination IP address 170.1.1.1, some server in
            the Internet."
            br {}
            "PC1 is an inside host, the server is an outside host, and 170.1.1.1 is the outside global address."
            br {}
            "(Note that these addresses match the previous example, which referenced Figure 10-10.)"
        }

        img {
            class: "mb-4 rounded-lg",
            loading: "lazy",
            alt: "Figure 10-11 Destination Address Changes on Outside to Inside (Only) with Source NAT",
            src: asset!("/assets/static/v2p3c10s3sh5f10-11.png", AssetOptions::image().with_avif()),
        }

        p { class: "mb-4",
            strong {
                "Note that with source NAT in what should be a familiar design, the destination IP address
                of the packet does not change during the entire trip."
            }
            br {}
            "So, troubleshooting of IPv4 routing toward the outside network will be based on the same IP address throughout."
        }

        p { class: "mb-4",
            "Now look at steps 3 and 4 in the figure, which reminds you that the return packet will first
            flow to the NAT inside global address (200.1.1.249 in this case) as shown at step 3."
            br {}
            "Then NAT converts the destination address to 10.1.1.1 in this case."
            br {}
            "So, to troubleshoot packets flowing right to left in this case, you have to troubleshoot based on two different 
            destination IP addresses."
        }

        {h3_heading("RECAP")}
        ol { class: "list-disc list-inside",
            li {
                "With source NAT, only the inside interface triggers IOS to add new translations, so designating the correct 
                inside interfaces is particularly important."
            }
            li {
                "Note that the order of operations inside the router matters, ACLs take precedence over NAT."
            }
            li {
                "Note that with source NAT in what should be a familiar design, the destination IP address of the packet does not 
                change during the entire trip."
            }
        }
    }
}