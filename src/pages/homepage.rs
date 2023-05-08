#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]
#![allow(unused_imports)]
#![allow(non_snake_case)]

use dioxus::prelude::*;
use dioxus_router::{Route, Router};
use dioxus_web;

pub fn render_homepage(context: Scope) -> Element {
    context.render(rsx! {
        head {
            // link { rel: "stylesheet", href: crate::utils::constants::css_path}
            // TODO: Add Tailwind.
        }
        body {
            div { class: "image-container",
             img {
                    src: "https://upload.wikimedia.org/wikipedia/commons/thumb/f/fb/Twochocolatefish.JPG/500px-Twochocolatefish.JPG",
                    alt: "Chocolate fish image",
                }
            }
            div { class: "flex-container1",
                ul {
                    li {class: "flex-item", "First Item."}
                    li {class: "flex-item", "Second Item."}
                    li {class: "flex-item", "Third Item."}
                    li {class: "flex-item", "Fourth Item."}
                    li {class: "flex-item", "Fifth Item."}
                    li {class: "flex-item", "Sixth Item."}
                }
            }
        }
    })
}
