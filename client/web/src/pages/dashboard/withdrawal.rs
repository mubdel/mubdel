use dioxus::prelude::*;

use crate::layout::dashboard::Dashboard;

#[component]
pub fn Withdrawal() -> Element {
    rsx! {
        Dashboard {
            page: rsx! {
                div {
                    class: "withdrawal",
                    div { "withdrawal" }
                }
            }
        }
    }
}
