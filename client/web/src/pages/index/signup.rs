use dioxus::prelude::*;

use crate::router::Route;

#[component]
pub fn Signup() -> Element {
    rsx! {
        div {
            class: "signup",
            div {
                class: "signup__header",
                div {
                    class: "signup__header--logo",
                    Link {
                        to: Route::Index {},
                        div { "Mubdel" }
                    }
                }
                div {
                    class: "signup__header--login",
                    Link {
                        to: Route::Login {},
                        div { "Login" }
                    }
                }
            }
            div {
                class: "signup__box",
                div {
                    class: "signup-box",
                    h4 {
                        "Signup"
                    }
                    div {
                        class: "signup-box__email",
                        label { "Email" }
                        input {}
                    }
                    div {
                        class: "signup-box__username",
                        label { "Username" }
                        input {}
                    }
                    div {
                        class: "signup-box__password",
                        label { "Password" }
                        input {}
                    }
                    button {
                        class: "signup-box__button btn-primary",
                        "Signup"
                    }
                }
            }
        }
    }
}
