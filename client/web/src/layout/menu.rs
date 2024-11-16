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

    let route: Route = use_route();

    let selected = use_signal(|| match route {
        Route::Cards {} => "cards",
        Route::Transactions {} => "transaction",
        Route::Wallets {} => "wallets",
        Route::Deposit {} => "deposit",
        Route::Withdrawal {} => "withdrawal",
        _ => "home",
    });

    rsx! {
        div {
            class: "menu",
            div {
                class: format!("menu__link {}", if *selected.read() == "home" { "menu-selected" } else { "" }),
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
                class: format!("menu__link {}", if *selected.read() == "cards" { "menu-selected" } else { "" }),
                Link {
                    to: Route::Cards {},
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
                class: format!("menu__link {}", if *selected.read() == "transaction" { "menu-selected" } else { "" }),
                Link {
                    to: Route::Transactions {},
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
                class: format!("menu__link {}", if *selected.read() == "wallets" { "menu-selected" } else { "" }),
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
                class: format!("menu__link {}", if *selected.read() == "deposit" { "menu-selected" } else { "" }),
                Link {
                    to: Route::Deposit {},
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
                class: format!("menu__link {}", if *selected.read() == "withdrawal" { "menu-selected" } else { "" }),
                Link {
                    to: Route::Withdrawal {},
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
