use dioxus::prelude::*;

use crate::{
    utils::h3_heading,
    components::{KeyTopic, GreenNote}
};

#[component]
pub fn Content() -> Element {
    rsx! {
        p { class: "mb-4",
            "Because a range of frequencies might be used for the same purpose, it is customary to refer
            to the range as a "
            i { "band" }
            " of frequencies."
            br {}
            "For example, the range from 530 kHz to around 1710 kHz is used by AM radio stations; therefore, it is commonly 
            called the AM band or the AM broadcast band."
        }

        {h3_heading("2.4GHz Band")}
        p { class: "mb-4",
            "One of the two main frequency ranges used for wireless LAN communication lies between 2.400 and 2.4835 GHz."
            br {}
            "This is usually called the "
            i { "2.4-GHz band" }
            ", even though it does not encompass the entire range between 2.4 and 2.5 GHz."
            br {}
            "It is much more convenient to refer to the band name instead of the specific range of frequencies included."
        }

        {h3_heading("5GHz Band")}
        p { class: "mb-1",
            "The other wireless LAN range is usually called the "
            i { "5-GHz band" }
            " because it lies between 5.150 and 5.825 GHz."
            br {}
            "The 5-GHz band actually contains the following four separate and distinct bands:"
        }
        ol { class: "mb-4",
            li { "5.150 to 5.250 GHz" }
            li { "5.250 to 5.350 GHz" }
            li { "5.470 to 5.725 GHz" }
            li { "5.725 to 5.825 GHz" }
        }

        GreenNote {
            p {
                strong { "TIP" }
                " You might have noticed that most of the 5-GHz bands are contiguous except for a gap
                between 5.350 and 5.470. At the time of this writing, this gap exists and cannot be used for
                wireless LANs. However, some governmental agencies have moved to reclaim the frequencies and repurpose them for 
                wireless LANs. Efforts are also underway to add 5.825 through 5.925 GHz."
            }
        }

        p { class: "mb-4",
            "It is interesting that the 5-GHz band can contain several smaller bands."
            br {}
            "Remember that the term "
            i { "band" }
            " is simply a relative term that is used for convenience."
            br {}
            strong {
                "Do not worry about memorizing the band names or exact frequency ranges; just be aware of the two main bands at 2.4
                and 5 GHz."
            }
        }

        {h3_heading("Channels")}
        p { class: "mb-4",
            strong { "A frequency band contains a continuous range of frequencies." }
            br {}
            "If two devices require a single frequency for a wireless link between them, which frequency can they use?"
            br {}
            "Beyond that, how many unique frequencies can be used within a band?"
            br {}
            strong {
                "To keep everything orderly and compatible, bands are usually divided into a number of distinct channels."
            }
            br {}
            strong { "Each channel is known by a channel number and is assigned to a specific frequency." }
            br {}
            "As long as the channels are defined by a national or international standards body, they can be used consistently in
            all locations."
            br {}
            "Figures 26-21 and 26-22 show the channel layout for the 2.4 and 5 GHz bands, respectively."
        }

        img {
            class: "mb-4 rounded-lg",
            alt: "Figure 26-21 Channel Layout in the 2.4-GHz Band",
            loading: "lazy",
            src: asset!("/assets/static/v1p8c26s4sh1f26-21.png", AssetOptions::image().with_avif()),
        }

        img {
            class: "mb-4 rounded-lg",
            alt: "Figure 26-22 Channel Layout in the 5-GHz Band",
            loading: "lazy",
            src: asset!("/assets/static/v1p8c26s4sh1f26-22.png", AssetOptions::image().with_avif()),
        }

        p { class: "mb-4",
            "You might assume that an AP can use any channel number without affecting any APs that use other channel numbers."
            br {}
            "In the 5-GHz band, this is the case because each channel is allocated a frequency range that does not encroach on or 
            overlap the frequencies allocated for any other channel."
            br {}
            "In other words, the 5-GHz band consists of "
            i { "nonoverlapping channels" }
            "."
        }

        KeyTopic {}
        p { class: "mb-4",
            "The same is not true of the 2.4-GHz band."
            br {}
            "Each of its channels is much too wide to avoid overlapping the next lower or upper channel number."
            br {}
            strong {
                "In fact, each channel covers the frequency range that is allocated to more than four consecutive channels!"
            }
            br {}
            "Notice the width of the channel spacing in Figure 26-21 as compared to the width of one of the shaded signals
            centered on channels 1, 6, and 11."
            br {}
            strong {
                "The only way to avoid any overlap between adjacent channels is to configure APs to use only channels 1, 6, and 11."
            }
            br {}
            "Even though there are 14 channels available to use, you should always strive for nonoverlapping channels in your 
            network."
        }

        {h3_heading("RECAP")}
        ol { class: "list-disc list-inside",
            li { "A band refers to a range of frequencies: AM band, 2.4GHz band and 5GHz band." }
            li {
                "A frequency band contains a continuous range of frequencies. To keep everything orderly and compatible, bands are 
                usually divided into a number of distinct channels."
            }
            li { "Each channel is known by a channel number and is assigned to a specific frequency." }
            li { "The 5-GHz band consists of nonoverlapping channels." }
            li {
                "The 2.4-GHz band is the opposite of 5-GHz band, and it is consits of overlapping channels. 
                The only way to avoid any overlap between adjacent channels is to configure APs to use only channels 1, 6, and 11."
            }
            li {
                "Because each channel in 2.4-GHz band is too wide, it covers the frequency range that is allocated to more than four 
                consecutive channels!"
            }
        }
    }
}