use dioxus::prelude::*;

use crate::libs::storage::is_authenticated;
use crate::pages::dashboard::cards::Cards as MyCards;

#[component]
pub fn Cards() -> Element {
    if is_authenticated() {
        return rsx! {
            MyCards {}
        };
    }

    rsx! {
        h1 { "Cards" }
    }
}
