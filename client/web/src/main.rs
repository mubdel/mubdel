use dioxus::prelude::*;

use crate::index::web;

pub mod forget_password;
pub mod home;
pub mod index;
pub mod login;
pub mod signup;

fn main() {
    launch(web);
}
