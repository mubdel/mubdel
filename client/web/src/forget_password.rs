use dioxus::prelude::*;

use crate::index::Route;

#[component]
pub fn ForgetPassword() -> Element {
    rsx! {
        div {
            class: "forget-password",
            div {
                class: "forget-password__header",
                div {
                    class: "forget-password__header--logo",
                    Link {
                        to: Route::Index {},
                        div { "Mubdel" }
                    }
                }
                div {
                    class: "forget-password__header--links",
                    Link {
                        to: Route::Login {},
                        div { "Login" }
                    }
                    Link {
                        to: Route::Signup {},
                        div { "Sign up" }
                    }
                }
            }
            div {
                class: "forget-password__box",
                div {
                    class: "forget-password-box",
                    h4 { "Request Password Reset" }
                    div {
                        class: "forget-password-box__email",
                        label { "Email" }
                        input {}
                    }
                    div {
                        class: "forget-password-box__actions",
                        Link {
                            to: Route::Login {},
                            div { "Cancel" }
                        }
                        button {
                            class: "btn-primary",
                            "Submit"
                        }
                    }
                }
            }
        }
    }
}
