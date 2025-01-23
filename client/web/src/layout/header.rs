use dioxus::prelude::*;

use crate::router::Route;

#[component]
pub fn Header() -> Element {
    const LOGO: Asset = asset!("/assets/img/logo.svg");
    const SETTINGS: Asset = asset!("/assets/img/settings.svg");

    rsx! {
        div {
            class: "dheader",
            div {
                class: "dheader__logo",
                Link {
                    to: Route::Index {},
                    img { src: "{LOGO}" }
                }
            }

            div {
                class: "dheader__buttons",
                img { src: "{SETTINGS}" }
            }
        }
    }
}
