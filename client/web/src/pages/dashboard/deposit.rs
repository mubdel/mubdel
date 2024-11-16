use dioxus::prelude::*;

use crate::layout::dashboard::Dashboard;

#[component]
pub fn Deposit() -> Element {
    rsx! {
        Dashboard {
            page: rsx! {
                div {
                    class: "deposit",
                    div { "deposit" }
                }
            }
        }
    }
}
