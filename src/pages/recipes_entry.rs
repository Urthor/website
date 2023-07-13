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

pub fn recipe_entry_one(context: Scope) -> Element {
    context.render(rsx! {
        b { p { class: "text-[#8B4513] text-2xl text-center", "Recipe entry here." } }
    })
}