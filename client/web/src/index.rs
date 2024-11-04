use dioxus::prelude::*;

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
            div { "Index" }
        }
    }
}
