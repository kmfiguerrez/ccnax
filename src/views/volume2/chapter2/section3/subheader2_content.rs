use dioxus::prelude::*;

use crate::{components::GreenNote, utils::{TextCommandColor, h3_heading, text_command}};

#[component]
pub fn Content() -> Element {
    rsx! {
        p { class: "mb-4",
            "In some cases, you may not be creating your own ACL."
            br {}
            "Instead, you may need to interpret some existing "
            {text_command("access-list", TextCommandColor::Gold)}
            " commands."
            br {}
            "To answer these types of questions on the exams, you need to determine the range of IP addresses matched by a 
            particular address/wildcard mask combination in each ACL statement."
        }

        {h3_heading("Finding the high end range")}
        p { class: "mb-4",
            "Under certain assumptions that are reasonable for CCNA certifications, calculating the
            range of addresses matched by an ACL can be relatively simple."
            br {}
            "Basically, the range of addresses begins with the address configured in the ACL command."
            br {}
            "The range of addresses ends with the sum of the address field and the wildcard mask."
            br {}
            "That's it."
        }

        p { class: "mb-4",
            "For example, with the command access-list 1 permit 172.16.200.0 0.0.7.255, the low end
            of the range is simply 172.16.200.0, taken directly from the command itself."
            br {}
            "Then, to find the high end of the range, just add this number to the WC mask, as follows:"
        }

        img {
            class: "mb-4 rounded-lg",
            loading: "lazy",
            alt: "The high end of the range computation sample",
            src: asset!(
                "/assets/static/v2p1c2s3sh2he-formula.png", AssetOptions::image().with_avif()
            ),
        }

        p { class: "mb-4",
            "For this last bit of practice, look at the existing access-list commands in Table 2-3."
            br {}
            "In each case, make a notation about the exact IP address, or range of IP addresses, matched by the
            command."
        }

        p { class: "mb-4", "See Table 2-3 in volume 2 on page 40." }

        {h3_heading("IOS intercepting the access-list command")}
        p { class: "mb-4",
            "Interestingly , IOS lets the CLI user type an "
            {text_command("access-list", TextCommandColor::Gold)}
            " command in configuration mode, and IOS will potentially change the address parameter before placing the 
            command into the running-config file."
            br {}
            "This process of just finding the range of addresses matched by the "
            {text_command("access-list", TextCommandColor::Gold)}
            " command expects that the "
            {text_command("access-list", TextCommandColor::Gold)}
            " command came from the router, so that any such changes were complete."
        }

        p { class: "mb-4",
            "The change IOS can make with an "
            {text_command("access-list", TextCommandColor::Gold)}
            " command is to convert to 0 any octet of an address for which the wildcard mask's octet is 255."
            br {}
            "For example, with a wildcard mask of 0.0.255.255, IOS ignores the last two octets."
            br {}
            "IOS expects the address field to end with two 0s."
            br {}
            "If not, IOS still accepts the "
            {text_command("access-list", TextCommandColor::Gold)}
            " command, but IOS changes the last two octets of the address to 0s."
            br {}
            "Example 2-5 shows an example, where the configuration shows address 10.1.1.1, but wildcard mask 0.0.255.255."
        }

        p { class: "mb-4", "See Example 2-5 in volume 2 on page 41." }

        p { class: "mb-4",
            "The math to find the range of addresses relies on the fact that either the command is fully
            correct or that IOS has already set these address octets to 0, as shown in the example."
        }

        GreenNote {
            p {
                strong { "NOTE" }
                " The most useful WC masks, in binary, do not interleave 0s and 1s. This book
                assumes the use of only these types of WC masks. However, Cisco IOS allows WC masks
                that interleave 0s and 1s, but using these WC masks breaks the simple method of calculating
                the range of addresses. As you progress through to CCIE studies, be ready to dig deeper to
                learn how to determine what an ACL matches."
            }
        }

        {h3_heading("RECAP")}
        ol { class: "list-disc list-inside",
            li {
                "The change IOS can make with an "
                {text_command("access-list", TextCommandColor::Gold)}
                " command is to convert to 0 any octet of an address for which the wildcard mask's octet is 255, 
                before placing the command into the running-config file."
            }
        }

    }
}