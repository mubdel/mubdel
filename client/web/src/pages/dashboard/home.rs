use dioxus::prelude::*;

use crate::layout::dashboard::Dashboard;

#[component]
pub fn Home() -> Element {
    rsx! {
        Dashboard {
            page: rsx! {
                div {
                    class: "home",
                    div { "Home" }
                }
            }
        }
    }
}
