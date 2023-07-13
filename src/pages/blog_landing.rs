#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]
#![allow(unused_imports)]
#![allow(non_snake_case)]

use dioxus::prelude::*;
use dioxus_web;

pub fn blog_landing(context: Scope) -> Element {
    context.render(rsx! { p { "Blog Landing Page" } })
}