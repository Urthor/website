#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]
#![allow(unused_imports)]
#![allow(non_snake_case)]

use dioxus::html::object;
use dioxus::prelude::*;
use dioxus_router::{Route, Router};
use dioxus_web;

pub fn render_homepage(context: Scope) -> Element {
    context.render(rsx! {
        p{"Rendered homepage"}

        // TODO: Figure out how to add button sending to Blog and Recipe.
    })
}
