#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]
#![allow(unused_imports)]
#![allow(non_snake_case)]

use dioxus::html::object;
use dioxus::prelude::*;
use dioxus_router::{Route, Router};
use dioxus_web;

pub fn homepage_route(context: Scope) -> Element {
    context.render(rsx! {

    crate::pages::homepage::render_homepage(context)

        })
}
