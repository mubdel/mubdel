use dioxus::prelude::*;

use crate::index::web;

pub mod about;
pub mod bug_bounty;
pub mod cards;
pub mod cookies_policy;
pub mod fees;
pub mod forget_password;
pub mod home;
pub mod index;
pub mod login;
pub mod mobile;
pub mod privacy_policy;
pub mod signup;

fn main() {
    launch(web);
}
