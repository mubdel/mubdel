use dioxus::prelude::*;

#[component]
pub fn PrivacyPolicy() -> Element {
    rsx! {
        div {
            class: "privacy-policy",
            "Privacy Policy"
        }
    }
}
