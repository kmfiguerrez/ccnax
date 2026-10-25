use dioxus::prelude::*;

use crate::utils::h3_heading;

#[component]
pub fn SectionIntroductionContent() -> Element {
    rsx! {
        p { class: "mb-4",
            "In a wired network, any two devices that need to communicate with each other must be connected by a wire."
            br {}
            "(That was obvious!) The “wire” might contain strands of metal or fiberoptic material that run continuously from one 
            end to the other."
            br {}
            strong { "Data that passes over the wire is bounded by the physical properties of the wire." }
            br {}
            "In fact, the IEEE 802.3 set of standards defines strict guidelines for the Ethernet wire itself, in addition to how 
            devices may connect, send, and receive data over the wire."
        }

        p { class: "mb-4",
            "Wired connections have been engineered with tight constraints and have few variables that might prevent successful 
            communication."
            br {}
            "Even the type and size of the wire strands, the number of twists the strands must make around each other over a 
            distance, and the maximum length of the wire must adhere to the standard."
        }

        {h3_heading("Data path in a wired connections")}
        p { class: "mb-4",
            strong {
                "Therefore, a wired network is essentially a bounded medium; data must travel over whatever path the wire or cable 
            takes between two devices."
            }
            br {}
            "If the cable goes around a corner or lies in a coil, the electrical signals used to carry the data must also go around 
            a corner or around a coil."
            br {}
            "Because only two devices may connect to a wire, only those two devices may send or transmit data."
            br {}
            "Even better: the two devices may transmit data to each other simultaneously because they each have a private, direct 
            path to each other."
        }

        {h3_heading("Wired connection shortcomings")}
        p { class: "mb-4",
            "Wired networks also have some shortcomings."
            br {}
            "When a device is connected by a wire, it cannot move around very easily or very far."
            br {}
            "Before a device can connect to a wired network, it must have a connector that is compatible with the one on the end 
            of the wire."
            br {}
            "As devices get smaller and more mobile, it just is not practical to connect them to a wire."
        }

        {h3_heading("Wireless connections")}
        p { class: "mb-4",
            strong {
                "As its name implies, a wireless network removes the need to be tethered to a wire or cable."
            }
            br {}
            "Convenience and mobility become paramount, enabling users to move around at will while staying connected to the network."
            br {}
            "A user can (and often does) bring along many different wireless devices that can all connect to the network easily 
            and seamlessly."
        }

        p { class: "mb-1",
            strong {
                "Wireless data must travel through free space, without the constraints and protection of a wire."
            }
            br {}
            "In the free space environment, many variables can affect the data and its delivery."
            br {}
            "To minimize the variables, wireless engineering efforts must focus on two things:"
        }
        ol { class: "list-disc list-inside mb-4",
            li { "Wireless devices must adhere to a common standard (IEEE 802.11)." }
            li { "Wireless coverage must exist in the area where devices are expected to use it." }
        }

        p { class: "mb-4",
            "As you study for the CCNA 200-301 exam, keep in mind that the exam is geared more toward a functional view of wireless 
            technology."
            br {}
            "More detailed topics like RF characteristics, antenna performance, and so on are reserved for the Implementing Cisco 
            Enterprise Network Core Technologies ENCOR 300-401 exam."
        }

        {h3_heading("RECAP")}
        ol { class: "list-disc list-inside",
            li {
                "In a wired network, any two devices that need to communicate with each other must be connected by a wire."
            }
            li {
                "The IEEE 802.3 set of standards defines strict guidelines for the Ethernet wire itself, in addition to how devices 
                may connect, send, and receive data over the wire."
            }
            li {
                "A wired network is essentially a bounded medium; data must travel over whatever path the wire or cable takes between two devices."
            }
            li { "A wireless network removes the need to be tethered to a wire or cable." }
            li {
                "Wireless data must travel through free space, without the constraints and protection of a wire."
            }
        }
    }
}