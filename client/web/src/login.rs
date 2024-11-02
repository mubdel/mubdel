use dioxus::prelude::*;

use crate::index::Route;

#[component]
pub fn Login() -> Element {
    rsx! {
        div {
            class: "login",
            div {
                class: "login__header",
                div {
                    class: "login__header--logo",
                    Link {
                        to: Route::Index {},
                        div { "Mubdel" }
                    }
                }

                div {
                    class: "login__header--signup",
                    Link {
                        to: Route::Signup {},
                        div { "Sign up" }
                    }
                }
            }

            div {
                class: "login__box",
                h4 { "Login In" }

                div {
                    class: "login-box",
                    div {
                        class: "login-box__email",
                        label { "Email or Username" }
                        input {}
                    }

                    div {
                        class: "login-box__password",
                        label { "Password" }
                        input {}
                    }

                    button {
                        "Login"
                    }

                    div {
                        class: "login-box__forget",
                        "Forget password?"
                    }
                }
            }
        }
    }
}
