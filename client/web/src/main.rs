use dioxus::prelude::*;

use crate::router::web;

pub mod components;
pub mod layout;
pub mod libs;
pub mod pages;
pub mod router;

fn main() {
    launch(web);
}
