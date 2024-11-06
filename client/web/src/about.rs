use dioxus::prelude::*;

#[component]
pub fn About() -> Element {
    rsx! {
        div {
            class: "about",
            "About"
        }
    }
}
