#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]
#![allow(unused_imports)]
#![allow(non_snake_case)]

use dioxus::html::object;
use dioxus::prelude::*;
use dioxus_router::{Link, Route, Router};
use dioxus_web;
use log;

pub use website::App;

fn main() {
    wasm_logger::init(wasm_logger::Config::new(log::Level::Debug));
    log::debug!("Testing error!");
    dioxus_web::launch(App);
}
