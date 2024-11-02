use dioxus::prelude::*;

#[component]
pub fn Signup() -> Element {
    rsx! {
        div {
            class: "signup",
            div { "Sign up" }
        }
    }
}
