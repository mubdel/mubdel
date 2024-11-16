use dioxus::prelude::*;

use crate::layout::dashboard::Dashboard;

#[component]
pub fn Deposit() -> Element {
    rsx! {
        Dashboard {
            page: rsx! {
                div {
                    class: "deposit",
                    h4 { "New deposit" }

                    div {
                        class: "deposit__ways",
                        div {
                            class: "deposit-way",
                            div {
                                class: "deposit-way__title",
                                "Cryptocurrency"
                            }
                            div {
                                class: "deposit-way__desc",
                                "We accept BTC, ETH, SOL, USDT and USDC"
                            }
                        }
                        div {
                            class: "deposit-way",
                            div {
                                class: "deposit-way__title",
                                "Bank Transfer (Comming Soon)"
                            }
                            div {
                                class: "deposit-way__desc",
                                "Comming soon"
                            }
                        }
                        div {
                            class: "deposit-way",
                            div {
                                class: "deposit-way__title",
                                "Credit/Debit Card"
                            }
                            div {
                                class: "deposit-way__desc",
                                "We accept USD and EUR"
                            }
                        }
                    }

                    div {
                        class: "deposit__table",
                        h4 { "Recent deposits" }
                        div {
                            class: "deposit-table",
                            div {
                                class: "deposit-table__row",
                                div {
                                    class: "deposit-table__row--date",
                                    "Date"
                                }
                                div {
                                    class: "deposit-table__row--status",
                                    "Status"
                                }
                                div {
                                    class: "deposit-table__row--type",
                                    "Deposit Type"
                                }
                                div {
                                    class: "deposit-table__row--amount",
                                    "Amount"
                                }
                            }
                            div {
                                class: "deposit-table__row",
                                div {
                                    class: "deposit-table__row--date",
                                    "16-11-24"
                                }
                                div {
                                    class: "deposit-table__row--status",
                                    "Cancelled"
                                }
                                div {
                                    class: "deposit-table__row--type",
                                    "Cryptocurrency"
                                }
                                div {
                                    class: "deposit-table__row--amount",
                                    "0.0001 BTC"
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
