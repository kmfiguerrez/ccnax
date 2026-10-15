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


#[derive(Clone, Copy)]
struct AccordionState {
    open_item: Signal<Option<String>>,
}
#[component]
pub fn Accordion(#[props(default)] class: String, children: Element) -> Element {
    let open_item = use_signal(|| None::<String>);

    use_context_provider(|| AccordionState { open_item });

    rsx! {
        div { class: "{class}", {children} }
    }
}


#[derive(Clone, Copy)]
struct AccordionItemState {
    id: Signal<String>,
    accordion: AccordionState,
}
use std::sync::atomic::{AtomicUsize, Ordering};

static ACCORDION_ID: AtomicUsize = AtomicUsize::new(0);

fn next_accordion_id() -> String {
    let id = ACCORDION_ID.fetch_add(1, Ordering::Relaxed);

    format!("accordion-content-{id}")
}
#[component]
pub fn AccordionItem(children: Element) -> Element {
    let accordion = use_context::<AccordionState>();
    let id = use_signal(next_accordion_id);
    let is_open = accordion.open_item.read().as_ref() == Some(&id());

    use_context_provider(|| AccordionItemState {
        id,
        accordion,
    });

    rsx! {
        div { class: "accordion-item", "data-open": if is_open { "true" } else { "false" }, {children} }
    }
}

#[component]
pub fn AccordionTrigger(children: Element) -> Element {
    let item = use_context::<AccordionItemState>();

    // aria-controls map to accordion content id.
    rsx! {
        button {
            "aria-expanded": item.accordion.open_item.read().as_ref() == Some(&(item.id)()), // item.id is inside a paranthesis because a signal is inside a struct.
            "aria-controls": (item.id)(), // item.id is inside a paranthesis because a signal is inside a struct.
            class: "accordion-trigger max-sm:text-left",
            onclick: move |_| {
                let current_id = (item.id)();

                let mut open_item = item.accordion.open_item;

                if open_item() == Some(current_id.clone()) {
                    // Clicking the open item closes it.
                    open_item.set(None);
                } else {
                    // Opening this item replaces the previous ID.
                    open_item.set(Some(current_id));
                }
            },
            {children}
            ChevronRightSVG {}
        }
    }
}


#[component]
pub fn AccordionContent(children: Element) -> Element {
    let item = use_context::<AccordionItemState>();
    let is_open = item.accordion.open_item.read().as_ref() == Some(&(item.id)());    

    rsx! {
        div {
            id: (item.id)(),
            class: "accordion-content",
            "data-open": if is_open { "true" } else { "false" },
            div { {children} }
        }
    }
}