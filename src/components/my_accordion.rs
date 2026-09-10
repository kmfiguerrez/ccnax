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
pub fn Accordion(#[props(default)] class: String, children: Element) -> Element {
    rsx! {
        div { class: "{class}", {children} }
    }
}


#[derive(Clone, Copy)]
struct AccordionItemState {
    open: Signal<bool>,
    content_id: Signal<String>,
}
use std::sync::atomic::{AtomicUsize, Ordering};

static ACCORDION_ID: AtomicUsize = AtomicUsize::new(0);

fn next_accordion_id() -> String {
    let id = ACCORDION_ID.fetch_add(1, Ordering::Relaxed);

    format!("accordion-content-{id}")
}
#[component]
pub fn AccordionItem(children: Element) -> Element {
    let open = use_signal(|| false);
    let content_id = use_signal(next_accordion_id);

    use_context_provider(|| AccordionItemState {
        open,
        content_id,
    });  

    rsx! {
        div { class: "accordion-item", "data-open": if open() { "true" } else { "false" }, {children} }
    }
}

#[component]
pub fn AccordionTrigger(children: Element) -> Element {
    let mut state = use_context::<AccordionItemState>();

    // aria-controls map to accordion content id.
    rsx! {
        button {
            "aria-expanded": (state.open)(), // state.open is inside a paranthesis because a signal is inside a struct.
            "aria-controls": (state.content_id)(), // state.content_id is inside a paranthesis because a signal is inside a struct.
            onclick: move |_| state.open.toggle(),
            class: "accordion-trigger max-sm:text-left",
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
            id: (state.content_id)(),
            class: "accordion-content",
            "data-open": if *state.open.read() { "true" } else { "false" },
            div { {children} }
        }
    }
}