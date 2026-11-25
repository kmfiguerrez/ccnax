use dioxus::prelude::*;

use crate::{utils::h3_heading, components::{KeyTopic, GreenNote}};

#[component]
pub fn SectionIntroductionContent() -> Element {
    rsx! {
        p { class: "mb-4",
            "This chapter covers a variety of authentication methods and encryption and message integrity algorithms."
            br {}
            "When it comes time to configure a WLAN with wireless security, should you try to select some combination of schemes based on 
            which one is best or which one is not deprecated?"
            br {}
            "Which authentication methods are compatible with which encryption algorithms?"
        }

        {h3_heading("Wi-Fi Alliance WPA Certifications")}
        p { class: "mb-4",
            "The Wi-Fi Alliance (http://wi-fi.org), a nonprofit wireless industry association, has worked out straightforward ways to do 
            that through its Wi-Fi Protected Access (WPA) industry certifications."
            br {}
            "To date, there are three different versions: WPA, WPA2, and WPA3."
            br {}
            "Wireless products are tested in authorized testing labs against stringent criteria that represent correct implementation of 
            a standard."
            br {}
            strong {
                "As long as the Wi-Fi Alliance has certified a wireless client device and an AP and its associated WLC for the same WPA 
                version, they should be compatible and offer the same security components."
            }
        }

        {h3_heading("WPA")}
        p { class: "mb-4",
            "The Wi-Fi Alliance introduced its first generation WPA certification (known simply as WPA and not WPA1) while the IEEE 
            802.11i amendment for best practice security methods was still being developed."
            br {}
            "WPA was based on parts of 802.11i and included 802.1x authentication, TKIP, and a method for dynamic encryption key 
            management."
        }

        {h3_heading("WPA2")}
        p { class: "mb-4",
            "Once 802.11i was ratified and published, the Wi-Fi Alliance included it in full in its WPA Version 2 (WPA2) certification."
            br {}
            "WPA2 is based around the superior AES CCMP algorithms,
            rather than the deprecated TKIP from WPA. It should be obvious that WPA2 was meant as a
            replacement for WPA."
        }

        {h3_heading("WPA3")}
        p { class: "mb-4",
            "In 2018, the Wi-Fi Alliance introduced WPA Version 3 (WPA3) as a future replacement for WPA2, adding several important and 
            superior security mechanisms."
            br {}
            "WPA3 leverages stronger encryption by AES with the Galois/Counter Mode Protocol (GCMP)."
            br {}
            "It also uses Protected Management Frames (PMF) to secure important 802.11 management frames between APs and clients, to 
            prevent malicious activity that might spoof or tamper with a BSS's operation."
        }

        p { class: "mb-4",
            "Table 28-2 summarizes the basic differences between WPA, WPA2, and WPA3."
            br {}
            strong {
                "Each successive version is meant to replace prior versions by offering better security features."
            }
            br {}
            "You should avoid using WPA and use WPA2 instead—at least until WPA3 becomes widely available on wireless client devices, 
            APs, and WLCs."
        }

        KeyTopic {}
        img {
            class: "rounded-lg",
            alt: "Table 28-2 Comparing WPA, WPA2, and WPA3",
            loading: "lazy",
            src: asset!("/assets/static/v1p8c28s4t28-2.png", AssetOptions::image().with_avif()),
        }
        p { class: "text-sm mb-4",
            "* WPA3 includes other features beyond WPA and WPA2, such as Simultaneous Authentication of Equals (SAE), Forward secrecy, 
            and Protected management frames (PMF)."
        }

        {h3_heading("Personal vs Enterprise mode")}
        p { class: "mb-4",
            "Notice that all three WPA versions support two client authentication modes: a pre-shared key (PSK) or 802.1x, based on the 
            scale of the deployment."
            br {}
            "These are also known as "
            i { "personal mode" }
            " and "
            i { "enterprise mode" }
            ", respectively."
            br {}
            "With personal mode, a key string must be shared or configured on every client and AP before the clients can connect to the 
            wireless network."
            br {}
            "The pre-shared key is normally kept confidential so that unauthorized users have no knowledge of it."
            br {}
            "The key string is never sent over the air."
            br {}
            "Instead, clients and APs work through a four-way handshake procedure that uses the pre-shared key string to construct
            and exchange encryption key material that can be openly exchanged."
            br {}
            "Once that process is successful, the AP can authenticate the client and the two can secure data frames that are
            sent over the air."
        }

        {h3_heading("WPA and WPA2-Personal vulnerabilities")}
        p { class: "mb-4",
            "With WPA-Personal and WPA2-Personal modes, a malicious user can eavesdrop and capture the four-way handshake between a 
            client and an AP."
            br {}
            "That user can then use a dictionary attack to automate guessing the pre-shared key."
            br {}
            "If he is successful, he can then decrypt the wireless data or even join the network posing as a legitimate user."
        }

        {h3_heading("WPA3-Personal strengths")}
        p { class: "mb-4",
            "WPA3-Personal avoids such an attack by strengthening the key exchange between clients and APs through a method known as 
            Simultaneous Authentication of Equals (SAE)."
            br {}
            "Rather than a client authenticating against a server or AP, the client and AP can initiate the authentication process 
            equally and even simultaneously."
        }

        p { class: "mb-4",
            "Even if a password or key is compromised, WPA3-Personal offers forward secrecy, which prevents attackers from being able to 
            use a key to unencrypt data that has already been transmitted over the air."
        }

        GreenNote {
            p {
                strong { "NOTE" }
                " The Personal mode of any WPA version is usually easy to deploy in a small environment or with clients that are embedded 
                in certain devices because a simple text key string is all that is needed to authenticate the clients. Be aware that every 
                device using the WLAN must be configured with an identical pre-shared key. If you ever need to update or change the key, 
                you must touch every device to do so. As well, the pre-shared key should remain a well kept secret; you should never 
                divulge the pre-shared key to any unauthorized person."
            }
        }

        {h3_heading("WPAs Enterprise mode")}
        p { class: "mb-4",
            "Notice from Table 28-2 that WPA, WPA2, and WPA3 also support 802.1x or enterprise authentication."
            br {}
            "This implies EAP-based authentication, but the WPA versions do not require any specific EAP method."
            br {}
            "Instead, the Wi-Fi Alliance certifies interoperability with wellknown EAP methods like EAP-TLS, PEAP, EAP-TTLS, and EAP-SIM."
            br {}
            "Enterprise authentication is more complex to deploy than personal mode because authentication servers must be
            set up and configured as a critical enterprise resource."
        }

        GreenNote {
            p {
                strong { "NOTE" }
                " The Wi-Fi Alliance has made wireless security configuration straightforward and consistent through its WPA, WPA2, and 
                WPA3 certifications."
                br {}
                "Each version is meant to replace its predecessors because of improved security mechanisms."
                br {}
                "You should always select the highest WPA version that the clients and wireless infrastructure in your environment will
                support."
            }
        }

        {h3_heading("RECAP")}
        ol { class: "list-disc list-inside",
            li {
                "The Wi-Fi Alliance has made wireless security configuration straightforward and consistent through its WPA, WPA2, and 
                WPA3 certifications."
            }
            li {
                "Each WPA version is meant to replace its predecessors because of improved security mechanisms."
            }
            li { "WPA and WPA have vulnerabilities and must avoid using them if WPA3 is available." }
            li {
                "The pre-shared key should remain a well kept secret; you should never divulge the pre-shared key to any unauthorized 
                person."
            }
            li {
                "You should always select the highest WPA version that the clients and wireless infrastructure in your environment will 
                support."
            }
        }
    }
}