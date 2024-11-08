use dioxus::prelude::*;
use serde::Serialize;

use ty::user::User;

use crate::libs::fetcher::{fetch, Data};
use crate::router::Route;

#[component]
pub fn Signup() -> Element {
    let mut user = use_signal(|| Vars::default());

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
                        input {
                            value: "{user.read().email}",
                            oninput: move |e| {
                                user.write().email = e.value();
                            }
                        }
                    }
                    div {
                        class: "signup-box__username",
                        label { "Username" }
                        input {
                            value: "{user.read().username}",
                            oninput: move |e| {
                                user.write().username = e.value();
                            }
                        }
                    }
                    div {
                        class: "signup-box__password",
                        label { "Password" }
                        input {
                            value: "{user.read().password}",
                            oninput: move |e| {
                                user.write().password = e.value();
                            }
                        }
                    }
                    button {
                        class: "signup-box__button btn-primary",
                        onclick: move |_| {
                            let vars = user.read().clone();
                            let _ = use_resource(move || fetch::<Vars, Data<User>>(SIGNUP_QUERY, vars.clone()));
                        },
                        "Signup"
                    }
                }
            }
        }
    }
}

const SIGNUP_QUERY: &str = r#"
mutation SignupQuery(
    $email: String!,
    $username: String!,
    $password: String!,
) {
    register(
        email: $email,
        username: $username,
        password: $password,
    ) {
        id
    }
}
"#;

#[derive(Default, Clone, Serialize)]
struct Vars {
    pub email: String,
    pub username: String,
    pub name: String,
    pub password: String,
}
