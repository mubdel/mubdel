use dioxus::prelude::*;

use crate::cards::Cards;
use crate::forget_password::ForgetPassword;
use crate::login::Login;
use crate::signup::Signup;

#[derive(Clone, Routable, Debug, PartialEq)]
pub enum Route {
    #[route("/")]
    Index {},
    #[route("/login")]
    Login {},
    #[route("/signup")]
    Signup {},
    #[route("/forget_password")]
    ForgetPassword {},
    #[route("/cards")]
    Cards {},
}

pub fn web() -> Element {
    rsx! {
        style { {include_str!("../assets/main.css")} }

        div {
            class: "root rst-a",
            Router::<Route> {}
        }
    }
}

#[component]
pub fn Index() -> Element {
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
        }
    }
}
