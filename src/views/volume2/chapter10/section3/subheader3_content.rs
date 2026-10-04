use dioxus::prelude::*;

use crate::utils::{TextCommandColor, h3_heading, text_command};

#[component]
pub fn Content() -> Element {
    rsx! {
        p { class: "mb-1",
            "Examples 10-3 and 10-4 show the evidence that dynamic NAT begins with no NAT
            table entries, but the router reacts after user traffic correctly drives the NAT function."
            br {}
            "Example 10-3 shows the output of the "
            {text_command("show ip nat translations", TextCommandColor::Gold)}
            " and "
            {text_command("show ip nat statistics", TextCommandColor::Gold)}
            " commands before any users generate traffic that makes NAT do some work."
            br {}
            "The "
            {text_command("show ip nat translations", TextCommandColor::Gold)}
            " command, which lists the NAT table entries, lists a blank line; the "
            {text_command("show ip nat statistics", TextCommandColor::Gold)}
            " command, which shows how many times NAT has created a NAT table entry, shows 0 active translations."
        }

        p { class: "mb-4", "See Example 10-3 in volume 1 on page 217." }

        p { class: "mb-4",
            "The "
            {text_command("show ip nat statistics", TextCommandColor::Gold)}
            " command at the end of the example lists some particularly interesting troubleshooting 
            information with two different counters labeled “misses,” as highlighted in the example."
            br {}
            "The first occurrence of this counter counts the number of times a new packet comes along, needing a NAT entry, and 
            not finding one."
            br {}
            "At that point, dynamic NAT reacts and builds an entry."
            br {}
            "The second misses counter toward the end of the command output lists the number of misses in the pool."
            br {}
            "This counter increments only when dynamic NAT tries to allocate a new NAT table entry and finds no available 
            addresses, so the packet cannot be translated—probably resulting in an end user not getting to the
            application."
        }

        p { class: "mb-4",
            "Next, Example 10-4 updates the output of both commands after the user of the host at
            10.1.1.1 telnets to host 170.1.1.1."
        }

        p { class: "mb-4", "See Example 10-4 in volume 1 on page 218." }

        p { class: "mb-4",
            "The example begins with host 10.1.1.1 telnetting to 170.1.1.1 (not shown), with the NAT
            router creating a NAT entry."
            br {}
            "The NAT table shows a single entry, mapping 10.1.1.1 to 200.1.1.1."
            br {}
            "And, the first line in the output of "
            {text_command("show ip nat statistics", TextCommandColor::Gold)}
            " command lists a counter for 1 active translation, as shown in the NAT table at the top of the example."
        }

        p { class: "mb-4",
            "Take an extra moment to consider the highlighted line, where the "
            {text_command("show ip nat statistics", TextCommandColor::Gold)}
            " command lists 1 miss and 69 hits."
            br {}
            "The first miss counter, now at 1, means that one packet arrived that needed NAT, but there was no NAT table entry."
            br {}
            "NAT reacted and added a NAT table entry, so the hit counter of 69 means that the next 69 packets used the newly 
            added NAT table entry."
            br {}
            "The second misses counter, still at 0, did not increment because the NAT pool had enough available inside global IP 
            addresses to use to allocate the new NAT table entry."
            br {}
            "Also note that the last line lists statistics on the number of pool members allocated (1)
            and the percentage of the pool currently in use (50%)."
        }

        {h3_heading("Dynamic NAT table entries time out")}
        p { class: "mb-4",
            "The dynamic NAT table entries time out after a period of inactivity, putting those inside
            global addresses back in the pool for future use."
            br {}
            "Example 10-5 shows a sequence in which two different hosts make use of inside global address 200.1.1.1."
            br {}
            "Host 10.1.1.1 uses inside global address 200.1.1.1 at the beginning of the example."
            br {}
            "Then, instead of just waiting on the NAT entry to time out, the example clears the NAT table entry with the "
            {text_command("clear ip nat translation * ", TextCommandColor::Gold)}
            " command."
            br {}
            "At that point, the user at 10.1.1.2 telnets to 170.1.1.1, and the new NAT table entry appears, using the same 
            200.1.1.1 inside global address."
        }

        p { class: "mb-4", "See Example 10-5 in volume 1 on page 219." }

        {h3_heading("Debugging with NAT")}
        p { class: "mb-4",
            "Finally, at the end of Example 10-5, you see that host 10.1.1.1 has telnetted to another
            host in the Internet, plus the output from the "
            {text_command("debug ip nat", TextCommandColor::Gold)}
            " command."
            br {}
            "This "
            {text_command("debug", TextCommandColor::Gold)}
            " command causes the router to issue a message every time a packet has its address translated for NAT."
            br {}
            "You generate the output results by entering a few lines from the Telnet connection from 10.1.1.1 to 170.1.1.1."
            br {}
            "The debug output tells you that host 10.1.1.1 now uses inside global address 200.1.1.2 for this new connection."
        }

    }
}