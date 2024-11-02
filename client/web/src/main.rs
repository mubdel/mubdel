use dioxus::prelude::*;

use crate::index::web;

pub mod index;
pub mod login;
pub mod signup;

fn main() {
    launch(web);
}
