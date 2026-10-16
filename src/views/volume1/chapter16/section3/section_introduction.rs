use dioxus::prelude::*;

use crate::components::GreenNote;

#[component]
pub fn SectionIntroductionContent() -> Element {
    rsx! {
        p { class: "mb-4",
            "All routers add connected routes, as discussed in the previous section."
            br {}
            "Then, most networks use dynamic routing protocols to cause each router to learn the rest of the routes in an
            internetwork."
            br {}
            "Networks use static routes—routes added to a routing table through direct configuration—much less often than dynamic 
            routing."
            br {}
            "However, static routes can be useful at times, and they happen to be useful learning tools as well."
            br {}
            "This next major section in the chapter discusses static routes."
        }

        GreenNote {
            p {
                strong { "NOTE" }
                " The CCNA 200-301 exam topic 3.2 breaks IPv4 (and IPv6) static routes into four subtopics: network routes, 
                host routes, floating static routes, and default routes. This section explains all four types as noted in the 
                upcoming headings."
            }
        }
    }
}