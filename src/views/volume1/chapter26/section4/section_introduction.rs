use dioxus::prelude::*;

use crate::{
    utils::{h3_heading, h4_heading},
    components::my_accordion::{Accordion, AccordionContent, AccordionItem, AccordionTrigger}
};

#[component]
pub fn SectionIntroductionContent() -> Element {
    rsx! {
        p { class: "mb-4",
            "To send data across a wired link, an electrical signal is applied at one end and carried to the other end."
            br {}
            "The wire itself is continuous and conductive, so the signal can propagate rather easily."
            br {}
            "A wireless link has no physical strands of anything to carry the signal along."
        }

        p { class: "mb-4",
            "How, then, can an electrical signal be sent across the air, or free space?"
            br {}
            "Consider a simple analogy of two people  standing far apart."
            br {}
            "One person wants to signal something to the other."
            br {}
            "They are connected by a long and somewhat loose rope; the rope represents free space."
            br {}
            "The sender at one end decides to lift his end of the rope high and hold it there so that the other
            end of the rope will also rise and notify the partner."
            br {}
            "After all, if the rope were a wire, he knows that he could apply a steady voltage at one end of the wire and it would 
            appear at the other end."
            br {}
            "Figure 26-15 shows the end result; the rope falls back down after a tiny distance, and the receiver never notices a 
            change."
        }

        img {
            class: "mb-4 rounded-lg",
            alt: "Figure 26-15 Failed Attempt to Pass a Message Down a Rope",
            loading: "lazy",
            src: asset!("/assets/static/v1p8c26s4f26-15.png", AssetOptions::image().with_avif()),
        }

        p { class: "mb-4",
            "The sender tries a different strategy."
            br {}
            "He cannot push the rope, but when he begins to wave it up and down in a steady, regular motion, a curious thing 
            happens."
            br {}
            "A continuous wave pattern appears along the entire length of the rope, as shown in Figure 26-16."
            br {}
            "In fact, the waves (each representing one up and down cycle of the sender's arm) actually travel from the
            sender to the receiver."
        }

        img {
            class: "mb-4 rounded-lg",
            alt: "Figure 26-16 Sending a Continuous Wave Down a Rope",
            loading: "lazy",
            src: asset!("/assets/static/v1p8c26s4f26-16.png", AssetOptions::image().with_avif()),
        }

        Accordion { class: "mb-4",
            AccordionItem {
                AccordionTrigger { {h3_heading("Electric and Magnetic Fields")} }
                AccordionContent {
                    p { class: "mb-4",
                        "In free space, a similar principle occurs."
                        br {}
                        "The sender (a transmitter) can send an alternating current into a section of wire (an antenna), which sets up moving 
                        electric and magnetic fields that propagate out and away as traveling waves."
                        br {}
                        strong {
                            "The electric and magnetic fields travel along together and are always at right angles to each other"
                        }
                        ", as shown in Figure 26-17."
                        br {}
                        strong {
                            "The signal must keep changing, or alternating, by cycling up and down, to keep the electric and magnetic fields 
                            cycling and pushing ever outward."
                        }
                    }

                    img {
                        class: "mb-4 rounded-lg",
                        alt: "Figure 26-17 Traveling Electric and Magnetic Waves",
                        loading: "lazy",
                        src: asset!("/assets/static/v1p8c26s4f26-17.png", AssetOptions::image().with_avif()),
                    }
                }
            }
            AccordionItem {
                AccordionTrigger { {h3_heading("Electromagnetic waves direction")} }
                AccordionContent {
                    p { class: "mb-4",
                        strong { "Electromagnetic waves do not travel in a straight line." }
                        br {}
                        "Instead, they travel by expanding in all directions away from the antenna."
                        br {}
                        "To get a visual image, think of dropping a pebble into a pond when the surface is still."
                        br {}
                        "Where it drops in, the pebble sets the water's surface into a cyclic motion."
                        br {}
                        "The waves that result begin small and expand outward, only to be replaced by new waves."
                        br {}
                        strong {
                            "In free space, the electromagnetic waves expand outward in all three dimensions."
                        }
                    }

                    p { class: "mb-4",
                        "Figure 26-18 shows a simple idealistic antenna that is a single point at the end of a wire."
                        br {}
                        "The waves produced expand outward in a spherical shape."
                        br {}
                        strong {
                            "The waves will eventually reach the receiver, in addition to many other locations in other directions."
                        }
                    }

                    img {
                        class: "mb-4 rounded-lg",
                        alt: "Figure 26-18 Wave Propagation with an Idealistic Antenna",
                        loading: "lazy",
                        src: asset!("/assets/static/v1p8c26s4f26-18.png", AssetOptions::image().with_avif()),
                    }

                    {h4_heading("Processing the received signal")}
                    p { class: "mb-4",
                        "At the receiving end of a wireless link, the process is reversed."
                        br {}
                        strong {
                            "As the electromagnetic waves reach the receiver's antenna, they induce an electrical signal."
                        }
                        br {}
                        "If everything works right, the received signal will be a reasonable copy of the original transmitted signal."
                    }
                }
            }
            AccordionItem {
                AccordionTrigger { {h3_heading("Interpreting electromagnetic waves")} }
                AccordionContent {
                    p { class: "mb-4",
                        "The electromagnetic waves involved in a wireless link can be measured and described in several ways."
                        br {}
                        strong {
                            "One fundamental property is the frequency of the wave, or the number of times the signal makes one complete up and 
                            down cycle in 1 second."
                        }
                        br {}
                        "Figure 26-19 shows how a cycle of a wave can be identified."
                        br {}
                        strong {
                            "A cycle can begin as the signal rises from the center line, falls through the center line, and rises again to meet 
                            the center line."
                        }
                        br {}
                        "A cycle can also be measured from the center of one peak to the center of the next peak."
                        br {}
                        strong {
                            "No matter where you start measuring a cycle, the signal must make a complete sequence back to its starting position
                            where it is ready to repeat the same cyclic pattern."
                        }
                    }

                    img {
                        class: "mb-4 rounded-lg",
                        alt: "Figure 26-19 Cycles Within a Wave",
                        loading: "lazy",
                        src: asset!("/assets/static/v1p8c26s4f26-19.png", AssetOptions::image().with_avif()),
                    }

                    {h4_heading("Hertz")}
                    p { class: "mb-4",
                        "In Figure 26-19, suppose that 1 second has elapsed, as shown."
                        br {}
                        "During that 1 second, the signal progressed through four complete cycles."
                        br {}
                        "Therefore, its frequency is 4 cycles/second or 4 hertz."
                        br {}
                        strong {
                            "A hertz (Hz) is the most commonly used frequency unit and is nothing other than one cycle per second."
                        }
                    }

                    p { class: "mb-4",
                        "Frequency can vary over a very wide range."
                        br {}
                        "As frequency increases by orders of magnitude, the numbers can become quite large."
                        br {}
                        "To keep things simple, the frequency unit name can be modified to denote an increasing number of zeros, as listed in 
                        Table 26-2."
                    }

                    img {
                        class: "mb-4 rounded-lg",
                        alt: "Table 26-2 Frequency Unit Names",
                        loading: "lazy",
                        src: asset!("/assets/static/v1p8c26s4t26-2.png", AssetOptions::image().with_avif()),
                    }

                    {h4_heading("Electromagnetic Spectrum")}
                    p { class: "mb-4",
                        "Figure 26-20 shows a simple representation of the continuous frequency spectrum ranging from 0 Hz to 1022 
                        (or 1 followed by 22 zeros) Hz."
                        br {}
                        "At the low end of the spectrum are frequencies that are too low to be heard by the human ear, followed by audible 
                        sounds."
                        br {}
                        "The highest range of frequencies contains light, followed by X, gamma, and cosmic rays."
                    }

                    img {
                        class: "mb-4 rounded-lg",
                        alt: "Figure 26-20 Continuous Frequency Spectrum",
                        loading: "lazy",
                        src: asset!("/assets/static/v1p8c26s4f26-20.png", AssetOptions::image().with_avif()),
                    }

                    p { class: "mb-4",
                        "The frequency range from around 3 kHz to 300 GHz is commonly called "
                        i { "radio frequency" }
                        " (RF)."
                        br {}
                        "It includes many different types of radio communication, including low-frequency radio, AM radio, shortwave radio, 
                        television, FM radio, microwave, and radar."
                        br {}
                        "The microwave category also contains the two main frequency ranges that are used for wireless LAN 
                        communication: 2.4 and 5 GHz."
                    }
                }
            }
        }

        {h3_heading("RECAP")}
        ol { class: "list-disc list-inside",
            li {
                "To send a signal in free space, the sender (a transmitter) can send an alternating current into a section of wire 
                (an antenna), which sets up moving electric and magnetic fields that propagate out and away as traveling waves."
            }
            li {
                "The electric and magnetic fields travel along together and are always at right angles to each other."
            }
            li {
                "The signal must keep changing, or alternating, by cycling up and down, to keep the electric and magnetic fields 
                cycling and pushing ever outward."
            }
            li {
                "Electromagnetic waves do not travel in a straight line. Instead, they travel by expanding in all directions away 
                from the antenna."
            }
            li {
                "The frequency of the wave is the number of times the signal makes one complete up and down cycle in 1 second."
            }
            li {
                "A hertz (Hz) is the most commonly used frequency unit and is nothing other than one cycle per second."
            }
            li {
                "The microwave category also contains the two main frequency ranges that are used for wireless LAN 
                communication: 2.4 and 5 GHz."
            }
        }
    }
}