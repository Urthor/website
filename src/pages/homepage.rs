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
        }
        body {
            div { class: "flex justify-center items-center pt-5",
                // Flex container.  Equivalent to display: flex in CSS.
                // Justify-center && items-center, Tailwind equivalent to justify-content: center, align-items: center.
                // pt-5, adds padding of 1.25rem, aka 20px.
                // max-w-full applies max-width: 100%.
                // h-auto, height: auto; maintains aspect ratio.
            img {
                   class: "max-w-full h-auto",
                   src: "https://upload.wikimedia.org/wikipedia/commons/thumb/f/fb/Twochocolatefish.JPG/500px-Twochocolatefish.JPG",
                   alt: "Chocolate fish image",
                }
            }
            div { class: "flex flex-wrap justify-center",
                ul {
                    li {class: "centred_list_item", "First Item."}
                    li {class: "centred_list_item", "Second Item."}
                    li {class: "centred_list_item", "Third Item."}
                    li {class: "centred_list_item", "Fourth Item."}
                    li {class: "centred_list_item", "Fifth Item."}
                    li {class: "centred_list_item", "Sixth Item."}
                }
            }
        }
    })
}