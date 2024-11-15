use dioxus::prelude::*;

pub mod wallets;

#[component]
pub fn Home() -> Element {
    rsx! {
        div {
            class: "home",
            div { "Home" }
        }
    }
}
