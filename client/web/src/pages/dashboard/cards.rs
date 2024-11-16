use dioxus::prelude::*;

use crate::layout::dashboard::Dashboard;

#[component]
pub fn Cards() -> Element {
    rsx! {
        Dashboard {
            page: rsx! {
                div {
                    class: "cards",
                    div { "cards" }
                }
            }
        }
    }
}
