use dioxus::prelude::*;

#[component]
pub fn ForgetPassword() -> Element {
    rsx! {
        div {
            class: "forget-password",
            div { "Forget Password" }
        }
    }
}
