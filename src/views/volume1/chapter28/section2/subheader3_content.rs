use dioxus:: prelude::*;

use crate::{
    components::{
        KeyTopic,
        GreenNote,
        my_accordion::{Accordion, AccordionItem, AccordionTrigger, AccordionContent}
    },
    utils::h3_heading
};

#[component]
pub fn Content() -> Element {
    rsx! {
        p { class: "mb-4",
            "With only open authentication and WEP available in the original 802.11 standard, a more secure authentication method was 
            needed."
            br {}
            strong {
                "Client authentication generally involves some sort of challenge, a response, and then a decision to grant access."
            }
            br {}
            "Behind the scenes, it can also involve an exchange of session or encryption keys, in addition to other parameters needed
            for client access."
            br {}
            strong {
                "Each authentication method might have unique requirements as a unique way to pass information between the client and the AP."
            }
        }

        {h3_heading("EAP")}
        p { class: "mb-4",
            "Rather than build additional authentication methods into the 802.11 standard, a more flexible and scalable authentication 
            framework, the Extensible Authentication Protocol (EAP), was chosen."
            br {}
            "As its name implies, EAP is extensible and does not consist of any one authentication method."
            br {}
            strong {
                "Instead, EAP defines a set of common functions that actual authentication methods can use to authenticate users."
            }
            br {}
            "As you read through this section, notice how many authentication methods have EAP in their names."
            br {}
            strong { "Each method is unique and different, but each one follows the EAP framework." }
        }

        {h3_heading("IEEE 802.1x")}
        p { class: "mb-4",
            strong {
                "EAP has another interesting quality: it can integrate with the IEEE 802.1x port-based access control standard."
            }
            br {}
            "When 802.1x is enabled, it limits access to a network media until a client authenticates."
            br {}
            strong {
                "This means that a wireless client might be able to associate with an AP but will not be able to pass data to any other part 
                of the network until it successfully authenticates."
            }
        }

        p { class: "mb-1",
            strong {
                "With open and WEP authentication, wireless clients are authenticated locally at the AP without further intervention."
            }
            br {}
            "The scenario changes with 802.1x; the client uses open authentication to associate with the AP, and then the actual client 
            authentication process occurs at a dedicated authentication server."
            br {}
            "Figure 28-6 shows the three-party 802.1x arrangement that consists of the following entities:"
        }
        div { class: "flex flex-col mb-4 sm:flex-row sm:items-center sm:gap-x-4",
            KeyTopic {}
            ol { class: "list-disc list-inside",
                li {
                    span { class: "font-bold mr-1", "Supplicant:" }
                    "The client device that is requesting access"
                }
                li {
                    span { class: "font-bold mr-1", "Authenticator:" }
                    "The network device that provides access to the network (usually a wireless LAN controller [WLC])"
                }
                li {
                    span { class: "font-bold mr-1", "Authentication server (AS):" }
                    "The device that takes user or client credentials and permits or denies network access based on a user database and 
                    policies (usually a RADIUS server)"
                }
            }
        }

        img {
            class: "mb-4 rounded-lg",
            alt: "Figure 28-6 802.1x Client Authentication Roles",
            loading: "lazy",
            src: asset!("/assets/static/v1p8c28s2sh3f28-6.png", AssetOptions::image().with_avif()),
        }

        p { class: "mb-4",
            "The wireless LAN controller becomes a middleman in the client authentication process, controlling user access with 802.1x 
            and communicating with the authentication server using the EAP framework."
        }

        p { class: "mb-4",
            "The following sections provide an overview of several common EAP-based authentication methods."
            br {}
            "The goal here is to become aware of the many methods without trying to memorize them all."
            br {}
            "In fact, even when you configure user authentication on a wireless LAN, you will not have to select a specific method."
            br {}
            strong {
                "Instead, you select 802.1x on the WLC so that it is ready to handle a variety of EAP methods."
            }
            br {}
            "It is then up to the client and the authentication server to use a compatible method."
            br {}
            "You will learn more about configuring security on a wireless LAN in Chapter 29, “Building a Wireless LAN.”"
        }

        Accordion { class: "mb-4",
            AccordionItem {
                AccordionTrigger { {h3_heading("LEAP")} }
                AccordionContent {
                    p { class: "mb-4",
                        "As an early attempt to address the weaknesses in WEP, Cisco developed a proprietary wireless authentication 
                        method called Lightweight EAP (LEAP)."
                        br {}
                        strong {
                            "To authenticate, the client must supply username and password credentials."
                        }
                        br {}
                        "Both the authentication server and the client exchange challenge messages that are then encrypted and returned."
                        br {}
                        "This provides mutual authentication; as long as the messages can be decrypted successfully, the client and the 
                        AS have essentially authenticated each other."
                    }

                    p { class: "mb-4",
                        "At the time, WEP-based hardware was still widely used."
                        br {}
                        "Therefore, LEAP attempted to overcome WEP weaknesses by using dynamic WEP keys that changed frequently."
                        br {}
                        "Nevertheless, the method used to encrypt the challenge messages was found to be vulnerable, so LEAP has since 
                        been deprecated."
                        br {}
                        strong {
                            "Even though wireless clients and controllers still offer LEAP, you should not use it."
                        }
                    }
                }
            }
            AccordionItem {
                AccordionTrigger { {h3_heading("EAP-FAST")} }
                AccordionContent {
                    p { class: "mb-1",
                        "Cisco developed a more secure method called EAP Flexible Authentication by Secure Tunneling (EAP-FAST)."
                        br {}
                        "Authentication credentials are protected by passing a protected access credential (PAC) between the AS and the 
                        supplicant."
                        br {}
                        "The PAC is a form of shared secret that is generated by the AS and used for mutual authentication."
                        br {}
                        "EAP-FAST is a sequence of three phases:"
                    }
                    ol { class: "list-disc list-inside mb-4",
                        li {
                            span { class: "font-bold mr-1", "Phase 0:" }
                            "The PAC is generated or provisioned and installed on the client."
                        }
                        li {
                            span { class: "font-bold mr-1", "Phase 1:" }
                            "After the supplicant and AS have authenticated each other, they negotiate a Transport Layer Security (TLS) 
                            tunnel."
                        }
                        li {
                            span { class: "font-bold mr-1", "Phase 2:" }
                            "The end user can then be authenticated through the TLS tunnel for additional security."
                        }
                    }

                    p { class: "mb-4",
                        "Notice that two separate authentication processes occur in EAP-FAST—one between the AS and the supplicant and 
                        another with the end user."
                        br {}
                        "These occur in a nested fashion, as an outer authentication (outside the TLS tunnel) and an inner authentication 
                        (inside the TLS tunnel)."
                    }

                    p { class: "mb-4",
                        "Like other EAP-based methods, a RADIUS server is required."
                        br {}
                        strong {
                            "However, the RADIUS server must also operate as an EAP-FAST server to be able to generate PACs, one per user."
                        }
                    }
                }
            }
            AccordionItem {
                AccordionTrigger { {h3_heading("PEAP")} }
                AccordionContent {
                    p { class: "mb-4",
                        "Like EAP-FAST, the Protected EAP (PEAP) method uses an inner and outer authentication; however, the AS presents 
                        a digital certificate to authenticate itself with the supplicant in the outer authentication."
                        br {}
                        "If the supplicant is satisfied with the identity of the AS, the two will build a TLS tunnel to be used for the 
                        inner client authentication and encryption key exchange."
                    }

                    p { class: "mb-4",
                        "The digital certificate of the AS consists of data in a standard format that identifies the owner and is 
                        “signed” or validated by a third party."
                        br {}
                        "The third party is known as a certificate authority (CA) and is known and trusted by both the AS and the 
                        supplicants."
                        br {}
                        "The supplicant must also possess the CA certificate just so that it can validate the one it receives from the AS."
                        br {}
                        "The certificate is also used to pass a public key, in plain view, which can be used to help decrypt messages 
                        from the AS."
                    }

                    p { class: "mb-1",
                        "Notice that only the AS has a certificate for PEAP."
                        br {}
                        "That means the supplicant can readily authenticate the AS."
                        br {}
                        "The client does not have or use a certificate of its own, so it must be authenticated within the TLS tunnel 
                        using one of the following two methods:"
                    }
                    ol { class: "list-disc list-inside mb-4",
                        li {
                            span { class: "font-bold mr-1", "MSCHAPv2:" }
                            "Microsoft Challenge Authentication Protocol version 2"
                        }
                        li {
                            span { class: "font-bold mr-1", "GTC:" }
                            "Generic Token Card; a hardware device that generates one-time passwords for the user or a manually generated 
                            password"
                        }
                    }
                }
            }
            AccordionItem {
                AccordionTrigger { {h3_heading("EAP-TLS")} }
                AccordionContent {
                    p { class: "mb-4",
                        "PEAP leverages a digital certificate on the AS as a robust method to authenticate the RADIUS server."
                        br {}
                        "It is easy to obtain and install a certificate on a single server, but the clients are left to identify 
                        themselves through other means."
                        br {}
                        "EAP Transport Layer Security (EAPTLS) goes one step further by requiring certificates on the AS and on every client 
                        device."
                    }

                    p { class: "mb-4",
                        "With EAP-TLS, the AS and the supplicant exchange certificates and can authenticate each other."
                        br {}
                        "A TLS tunnel is built afterward so that encryption key material can be securely exchanged."
                    }

                    p { class: "mb-4",
                        strong {
                            "EAP-TLS is considered to be the most secure wireless authentication method available; however, implementing 
                            it can sometimes be complex."
                        }
                        br {}
                        "Along with the AS, each wireless client must obtain and install a certificate."
                        br {}
                        "Manually installing certificates on hundreds or thousands of clients can be impractical."
                        br {}
                        "Instead, you would need to implement a Public Key Infrastructure (PKI) that could supply certificates securely 
                        and efficiently and revoke them when a client or user should no longer have access to the network."
                        br {}
                        "This usually involves setting up your own CA or building a trust relationship with a third-party CA that can 
                        supply certificates to your clients."
                    }

                    GreenNote {
                        p {
                            strong { "NOTE:" }
                            " EAP-TLS is practical only if the wireless clients can accept and use digital certificates. Many wireless 
                            devices, such as communicators, medical devices, and RFID tags, have an underlying operating system that 
                            cannot interface with a CA or use certificates."
                        }
                    }
                }
            }
        }

        {h3_heading("RECAP")}
        ol { class: "list-disc list-inside",
            li {
                "Client authentication generally involves some sort of challenge, a response, and then a decision to grant access."
            }
            li {
                "Each authentication method might have unique requirements as a unique way to pass information between the client and 
                the AP."
            }
            li {
                "The Extensible Authentication Protocol (EAP) is a more flexible and scalable authentication framework."
            }
            li {
                "EAP is extensible and does not consist of any one authentication method. Instead, EAP defines a set of common functions 
                that actual authentication methods can use to authenticate users."
            }
            li {
                "EAP has another interesting quality: it can integrate with the IEEE 802.1x port-based access control standard."
            }
            li { "EAP-based methods use username and password as the credential and an AS server." }
            li { "LEAP has vulnerabilities and has since been deprecated, so never use it." }
            li {
                "EAP-FAST, PEAP and EAP-TLS are more secure alternatives and use digital certificates 
                for authenticating both the supplicant and the AS."
            }
            li {
                "EAP-TLS is considered to be the most secure wireless authentication method available; however, implementing it can 
                sometimes be complex.."
            }
        }
    }
}