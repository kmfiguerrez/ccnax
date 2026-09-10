use dioxus::prelude::*;

use crate::{
    components::KeyTopic, 
    utils::{h3_heading, text_command, TextCommandColor}
};

#[component]
pub fn Content() -> Element {
    rsx! {
        p { class: "mb-4",
            "When the interface reaches the connect (up/up) state, the switch considers the interface to
            be working."
            br {}
            "The switch, of course, tries to use the interface, and at the same time, the switch
            keeps various interface counters."
            br {}
            "These interface counters can help identify problems that can occur even though the interface is in a connect state, 
            like issues related to the duplex mismatch problem that was just described."
            br {}
            "This section explains some of the related concepts and a few of the most common problems."
        }

        {h3_heading("Ethernet trailer FCS field")}
        p { class: "mb-4",
            "Whenever the physical transmission has problems, the receiving device might receive a frame
            whose bits have changed values."
            br {}
            "These frames do not pass the error detection logic as implemented in the FCS field in the Ethernet trailer, 
            as covered in Chapter 2."
            br {}
            "The receiving device discards the frame and counts it as some kind of input error."
            br {}
            "Cisco switches list this error as a CRC error, as highlighted in Example 7-9."
            br {}
            "(Cyclic redundancy check [CRC] is a term related to how the frame check sequence [FCS] math detects an error.)"
        }

        img {
            class: "mb-4 rounded-lg",
            alt: "Example 7-9 Interface Counters for Layer 1 Problems",
            loading: "lazy",
            src: asset!("/assets/static/v1p2c7s2sh3ex7-9.png", AssetOptions::image().with_avif()),
        }

        {h3_heading("List of Interface counters")}
        p { class: "mb-4",
            "The number of input errors and the number of CRC errors are just a few of the counters in
            the output of the "
            {text_command("show interfaces", TextCommandColor::Gold)}
            " command."
            br {}
            "The challenge is to decide which counters you need to think about, which ones show that a problem is happening, 
            and which ones are normal and of no concern."
        }

        p {
            "The example highlights several of the counters as examples so that you can start to understand which ones point to 
            problems and which ones are just counting normal events that are not problems."
            br {}
            "The following list shows a short description of each highlighted counter, in the order shown in the example:"
        }
        KeyTopic {}
        ul { class: "list-disc list-inside mb-4",
            li {
                span { class: "font-bold", "Runts:" }
                " Frames that did not meet the minimum frame size requirement (64 bytes, including
                the 18-byte destination MAC, source MAC, type, and FCS). Can be caused by collisions."
            }
            li {
                span { class: "font-bold", "Giants:" }
                " Frames that exceed the maximum frame size requirement (1518 bytes, including
                the 18-byte destination MAC, source MAC, type, and FCS)."
            }
            li {
                span { class: "font-bold", "Input Errors:" }
                " A total of many counters, including runts, giants, no buffer, CRC, frame,
                overrun, and ignored counts."
            }
            li {
                span { class: "font-bold", "CRC:" }
                " Received frames that did not pass the FCS math; can be caused by collisions."
            }
            li {
                span { class: "font-bold", "Frame:" }
                " Received frames that have an illegal format, for example, ending with a partial
                byte; can be caused by collisions."
            }
            li {
                span { class: "font-bold", "Packets Output:" }
                " Total number of packets (frames) forwarded out the interface."
            }
            li {
                span { class: "font-bold", "Collisions:" }
                " Counter of all collisions that occur when the interface is transmitting a frame."
            }
            li {
                span { class: "font-bold", "Late Collisions:" }
                " The subset of all collisions that happen after the 64th byte of the frame
                has been transmitted. (In a properly working Ethernet LAN--Half-duplex Ethernet, collisions should occur within
                the first 64 bytes; late collisions today often point to a duplex mismatch.)"
            }
        }

        {h3_heading("Counters related to CSMA/CD")}
        p { class: "mb-4",
            "Note that many of these counters occur as part of the CSMA/CD process used when half duplex is enabled."
            br {}
            "Collisions occur as a normal part of the half-duplex logic imposed by CSMA/CD, so a switch interface with an 
            increasing collisions counter might not even have a problem."
            br {}
            "However, one problem, called late collisions, points to the classic duplex mismatch problem."
        }

        {h3_heading("Late collisions")}
        p { class: "mb-4",
            "If a LAN design follows cabling guidelines, all collisions should occur by the end of the
            64th byte of any frame (Other way to say this is If the Ethernet network is designed correctly, 
            a collision should be detected before or by the end of byte 64.)."
            br {}
            "When a switch has already sent 64 bytes of a frame, and the switch receives a frame on that same interface, 
            the switch senses a collision."
            br {}
            "In this case, the collision is a late collision, and the switch increments the late collision counter in addition 
            to the usual CSMA/CD actions to send a jam signal (adds to the collision counter), wait a random time, and try again."
        }

        {h3_heading("Why late collisions matter today")}
        p {
            "In modern networks almost everything is switched and runs full-duplex, so collisions should be rare or nonexistent."
            br {}
            "When late collisions do appear, the most common cause is a duplex mismatch:"
        }
        ul { class: "list-disc list-inside",
            li { "One side is set to full-duplex (or auto-negotiated to full-duplex)." }
            li { "The other side is half-duplex." }
            li {
                "The full-duplex side never listens before transmitting and never expects collisions, so when a collision finally 
                occurs it is often detected late."
            }
        }
        p { class: "mb-4",
            "Other less common causes include excessively long cables that violate the maximum 
            network diameter (maximum cable lengths + number of repeaters/hubs), or faulty hardware."
        }

        p { class: "mb-4",
            "With a duplex mismatch, like the mismatch between SW1 and SW2 in Figure 7-4, the halfduplex interface will likely see the 
            late collisions counter increment."
            br {}
            "Why? The half-duplex interface sends a frame (SW1), but the full-duplex neighbor (SW2) sends at any time, even
            after the 64th byte of the frame sent by the half-duplex switch."
            br {}
            "So, just keep repeating the "
            {text_command("show interfaces", TextCommandColor::Gold)}
            " command, and if you see the late collisions counter incrementing on a 
            half duplex interface, you might have a duplex mismatch problem."
        }

        {h3_heading("Physical Cabling Issues")}
        p { class: "mb-4",
            "A working interface (in an up/up state) can still suffer from issues related to the physical
            cabling as well."
            br {}
            "The cabling problems might not be bad enough to cause a complete failure, but the transmission failures result 
            in some frames failing to pass successfully over the cable."
            br {}
            "For example, excessive interference on the cable can cause the various input error counters
            to keep growing larger, especially the CRC counter."
            br {}
            "In particular, if the CRC errors grow, but the collisions counters do not, the problem might simply be interference 
            on the cable."
            br {}
            "(The switch counts each collided frame as one form of input error as well.)"
        }

        {h3_heading("RECAP")}
        ol { class: "list-disc list-inside",
            li {
                "Interface counters can help identify problems that can occur even though the interface is in a connect state, 
                like issues related to the duplex mismatch problem (intermittent problems)."
            }
            li {
                "Cyclic redundancy check [CRC] is a term related to how the frame check sequence [FCS] math detects an error."
            }
            li { "CRC error is a kind of input error that happens when a frame arrives with an error." }
            li {
                "In a properly working Ethernet LAN (we are talking about half-duplex Ethernet-CSMA/CD), collisions should occur 
                within the first 64 bytes; late collisions today often point to a duplex mismatch."
            }
            li {
                "Half-duplex Ethernet (CSMA/CD) expects collision to occur within the first 64 bytes of a frame."
            }
            li {
                "Collisions occur as a normal part of the half-duplex logic imposed by CSMA/CD, so a switch interface with an 
                increasing collisions counter might not even have a problem."
            }
            li {
                "A late collision is any collision sensed after the switch (or NIC) has already transmitted more than 64 bytes 
                of a frame."
            }
            li { "The switch counts each collided frame as one form of input error as well." }
            li {
                "In modern switched Ethernet, collisions should essentially disappear when links operate correctly in full duplex."
                " Both sides can transmit and receive simultaneously without CSMA/CD collisions."
            }
            li {
                "A working interface (in an up/up state) can still suffer from issues related to the physical cabling as well."
            }
            li {
                "If the CRC errors grow, but the collisions counters do not, the problem might simply be interference on the cable."
            }
        }
    }
}