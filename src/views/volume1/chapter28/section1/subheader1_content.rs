use dioxus::prelude::*;

use crate::utils::h3_heading;

#[component]
pub fn Content() -> Element {
    rsx! {
        p { class: "mb-4",
            "To use a wireless network, clients must first discover a basic service set (BSS) and then request permission to associate 
            with it."
            br {}
            "Clients should be authenticated by some means before they can become functioning members of the wireless LAN. Why?"
        }

        {h3_heading("Reasons for authenticating wireless clients")}
        p { class: "mb-4",
            "Suppose that your wireless network connects to corporate resources where confidential information can be accessed."
            br {}
            "In that case, only devices known to be trusted and expected should be given access."
            br {}
            "Guest users, if they are permitted at all, should be allowed to join a different guest WLAN where they can access 
            nonconfidential or public resources."
            br {}
            "Rogue clients, which are not expected or welcomed, should not be permitted to associate at all."
            br {}
            "After all, they are not affiliated with the corporate network and are likely to be unknown devices that happen to be within 
            range of your network."
        }

        p { class: "mb-4",
            strong {
                "To control access, wireless networks can authenticate the client devices before they are allowed to associate."
            }
            br {}
            "Potential clients must identify themselves by presenting some form of credentials to the APs."
            br {}
            "Figure 28-2 shows the basic client authentication process."
        }

        {h3_heading("Wireless authentication methods")}
        p { class: "mb-4",
            "Wireless authentication can take many forms."
            br {}
            "Some methods require only a static text string that is common across all trusted clients and APs."
            br {}
            "The text string is stored on the client device and presented directly to the AP when needed."
            br {}
            "What might happen if the device was stolen or lost?"
            br {}
            "Most likely, any user who possessed the device could still authenticate to the network."
            br {}
            "Other more stringent authentication methods require interaction with a corporate user database."
            br {}
            "In those cases, the end user must enter a valid username and password—something that would not be known to a thief or an 
            imposter."
        }

        img {
            class: "mb-4 rounded-lg",
            alt: "Figure 28-2 Authenticating a Wireless Client",
            loading: "lazy",
            src: asset!("/assets/static/v1p8c28s1sh1f28-2.png", AssetOptions::image().with_avif()),
        }

        p { class: "mb-4",
            "If you have ever joined a wireless network, you might have focused on authenticating your device or yourself, while 
            implicitly trusting the nearest AP."
            br {}
            "For example, if you turn on your wireless device and find a wireless network that is available at your workplace, you 
            probably join it without hesitating."
            br {}
            "The same is true for wireless networks in an airport, a hotel, a hot spot, or in your home—you expect the AP that is 
            advertising the SSID to be owned and operated by the entity where you are located."
            br {}
            "But how can you be sure?"
        }

        p { class: "mb-4",
            "Normally, the only piece of information you have is the SSID being broadcast or advertised by an AP."
            br {}
            "If the SSID looks familiar, you will likely choose to join it."
            br {}
            "Perhaps your computer is configured to automatically connect to a known SSID so that it associates without your
            intervention."
            br {}
            "Either way, you might unwittingly join the same SSID even if it was being advertised by an imposter."
        }

        {h3_heading("Common Attacks on Wireless Networks")}
        p { class: "mb-4",
            "Some common attacks focus on a malicious user pretending to be an AP."
            br {}
            "The fake AP can send beacons, answer probes, and associate clients just like the real AP it is impersonating."
            br {}
            "Once a client associates with the fake AP, the attacker can easily intercept all communication to and from the client from 
            its central position."
            br {}
            "A fake AP could also send spoofed management frames to disassociate or deauthenticate legitimate and active clients, just to 
            disrupt normal network operation."
        }

        {h3_heading("Authenticating the AP")}
        p { class: "mb-4",
            "To prevent this type of man-in-the-middle attack, the client should authenticate the AP before the client itself is 
            authenticated."
            br {}
            "Figure 28-3 shows a simple scenario."
            br {}
            "Even further, any management frames received by a client should be authenticated too, as proof that they
            were sent by a legitimate and expected AP."
        }

        img {
            class: "mb-4 rounded-lg",
            alt: "Figure 28-3 Authenticating a Wireless AP",
            loading: "lazy",
            src: asset!("/assets/static/v1p8c28s1sh1f28-3.png", AssetOptions::image().with_avif()),
        }

        {h3_heading("RECAP")}
        ol { class: "list-disc list-inside",
            li {
                "The main purpose of autheniticating wireless clients is to protect confidential informations."
            }
            li { "Wireless authentication can take many forms." }
            li {
                "To prevent common attacks on wireless networks, like the man-in-the-middle attack, the client should authenticate the AP 
                before the client itself is authenticated."
            }
            li {
                "Even further, any management frames received by a client should be authenticated too, as proof that they were sent by a 
                legitimate and expected AP."
            }
        }
    }
}