use dioxus::prelude::*;
use futures_util::stream::StreamExt;
use serde::{Deserialize, Serialize};

use crate::components::modal::ErrorModal;
use crate::libs::fetcher::{fetch, Service};
use crate::router::Route;

#[component]
pub fn Signup() -> Element {
    let mut user = use_signal(|| Vars::default());

    let mut error = use_signal(|| None);

    let signup: Coroutine<()> = use_coroutine(|mut rx| async move {
        while let Some(_) = rx.next().await {
            let vars = user.read().clone();
            let r = fetch::<Vars, Register>(SIGNUP_QUERY, vars.clone(), Service::User).await;
            if r.is_err() {
                error.set(Some("Could not sign up".to_string()));
            }
        }
    });

    let handle_signup = move |_| {
        signup.send(());
    };

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
                            r#type: "password",
                            value: "{user.read().password}",
                            oninput: move |e| {
                                user.write().password = e.value();
                            }
                        }
                    }
                    button {
                        class: "signup-box__button btn-primary",
                        onclick: handle_signup,
                        "Signup"
                    }
                }
            }
            ErrorModal {
                err: error
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
    register(user: {
        email: $email,
        username: $username,
        password: $password,
    }) {
        id
    }
}
"#;

#[derive(Default, Clone, Serialize)]
struct Vars {
    pub email: String,
    pub username: String,
    pub password: String,
}

#[derive(Debug, Deserialize)]
pub struct Register {
    pub register: User,
}

#[derive(Debug, Deserialize)]
pub struct User {
    pub id: String,
}
