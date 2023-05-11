#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]
#![allow(unused_imports)]
#![allow(non_snake_case)]

use dioxus::prelude::*;
use dioxus_router::{Link, Route, Router};
use dioxus_web;

use crate::pages;

pub fn recipe_landing(context: Scope) -> Element {
    context.render(rsx! {
            body {
            p {
                class: "text-[#8B4513] text-2xl text-center",
                "Recipe Landing Page Component." }
                landing_grid(context)
            }
    })
}

fn landing_grid(context: Scope) -> Element {
    // TODO: Convert to Dinner, Snacks/Dessert, Breakfast/Lunch.
    context.render(rsx! {
        div {
            class: "flex justify-center mt-90",
            // Flex is a flex container.  Justify-center will center containers.
            Link {to: "/my_first_recipe",
                  class: "text-[#8B4515] text-2xl text-center",
                  "My first recipe.  Start of Recipe Grid Component."
            }
        }
    })
}