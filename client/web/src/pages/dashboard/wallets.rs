use dioxus::prelude::*;

use crate::layout::dashboard::Dashboard;

#[component]
pub fn Wallets() -> Element {
    rsx! {
        Dashboard {
            page: rsx! {
                div {
                    class: "wallets",
                    div { "wallets" }
                }
            }
        }
    }
}
