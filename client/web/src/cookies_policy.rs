use dioxus::prelude::*;

#[component]
pub fn CookiesPolicy() -> Element {
    rsx! {
        div {
            class: "cookies-policy",
            "Cookies Policy"
        }
    }
}
