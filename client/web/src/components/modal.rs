#[cfg(not(target_arch = "wasm32"))]
use std::time::Duration;

#[cfg(target_arch = "wasm32")]
use async_std::task::sleep;
use dioxus::prelude::*;
use futures_util::stream::StreamExt;
#[cfg(target_arch = "wasm32")]
use instant::Duration;
#[cfg(not(target_arch = "wasm32"))]
use tokio::time::sleep;

#[component]
pub fn SuccessModal() -> Element {
    rsx! {}
}

#[component]
pub fn ErrorModal(err: Signal<Option<String>>) -> Element {
    if let Some(msg) = err.clone().read().clone() {
        const WARNING: &str = manganis::mg!(file("./assets/img/warning.svg"));
        const WRONG: &str = manganis::mg!(file("./assets/img/wrong.svg"));

        let timer: Coroutine<()> = use_coroutine(|mut rx| async move {
            while (rx.next().await).is_some() {
                sleep(Duration::from_secs(3)).await;
                err.set(None);
            }
        });
        timer.send(());

        rsx! {
            div {
                class: "modal modal-success",
                div {
                    class: "modal__elements",
                    img {
                        class: "modal__elements--warning",
                        src: WARNING
                    }

                    h4 { "{msg}" }

                    img {
                        class: "modal__elements--wrong",
                        onclick: move |_| {
                            err.set(None);
                        },
                        src: WRONG
                    }
                }
            }
        }
    } else {
        rsx! {}
    }
}
