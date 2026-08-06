use dioxus::prelude::*;

/// A green note component that can be used to display important information in a visually distinct way.
/// 
/// Renders div element as a wrapper.
#[component]
pub fn GreenNote(children: Element) -> Element {
    rsx! {
        div { class: "bg-green-700 max-w-max px-4 py-2 rounded-lg text-black mb-4",
            {children}
        }
    }
}