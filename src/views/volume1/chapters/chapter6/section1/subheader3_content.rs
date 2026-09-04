use dioxus::prelude::*;

use crate::utils::h3_heading;

#[component]
pub fn Content() -> Element {
    rsx! {
        p { class: "mb-4",
            "Using a username/password configured directly on the switch causes some administrative headaches."
            br {}
            "For instance, every switch and router needs the configuration for all
            users who might need to log in to the devices."
            br {}
            "Then, when any changes need to happen, like an occasional change to the passwords for good security practices, 
            the configuration of all devices must be changed."
        }

        p { class: "mb-4",
            "A better option would be to use tools like those used for many other IT login functions."
            br {}
            "Those tools allow for a central place to securely store all username/password pairs, with
            tools to make users change their passwords regularly, tools to revoke users when they leave
            their current jobs, and so on."
        }

        p { class: "mb-4",
            "Cisco switches allow exactly that option using an external server called an authentication,
            authorization, and accounting (AAA) server."
            br {}
            "These servers hold the usernames/passwords."
            br {}
            "Typically, these servers allow users to do self-service and forced maintenance to their passwords."
            br {}
            "Many production networks use AAA servers for their switches and routers today."
        }

        p { class: "mb-4",
            "The underlying login process requires some additional work on the part of the switch for
            each user login, but once set up, the username/password administration is much less."
            br {}
            "When using a AAA server for authentication, the switch (or router) simply sends a message to the
            AAA server asking whether the username and password are allowed, and the AAA server
            replies."
            br {}
            "Figure 6-4 shows an example, with the user first supplying a username/password,
            the switch asking the AAA server, and the server replying to the switch stating that the username/password is valid."
        }

        img {
            class: "mb-4 rounded-lg",
            alt: "Figure 6-4 Basic Authentication Process with an External AAA Server",
            loading: "lazy",
            src: asset!("/assets/static/v1p2c6s1sh3f6-4.png", AssetOptions::image().with_avif()),
        }

        p { class: "mb-4",
            "While the figure shows the general idea, note that the information flows with a couple
            of different protocols."
            "On the left, the connection between the user and the switch or router uses Telnet or SSH."
            "On the right, the switch and AAA server typically use either the
            RADIUS or TACACS+ protocol, both of which encrypt the passwords as they traverse the
            network."
        }

        {h3_heading("RECAP")}
        ol { class: "list-disc list-inside",
            li {
                "Cisco allows using an external server called an authentication, authorization, and accounting (AAA) server
                to store usernames/passwords."
            }
            li { "Many production networks use AAA servers for their switches and routers today." }
        }
    }
}