use dioxus::prelude::*;

#[component]
pub fn Content() -> Element {
    rsx! {
        p { class: "mb-4",
            "IOS defines the format of log messages."
            br {}
            "The message begins with some data fields about the message, followed by some text more easily read by humans."
            br {}
            "For example, take a close look at this sample message:"
            br {}
            span { class: "text-gray-300",
                "*Dec 18 17:10:15.079: %LINEPROTO-5-UPDOWN: Line protocol on Interface FastEthernet0/0, changed state to down"
            }
        }

        p { class: "mb-1", "Notice that by default on this particular device, we see the following:" }
        ol { class: "mb-4",
            li {
                span { class: "font-semibold mr-1", "A timestamp:" }
                span { "*Dec 18 17:10:15.079" }
            }
            li {
                span { class: "font-semibold mr-1",
                    "The facility on the router that generated the message:"
                }
                span { "%LINEPROTO" }
            }
            li {
                span { class: "font-semibold mr-1", "The severity level:" }
                span { "5" }
            }
            li {
                span { class: "font-semibold mr-1", "A mnemonic for the message:" }
                span { "UPDOWN" }
            }
            li {
                span { class: "font-semibold mr-1", "The description of the message:" }
                span { "Line protocol on Interface FastEthernet0/0, changed state to down" }
            }
        }

        p { class: "mb-3",
            "IOS dictates most of the contents of the messages, but you can at least toggle on and off
            the use of the timestamp (which is included by default) and a log message sequence number
            (which is not enabled by default)."
            br {}
            "Example 9-1 reverses those defaults by turning off timestamps and turning on sequence numbers."
        }
        img { src: asset!("/assets/static/v2p3c9s1sh3ex9-1.png", AssetOptions::image().with_avif()) }

    }
}