use dioxus::prelude::*;

use crate::layout::dashboard::Dashboard;

#[component]
pub fn Transactions() -> Element {
    rsx! {
        Dashboard {
            page: rsx! {
                div {
                    class: "transactions",
                    div { "transactions" }
                }
            }
        }
    }
}
