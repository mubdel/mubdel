#![allow(clippy::redundant_closure)]

use crate::router::web;

pub mod components;
pub mod layout;
pub mod libs;
pub mod pages;
pub mod router;

fn main() {
    dioxus::logger::initialize_default();

    dioxus::launch(web);
}
