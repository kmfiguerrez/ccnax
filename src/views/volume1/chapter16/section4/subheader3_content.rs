use dioxus::prelude::*;

use crate::{
    components::KeyTopic,
    utils::{text_command, TextCommandColor}
};

#[component]
pub fn Content() -> Element {
    rsx! {
        p { class: "mb-4",
            "The "
            {text_command("show ip route", TextCommandColor::Gold)}
            " command plays a huge role in verifying and troubleshooting IP routing and addressing."
            br {}
            "This final topic of the chapter pulls the concepts together in one place for easier reference and study."
        }

        p { class: "mb-4",
            "Figure 16-15 shows the output of a sample "
            {text_command("show ip route", TextCommandColor::Gold)}
            " command."
            br {}
            "The figure numbers various parts of the command output for easier reference, with Table 16-3 describing the
            output noted by each number."
        }

        img {
            class: "mb-4 rounded-lg",
            alt: "Figure 16-15 show ip route Command Output Reference",
            loading: "lazy",
            src: asset!("/assets/static/v1p5c16s4sh3ex16-15.png", AssetOptions::image().with_avif()),
        }

        KeyTopic {}
        img {
            class: "mb-4 rounded-lg",
            alt: "Table 16-3 Descriptions of the show ip route Command Output",
            loading: "lazy",
            src: asset!("/assets/static/v1p5c16s4sh3t16-3.png", AssetOptions::image().with_avif()),
        }

    }
}