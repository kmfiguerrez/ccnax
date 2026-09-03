use dioxus::prelude::*;

/// A config checklist component that can be used to display a configuration checklist.
/// 
/// Renders a paragraph element.
#[component]
pub fn KeyTopic(children: Element) -> Element {
    rsx! {
        p { class: "leading-4 bg-orange-400 w-fit border border-black rounded-lg px-1 py-1",
            "Key"
            br {}
            "Topic"
        }
    }
}