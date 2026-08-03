use dioxus::{logger::tracing, prelude::*};

use crate::components::{button::Button, card::{Card, CardContent, CardDescription, CardFooter, CardHeader, CardTitle}, input::Input};

#[component]
pub fn CcnaBookPage() -> Element {
    let mut volume: Signal<u8> = use_signal(|| 1);
    let mut book_page: Signal<u16> = use_signal(|| 1);
    let mut result_page: Signal<u16> = use_signal(|| 0);
    // let mut disable_button: Signal<bool> = use_signal(|| true);

    let is_volume_valid: Memo<bool> = use_memo(move || {
        let v = volume();
        if v < 1 || v > 2 {
            // Reset the result_page to 0 if the volume is invalid
            result_page.set(0);
            return false;
        }
        true
    });
    let is_book_page_valid: Memo<bool> = use_memo(move || {
        if book_page() < 1 {
            // Reset the result_page to 0 if the book page is invalid
            result_page.set(0);
            return false;
        }
        true
    });

    rsx! {
        Card { style: "width: 100%; max-width: 24rem;",
            CardHeader {
                CardTitle { "CCNA Book Page" }
                CardDescription {
                    "Enter a volume and a book page number to get the page number to use in page navigator"
                }
            }
            CardContent {
                div { class: "flex flex-col gap-y-4 mb-4 sm:flex-row sm:justify-between sm:gap-x-4",
                    div { class: "flex flex-col sm:w-1/2",
                        label {
                            class: if !is_volume_valid() { "text-red-500 font-semibold" },
                            r#for: "volume",
                            "Volume"
                        }
                        Input {
                            class: "invalid:outline! invalid:outline-offset-2! invalid:outline-red-500!",
                            id: "volume",
                            r#type: "number",
                            min: 1,
                            max: 2,
                            name: "volume",
                            value: volume,
                            oninput: move |e: FormEvent| {
                                if let Ok(parsed) = e.value().parse::<u8>() {
                                    volume.set(parsed);
                                    // tracing::info!("Book page updated to {}", * is_book_page_valid.peek());
                                }
                            },
                        }
                        // Validator
                        if !is_volume_valid() {
                            span { class: "text-red-500 text-sm", "Volume must be 1 or 2" }
                        }
                    }
                    div { class: "flex flex-col sm:w-1/2",
                        label {
                            class: if !is_book_page_valid() { "text-red-500 font-semibold" },
                            r#for: "page",
                            "Page"
                        }
                        Input {
                            class: "invalid:outline! invalid:outline-offset-2! invalid:outline-red-500!",
                            id: "page",
                            r#type: "number",
                            min: 1,
                            name: "page",
                            value: book_page,
                            oninput: move |e: FormEvent| {
                                if let Ok(parsed) = e.value().parse::<u16>() {
                                    book_page.set(parsed);
                                }
                                else {
                                    book_page.set(0);
                                }
                            },
                        }
                        // Validator
                        if !is_book_page_valid() {
                            span { class: "text-red-500 text-sm", "Page must be positive integers" }
                        }
                    }
                }
                // Result
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
                    class: "max-sm:w-full",
                    disabled: if !is_volume_valid() || !is_book_page_valid() { true } else { false },
                    // disabled: disable_button(),
                    onclick: move |_e| {
                        if volume() == 1 as u8 {
                            return result_page.set(book_page + 54 as u16);
                        }

                        result_page.set(book_page + 46);
                    },
                    "Get Page"
                }
            }
        }
    }
}
