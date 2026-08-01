use dioxus::prelude::*;

use crate::components::{button::Button, card::{Card, CardContent, CardDescription, CardFooter, CardHeader, CardTitle}, input::Input};

#[component]
pub fn CcnaBookPage() -> Element {
    let mut volume: Signal<u8> = use_signal(|| 1);
    let mut book_page: Signal<u16> = use_signal(|| 1);
    let mut result_page: Signal<u16> = use_signal(|| 0);


    rsx! {
        Card {
            CardHeader {
                CardTitle { "CCNA Book Page" }
                CardDescription {
                    "Enter a volume and a book page number to get the page number to use in page navigator"
                }
            }
            CardContent {
                div { class: "flex flex-col gap-y-4 mb-4 sm:flex-row sm:justify-between sm:gap-x-4",
                    div { class: "flex flex-col sm:w-1/2",
                        label { r#for: "volume", "Volume" }
                        Input {
                            class: "",
                            id: "volume",
                            r#type: "number",
                            min: 1,
                            max: 2,
                            value: volume,

                            oninput: move |e: FormEvent| {
                                if let Ok(parsed) = e.value().parse::<u8>() {
                                    volume.set(parsed);
                                }
                            },
                        }
                    }
                    div { class: "flex flex-col sm:w-1/2",
                        label { r#for: "page", "Page" }
                        // input {
                        //     id: "page",
                        //     class: "border",
                        //     r#type: "number",
                        //     min: 1,
                        //     value: book_page,
                        //     oninput: move |e: FormEvent| {
                        //         if let Ok(parsed) = e.value().parse::<u16>() {
                        //             book_page.set(parsed);
                        //         }
                        //     },
                        // }
                        Input {
                            class: "",
                            id: "page",
                            r#type: "number",
                            min: 1,
                            // max: 2,
                            value: book_page,
                            oninput: move |e: FormEvent| {
                                if let Ok(parsed) = e.value().parse::<u16>() {
                                    book_page.set(parsed);
                                }
                            },
                        }
                    
                    }
                }
                div { class: "flex flex-col items-center",
                    span { class: "text-xl", "Result:" }
                    span {
                        class: "text-xl",
                        class: if result_page() == 0 { "invisible" },
                        "{result_page}"
                    }
                }
            }
            CardFooter {
                Button {
                    onclick: move |_e| {
                        if volume() == 1 as u8 {
                            return result_page.set(book_page + 54);
                        }

                        result_page.set(book_page + 46);
                    },
                    "Get Page"
                }
            }
        }

        // Will replicate the dioxus card component.
        // Card Wrapper
        // div { class: "flex flex-col gap-[1.5rem] p-[1.5rem] border border-[#a1a1a1] rounded-lg  w-full sm:w-sm",
        //     // Card Header
        //     div {
        //         // Card title
        //         div { class: "text-base font-semibold", "CCNA Book Page" }
        //         // Card description
        //         div { class: "text-sm text-[#a1a1a1]",
        //             "Enter a volume and a book page number to get the page number to use in page navigator."
        //         }
        //     }
        //     // Card content
        //     div { class: "",
        //         div { class: "flex flex-col gap-y-4 mb-4 sm:flex-row sm:justify-between sm:gap-x-4",
        //             div { class: "flex flex-col sm:w-1/2",
        //                 label { r#for: "volume", "Volume" }
        //                 Input {
        //                     class: "",
        //                     id: "volume",
        //                     r#type: "number",
        //                     min: 1,
        //                     max: 2,
        //                     value: volume,

        //                     oninput: move |e: FormEvent| {
        //                         if let Ok(parsed) = e.value().parse::<u8>() {
        //                             volume.set(parsed);
        //                         }
        //                     },
        //                 }
        //             }
        //             div { class: "flex flex-col sm:w-1/2",
        //                 label { r#for: "page", "Page" }
        //                 // input {
        //                 //     id: "page",
        //                 //     class: "border",
        //                 //     r#type: "number",
        //                 //     min: 1,
        //                 //     value: book_page,
        //                 //     oninput: move |e: FormEvent| {
        //                 //         if let Ok(parsed) = e.value().parse::<u16>() {
        //                 //             book_page.set(parsed);
        //                 //         }
        //                 //     },
        //                 // }
        //                 Input {
        //                     class: "",
        //                     id: "page",
        //                     r#type: "number",
        //                     min: 1,
        //                     // max: 2,
        //                     value: book_page,
        //                     oninput: move |e: FormEvent| {
        //                         if let Ok(parsed) = e.value().parse::<u16>() {
        //                             book_page.set(parsed);
        //                         }
        //                     },
        //                 }

        //             }
        //         }
        //         div { class: "flex flex-col items-center",
        //             span { class: "text-xl", "Result:" }
        //             span {
        //                 class: "text-xl",
        //                 class: if result_page() == 0 { "invisible" },
        //                 "{result_page}"
        //             }
        //         }
        //     }
        //     // Card footer
        //     div {
        //         button {
        //             class: "px-4 py-2 inline-flex items-center justify-center rounded-md text-sm font-medium transition-colors focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-zinc-400 disabled:pointer-events-none disabled:opacity-50  w-full sm:w-auto bg-zinc-100 text-zinc-950 hover:bg-zinc-200 active:bg-zinc-300",
        //             onclick: move |_e| {
        //                 if volume() == 1 as u8 {
        //                     return result_page.set(book_page + 54);
        //                 }

        //                 result_page.set(book_page + 46);
        //             },
        //             "Get Page"
        //         }
        //     }
        // }
    }
}
