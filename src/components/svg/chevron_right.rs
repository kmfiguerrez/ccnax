use dioxus::prelude::*;

#[component]
pub fn ChevronRightSVG() -> Element {
    rsx! {
        svg {
            class: "accordion-expand-icon h-6 w-6",
            fill: "none",
            view_box: "0 0 24 24",
            xmlns: "http://www.w3.org/2000/svg",
            g { id: "SVGRepo_bgCarrier", stroke_width: "0" }
            g {
                id: "SVGRepo_tracerCarrier",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
            g { id: "SVGRepo_iconCarrier",
                path {
                    d: "M9 6L15 12L9 18",
                    stroke: "white",
                    stroke_linecap: "round",
                    stroke_linejoin: "round",
                    stroke_width: "2",
                }
            }
        }
    }
}

