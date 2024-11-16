use dioxus::prelude::*;

use crate::pages::dashboard::deposit::Deposit;
use crate::pages::dashboard::transactions::Transactions;
use crate::pages::dashboard::wallets::Wallets;
use crate::pages::dashboard::withdrawal::Withdrawal;
use crate::pages::index::about::About;
use crate::pages::index::bug_bounty::BugBounty;
use crate::pages::index::cards::Cards;
use crate::pages::index::cookies_policy::CookiesPolicy;
use crate::pages::index::fees::Fees;
use crate::pages::index::forget_password::ForgetPassword;
use crate::pages::index::login::Login;
use crate::pages::index::mobile::Mobile;
use crate::pages::index::privacy_policy::PrivacyPolicy;
use crate::pages::index::signup::Signup;
use crate::pages::index::Index;

#[derive(Clone, Routable, Debug, PartialEq)]
#[rustfmt::skip]
pub enum Route {
    // Index
    #[route("/")]
    Index {},
    #[route("/login")]
    Login {},
    #[route("/signup")]
    Signup {},
    #[route("/forget-password")]
    ForgetPassword {},
    #[route("/cards")]
    Cards {},
    #[route("/bug-bounty")]
    BugBounty {},
    #[route("/mobile")]
    Mobile {},
    #[route("/about")]
    About {},
    #[route("/fees")]
    Fees {},
    #[route("/privacy-policy")]
    PrivacyPolicy {},
    #[route("/cookies-policy")]
    CookiesPolicy {},

    // Dashboard
    #[route("/transactions")]
    Transactions {},
    #[route("/wallets")]
    Wallets {},
    #[route("/deposit")]
    Deposit {},
    #[route("/withdrawal")]
    Withdrawal {},
}

pub fn web() -> Element {
    rsx! {
        style { {include_str!("../assets/main.css")} }

        div {
            class: "root rst-a",
            Router::<Route> {}
        }
    }
}
