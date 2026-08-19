use dioxus::prelude::*;

#[component]
pub fn CaretRightSVG() -> Element {
    rsx! {
        svg {
            class: "h-7 w-7",
            fill: "#ffffff",
            stroke: "#000000",
            view_box: "0 0 32 32",
            xmlns: "http://www.w3.org/2000/svg",
            g { id: "SVGRepo_bgCarrier", stroke_width: "0" }
            g {
                id: "SVGRepo_tracerCarrier",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
            g { id: "SVGRepo_iconCarrier",
                path { d: "M11.303 8l11.394 7.997L11.303 24z" }
            }
        }
    }
}
