use dioxus::prelude::*;

/// A config checklist component that can be used to display a configuration checklist.
/// 
/// Renders a paragraph element.
#[component]
pub fn ConfigChecklist(children: Element) -> Element {
    rsx! {
        p { class: "text-xs bg-purple-900 w-fit border border-black rounded-lg px-2",
            "Config"
            br {}
            "Checklist"
        }
    }
}