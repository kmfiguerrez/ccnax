use dioxus::prelude::*;

use crate::utils::h3_heading;

#[component]
pub fn SectionIntroductionContent() -> Element {
    rsx! {
        p { class: "mb-4",
            "Some CCNA topics, like ACLs, simply require more drills and practice than others."
            br {}
            "ACLs require you to think of parameters to match ranges of numbers, and that of course requires
            some use of math and some use of processes."
        }

        p { class: "mb-4",
            "This section provides some practice problems and tips, from two perspectives."
            br {}
            "First, this section asks you to build one-line standard ACLs to match some packets."
            br {}
            "Second, this section asks you to interpret existing ACL commands to describe what packets the ACL will match."
            br {}
            "Both skills are useful for the exams."
        }
    }
}