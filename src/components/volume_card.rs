use dioxus::prelude::*;

use crate::components::card::{Card, CardContent};

#[component]
pub fn VolumeCard(number: u8) -> Element {
    rsx! {
        Card { class: "max-w-fit border! border-zinc-500! bg-transparent! rounded-xl!",
            CardContent { class: "flex items-center gap-x-2",
                img {
                    src: asset!("/assets/static/cisco-logo.svg"),
                    alt: "cisco logo",
                    class: "h-15 w-15",
                }
                div {
                    h5 { class: "text-2xl font-semibold text-[#ffffff]", "Volume {number}" }
                    p { class: "text-sm", "CCNA" }
                }
            }
        }
    }
}