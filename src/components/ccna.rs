use dioxus::prelude::*;

#[component]
pub fn CcnaBookPage() -> Element {
    let mut volume: Signal<u8> = use_signal(|| 1);
    let mut book_page: Signal<u16> = use_signal(|| 1);
    let mut result_page: Signal<u16> = use_signal(|| 0);


    rsx! {
        // Will replicate the dioxus card component.
        // Card Wrapper
        div { class: "flex flex-col gap-[1.5rem] p-[1.5rem] border border-[#a1a1a1] rounded-lg  w-full sm:w-sm",
            // Card Header
            div {
                // Card title
                div { class: "text-base font-semibold", "CCNA Book Page" }
                // Card description
                div { class: "text-sm text-[#a1a1a1]",
                    "Enter a volume and a book page number to get the page number to use in page navigator."
                }
            }
            // Card content
            div { class: "",
                div { class: "flex flex-col mb-4 sm:flex-row sm:justify-between",
                    // Only this div has sm:w-full because the Volume input shrinks in width when the max attribute is in place.
                    div { class: "flex flex-col sm:base-1/2 sm:w-full",
                        label { r#for: "volume", "Volume" }
                        input {
                            id: "volume",
                            class: "border sm:w-full",
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
                    div { class: "flex flex-col sm:base-1/2",
                        label { r#for: "page", "Page" }
                        input {
                            id: "page",
                            class: "border",
                            r#type: "number",
                            min: 1,
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
            // Card footer
            div {
                button {
                    class: "px-4 py-2 inline-flex items-center justify-center rounded-md text-sm font-medium transition-colors focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-zinc-400 disabled:pointer-events-none disabled:opacity-50  w-full sm:w-auto bg-zinc-100 text-zinc-950 hover:bg-zinc-200 active:bg-zinc-300",
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
    }
}