#![allow(clippy::redundant_closure)]

use dioxus::prelude::*;
use dioxus_logger::tracing::Level;

use crate::router::web;

pub mod components;
pub mod layout;
pub mod libs;
pub mod pages;
pub mod router;

fn main() {
    dioxus_logger::init(Level::INFO).expect("logger failed to init");

    launch(web);
}
