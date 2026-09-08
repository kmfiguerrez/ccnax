use dioxus::prelude::*;

use crate::components::svg::{ChevronRightSVG};

// Desired
// 
// Accordion
//  AccordionItem
//      AccordionTrigger
//      AccordionContent
//  AccordionItem
// Accordion


#[component]
pub fn Accordion(children: Element) -> Element {
    rsx! {
        div { {children} }
    }
}


#[derive(Clone, Copy)]
struct AccordionItemState {
    open: Signal<bool>,
}
#[component]
pub fn AccordionItem(children: Element) -> Element {
    let open = use_signal(|| false);

    use_context_provider(|| AccordionItemState { open });   

    rsx! {
        div { class: "accordion-item", "data-open": if open() { "true" } else { "false" }, {children} }
    }
}

#[component]
pub fn AccordionTrigger(children: Element) -> Element {
    let mut state = use_context::<AccordionItemState>();

    rsx! {
        button {
            onclick: move |_| state.open.toggle(),
            class: "flex items-center gap-x-1",
            {children}
            ChevronRightSVG {}
        }
    }
}


#[component]
pub fn AccordionContent(children: Element) -> Element {
    let state = use_context::<AccordionItemState>();

    rsx! {
        div {
            class: "accordion-content",
            "data-open": if *state.open.read() { "true" } else { "false" },
            div { {children} }
        }
    }
}