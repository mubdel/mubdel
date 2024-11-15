use dioxus::prelude::*;

#[component]
pub fn Menu() -> Element {
    rsx! {
        div {
            class: "menu",
            div { "Menu" }
        }
    }
}
