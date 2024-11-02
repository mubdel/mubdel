use dioxus::prelude::*;

#[component]
fn Home() -> Element {
    rsx! {
        div {
            class: "login",
            div { "Home" }
        }
    }
}
