use dioxus::prelude::*;

use crate::router::Route;

#[component]
pub fn Header() -> Element {
    const LOGO: &str = manganis::mg!(file("./assets/img/logo.svg"));
    const SETTINGS: &str = manganis::mg!(file("./assets/img/settings.svg"));

    rsx! {
        div {
            class: "dheader",
            div {
                class: "dheader__logo",
                Link {
                    to: Route::Index {},
                    img { src: LOGO }
                }
            }

            div {
                class: "dheader__buttons",
                img { src: SETTINGS }
            }
        }
    }
}
