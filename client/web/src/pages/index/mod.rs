use dioxus::prelude::*;

use crate::libs::storage::is_authenticated;
use crate::pages::dashboard::home::Home;
use crate::router::Route;

pub mod about;
pub mod bug_bounty;
pub mod cards;
pub mod cookies_policy;
pub mod fees;
pub mod forget_password;
pub mod login;
pub mod mobile;
pub mod privacy_policy;
pub mod signup;

#[component]
pub fn Index() -> Element {
    if is_authenticated() {
        return rsx! {
            Home {}
        };
    }

    const X_LOGO: Asset = asset!("/assets/img/x.svg");
    const YOUTUBE_LOGO: Asset = asset!("/assets/img/youtube.svg");
    const LINKEDIN_LOGO: Asset = asset!("/assets/img/linkedin.svg");
    const INSTAGRAM_LOGO: Asset = asset!("/assets/img/instagram.svg");
    const TELEGRAM_LOGO: Asset = asset!("/assets/img/telegram.svg");

    rsx! {
        div {
            class: "index",
            div {
                class: "header",
                div {
                    class: "header__navbar",
                    div {
                        class: "header__navbar--logo",
                        "Mubdel"
                    }
                    div {
                        class: "header__navbar--links",
                        Link {
                            to: Route::Login {},
                            div { "Login" }
                        }
                        Link {
                            to: Route::Signup {},
                            div { "Signup" }
                        }
                    }
                }
                div {
                    class: "header__join",
                    h1 {
                        "Unlock the power of crypto"
                        br {}
                        "for everyday transactions"
                    }
                    div {
                        class: "header__join--links",
                        button {
                            class: "btn-primary",
                            Link {
                                to: Route::Signup {},
                                div { "Open Account" }
                            }
                        }
                        button {
                            class: "btn-secondary",
                            Link {
                                to: Route::Cards {},
                                div { "Cards" }
                            }
                        }
                    }
                }
            }
            div {
                class: "features",
                "Mubdel Platform Features"
            }
            div {
                class: "footer",
                div {
                    class: "footer__item",
                    div {
                        class: "footer__item--title",
                        "Products"
                    }
                    div {
                        class: "footer__item--object",
                        Link {
                            to: Route::Cards {},
                            div { "Cards" }
                        }
                    }
                    div {
                        class: "footer__item--object",
                        Link {
                            to: Route::Mobile {},
                            div { "Mobile Apps" }
                        }
                    }
                }
                div {
                    class: "footer__item",
                    div {
                        class: "footer__item--title",
                        "Community"
                    }
                    div {
                        class: "footer__item--object",
                        Link {
                            to: "https://x.com/mubdel",
                            div {
                                img { src: "{X_LOGO}" }
                                span { "X" }
                            }
                        }
                    }
                    div {
                        class: "footer__item--object",
                        Link {
                            to: "https://www.youtube.com/@mubdel_finance",
                            div {
                                img { src: "{YOUTUBE_LOGO}" }
                                span { "Youtube" }
                            }
                        }
                    }
                    div {
                        class: "footer__item--object",
                        Link {
                            to: "https://www.linkedin.com/company/mubdel",
                            div {
                                img { src: "{LINKEDIN_LOGO}" }
                                span { "LinkedIn" }
                            }
                        }
                    }
                    div {
                        class: "footer__item--object",
                        Link {
                            to: "https://t.me/mubdel",
                            div {
                                img { src: "{TELEGRAM_LOGO}" }
                                span { "Telegram" }
                            }
                        }
                    }
                    div {
                        class: "footer__item--object",
                        Link {
                            to: "https://www.instagram.com/mubdel_offical",
                            div {
                                img { src: "{INSTAGRAM_LOGO}" }
                                span { "Instagram" }
                            }
                        }
                    }
                }
                div {
                    class: "footer__item",
                    div {
                        class: "footer__item--title",
                        "Company"
                    }
                    div {
                        class: "footer__item--object",
                        Link {
                            to: Route::About {},
                            div { "About" }
                        }
                    }
                    div {
                        class: "footer__item--object",
                        Link {
                            to: Route::Fees {},
                            div { "Our Fees" }
                        }
                    }
                    div {
                        class: "footer__item--object",
                        Link {
                            to: "https://www.linkedin.com/company/mubdel/jobs",
                            div { "Careers" }
                        }
                    }
                }
                div {
                    class: "footer__item",
                    div {
                        class: "footer__item--title",
                        "Support"
                    }
                    div {
                        class: "footer__item--object",
                        Link {
                            to: "https://support.mubdel.com",
                            div { "Help Center" }
                        }
                    }
                    div {
                        class: "footer__item--object",
                        Link {
                            to: "https://cs.mubdel.com",
                            div { "Contact us" }
                        }
                    }
                    div {
                        class: "footer__item--object",
                        Link {
                            to: Route::BugBounty {},
                            div { "Bug Bounty" }
                        }
                    }
                }
                div {
                    class: "footer__item",
                    div {
                        class: "footer__item--title",
                        "Legal & privacy"
                    }
                    div {
                        class: "footer__item--object",
                        Link {
                            to: Route::PrivacyPolicy {},
                            div { "Privacy policy" }
                        }
                    }
                    div {
                        class: "footer__item--object",
                        Link {
                            to: Route::CookiesPolicy {},
                            div { "Cookies policy" }
                        }
                    }
                }
                div {
                    class: "footer__copyright",
                    "Copyright © 2025 Mubdel Inc. All rights reserved."
                }
            }
        }
    }
}
