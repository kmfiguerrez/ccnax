use dioxus::prelude::*;

#[component]
pub fn Content() -> Element {
    rsx! {
        p { class: "mb-4",
            "When you enter commands from the CLI, the switch saves the last several commands in the
            history buffer."
            br {}
            "Then, as mentioned in Chapter 4, “Using the Command-Line Interface,” you
            can use the up-arrow key or press Ctrl+P to move back in the history buffer to retrieve a
            command you entered a few commands ago."
            br {}
            "This feature makes it very easy and fast to use a set of commands repeatedly."
            br {}
            "Table 6-2 lists some of the key commands related to the history buffer"
        }

        img {
            class: "mb-4 rounded-lg",
            alt: "Table 6-2 Commands Related to the History Buffer",
            loading: "lazy",
            src: asset!("/assets/static/v1p2c6s3sh1t6-2.png", AssetOptions::image().with_avif()),
        }
    }
}