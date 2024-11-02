use dioxus::prelude::*;

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
}

pub fn web() -> Element {
    rsx! {
        Router::<Route> {}
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
