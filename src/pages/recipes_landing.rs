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
                class: "centred-paragraph",
                "Recipe Landing Page." }

                landing_grid(context)
                Route{to: "/recipes/my_first_recipe", crate::pages::recipes_entry::recipe_entry_one(context)}
            }
    })
}

fn landing_grid(context: Scope) -> Element {
    // TODO: Convert to Dinner, Snacks/Dessert, Breakfast/Lunch.
    context.render(rsx! {

        Link {to: "/recipes/my_first_recipe",
              class: "centred-inline",
              "My first recipe"
            }
    })
}