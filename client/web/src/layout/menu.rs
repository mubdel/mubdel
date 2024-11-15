use dioxus::prelude::*;

use crate::router::Route;

#[component]
pub fn Menu() -> Element {
    const HOME: &str = manganis::mg!(file("./assets/img/home.svg"));
    const CARD: &str = manganis::mg!(file("./assets/img/card.svg"));
    const TRANSACTION: &str = manganis::mg!(file("./assets/img/transaction.svg"));
    const WALLET: &str = manganis::mg!(file("./assets/img/wallet.svg"));
    const DEPOSIT: &str = manganis::mg!(file("./assets/img/deposit.svg"));
    const WITHDRAWAL: &str = manganis::mg!(file("./assets/img/withdrawal.svg"));

    rsx! {
        div {
            class: "menu",
            div {
                class: "menu__link",
                Link {
                    to: Route::Index {},
                    div {
                        class: "menu__link--image",
                        img { src: HOME }
                    }
                    div {
                        class: "menu__link--text",
                        "Home"
                    }
                }
            }
            div {
                class: "menu__link",
                Link {
                    to: Route::Wallets {},
                    div {
                        class: "menu__link--image",
                        img { src: CARD }
                    }
                    div {
                        class: "menu__link--text",
                        "Cards"
                    }
                }
            }
            div {
                class: "menu__link",
                Link {
                    to: Route::Wallets {},
                    div {
                        class: "menu__link--image",
                        img { src: TRANSACTION }
                    }
                    div {
                        class: "menu__link--text",
                        "Transaction"
                    }
                }
            }
            div {
                class: "menu__link",
                Link {
                    to: Route::Wallets {},
                    div {
                        class: "menu__link--image",
                        img { src: WALLET }
                    }
                    div {
                        class: "menu__link--text",
                        "Wallets"
                    }
                }
            }
            div {
                class: "menu__link",
                Link {
                    to: Route::Wallets {},
                    div {
                        class: "menu__link--image",
                        img { src: DEPOSIT }
                    }
                    div {
                        class: "menu__link--text",
                        "Deposit"
                    }
                }
            }
            div {
                class: "menu__link",
                Link {
                    to: Route::Wallets {},
                    div {
                        class: "menu__link--image",
                        img { src: WITHDRAWAL }
                    }
                    div {
                        class: "menu__link--text",
                        "Withdrawal"
                    }
                }
            }
        }
    }
}
