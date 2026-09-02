use dioxus::prelude::*;

use crate::{components::green_div::GreenNote, utils::text_command::text_command};

#[component]
pub fn Content() -> Element {
    rsx! {
        p { class: "mb-4",
            "CDP discovers basic information about neighboring routers and switches without needing
            to know the passwords for the neighboring devices."
            br {}
            "To discover information, routers and switches send CDP messages out each of their interfaces."
            br {}
            "The messages essentially announce information about the device that sent the CDP message."
            br {}
            "Devices that support CDP learn information about others by listening for the advertisements sent by other devices."
        }

        p { class: "mb-1", "CDP discovers several useful details from the neighboring Cisco devices:" }
        ol { class: "list-disc list-inside mb-4",
            li {
                strong { class: "text-sky-600", "Device identifier:" }
                " Typically the host name"
            }
            li {
                strong { class: "text-sky-600", "Address list:" }
                " Network and data-link addresses"
            }
            li {
                strong { class: "text-sky-600", "Port identifier:" }
                " The interface on the remote router or switch on the other end of the link
                that sent the CDP advertisement"
            }
            li {
                strong { class: "text-sky-600", "Capabilities list:" }
                " Information on what type of device it is (for example, a router or a switch)"
            }
            li {
                strong { class: "text-sky-600", "Platform:" }
                " The model and OS level running on the device"
            }
        }

        p { class: "mb-3",
            strong { "CDP plays two general roles:" }
            " to provide information to the devices to support some function and to provide information 
            to the network engineers that manage the devices."
            br {}
            "For example, Cisco IP Phones use CDP to learn the data and voice VLAN IDs as configured
            on the access switch."
            br {}
            "For that second role, CDP has "
            {text_command("show")}
            " commands that list information about neighboring devices, as well as 
            information about how CDP is working."
            br {}
            "Table 9-3 describes the three "
            {text_command("show")}
            " commands that list the most important CDP information."
        }
        img {
            class: "mb-4 rounded-lg",
            alt: "Table 9-3 that lists options for the show dcp command",
            src: asset!("/assets/static/v2p3c9s3sh1t9-3.png", AssetOptions::image().with_avif()),
        }

        GreenNote {
            p {
                strong { "NOTE" }
                " Cisco routers and switches support the same CDP commands, with the same parameters and same types of output."
            }
        }

        p { class: "mb-3",
            "The next example shows the power of the information in CDP commands."
            br {}
            "The example uses the network shown in Figure 9-8, with Example 9-15 listing the output of several "
            {text_command("show cdp")}
            " commands."
        }
        img {
            class: "mb-4 rounded-lg",
            alt: "A picture of a Figure 9-8 and a screentshot of the output of show cdp neighbors",
            src: asset!("/assets/static/v2p3c9s3sh1f9-3.png", AssetOptions::image().with_avif()),
        }

        p {
            "The "
            {text_command("show cdp neighbors")}
            " command lists one line per neighbor."
            br {}
            "(Look for the Device ID column and the list that includes SW1 and R1.)"
            br {}
            "Each of those two lines lists the most important
            topology information about each neighbor:"
        }
        ol { class: "list-disc list-inside mb-4",
            li { "the neighbor's host name (Device ID)," }
            li { "the local device's interface," }
            li { "and the neighboring device's interface (under the Port heading)." }
        }

        p { class: "mb-4",
            "Pay close attention to the local device's interface and the neighboring device's interface,
            comparing the example to the figure."
            br {}
            "For example, SW2's "
            {text_command("show cdp neighbors")}
            " command lists an entry for SW1, with SW2's local interface 
            of Gi0/2 and SW1's interface of Gi0/1 under the heading “Port ID .”"
        }

        p { class: "mb-4",
            "This command also lists the platform, identifying the specific model of the neighboring
            router or switch."
            br {}
            strong {
                "So, even using this basic information, you could either construct a figure
                like Figure 9-8 or confirm that the details in the figure are correct."
            }
        }

        h3 { class: "font-semibold text-lg mb-1", "The CDP encapsulation" }
        p { class: "mb-4",
            "Figure 9-8 and Example 9-15 provide a good backdrop as to why devices learn about direct
            neighbors with CDP, but not other neighbors."
            br {}
            " First, CDP defines encapsulation that uses the data-link header, but no IP header."
            br {}
            "To ensure all devices receive a CDP message, the Ethernet header uses a multicast destination MAC 
            address (0100.0CCC.CCCC)."
            br {}
            " However, when any device that supports CDP receives a CP message, the device processes the message and 
            then discards it, rather than forwarding it."
            br {}
            "So, for instance, when router R1 sends a CDP message to Ethernet multicast address 0100.0CCC.CCCC, 
            switch SW2 receives it, processes it, but does not forward it to switch SW1—so SW1 will not list router R1 as a
            CDP neighbor."
        }

        h3 { class: "font-semibold text-lg mb-1", "The show cdp neighbors detail" }
        p { class: "mb-4",
            "Next, consider the "
            {text_command("show cdp neighbors detail")}
            " command as shown in Example 9-16, again taken from switch SW2."
            br {}
            "This command lists more detail, as you might have guessed."
            br {}
            "The detail lists the full name of the switch model (WS-2960XR-24TS-I) and the IP address configured on the 
            neighboring device."
            br {}
            "You have to look closely, but the example has one long
            group of messages for each of the two neighbors; the example includes one comment line
            with gray highlight to help you find the dividing point between groups of messages "
        }

        p { class: "mb-4", "See Example 9-16 on page 192 in the book." }

        GreenNote {
            p {
                strong { "NOTE" }
                " The "
                {text_command("show cdp entry")}
                i { " name" }
                " command lists the exact same details shown in the output of the "
                {text_command("show cdp neighbors detail")}
                " command, but for only the one neighbor listed in the command."
            }
        }

        p { class: "mb-4",
            "As you can see, you can sit on one device and discover a lot of information about a neighboring device—a fact that 
            actually creates a security exposure."
            br {}
            strong {
                "Cisco recommends that CDP be disabled on any interface that might not have a need for CDP."
            }
            br {}
            "For switches, any switch port connected to another switch, a router, or to an IP phone should use CDP."
        }

        p { class: "mb-4",
            "Finally, note that CDP shows information about directly connected neighbors."
            br {}
            "For instance, "
            {text_command("show cdp neighbors")}
            " on SW1 would list an entry for SW2 in this case, but not R1, because
            R1 is not directly connected to SW1."
        }

        h3 { class: "font-semibold text-lg mb-1", "REMEMBER" }
        ul { class: "list-disc pl-4",
            li { "Network devices use CDP to advertise informations about themselves." }
            li { "CDP uses a multicast destination MAC address (0100.0CCC.CCCC)." }
            li {
                "CDP plays two general roles: to provide information to the devices to support some function and to provide information 
                to the network engineers that manage the devices."
            }
            li {
                "Cisco recommends that CDP be disabled on any interface that might not have a need for CDP."
            }
            li {
                "For switches, any switch port connected to another switch, a router, or to an IP phone should use CDP."
            }
            li { "CDP only shows information about directly connected neighbors." }
            li { "CDP is a Cisco-proprietary layer 2 protocol." }
        }
    }
}