use dioxus::prelude::*;

use super::header::Header;
use super::menu::Menu;

#[derive(PartialEq, Clone, Props)]
pub struct DashboardProps {
    page: Element,
}

#[component]
pub fn Dashboard(props: DashboardProps) -> Element {
    rsx! {
        div {
            class: "dashboard",
            Header {}
            div {
                class: "dashboard__main",
                Menu {}
                div {
                    class: "dmain",
                    {props.page}
                }
            }
        }
    }
}
