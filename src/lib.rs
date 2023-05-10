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

pub mod features;
pub mod pages;
pub mod utils;

pub fn App(context: Scope) -> Element {
    context.render(rsx! {
        div {
            render_all_routes(context)
            }
    })
}

fn render_all_routes(context: Scope) -> Element {
    context.render(rsx! {
        Router {
        self::navbar(context)
        Route{to: "/", children: features::homepage::homepage_route(context)}
        Route{to: "/recipes", children: features::recipes::recipe_route(context)}
        Route{to: "/recipes/my_first_recipe", children: crate::pages::recipes_entry::recipe_entry_one(context)}
        Route{to: "", context.render(rsx!{p{"404"}})}
        }
    })
}

fn navbar(context: Scope) -> Element {
    context.render(
        rsx! {
            div {
                nav {
                    class: "flex flex-col items-start bg-amber-500 text-white p-3 fixed top-0 left-0",
                    // Flex & flex-col arranges children in a column aligned to start of container.
                    // w gives width, padding is the amount of padding around the navbar.
                    Link { to: "/", "Homepage" }
                    Link { class: "navbar-item", to: "/recipes",  "Recipes" }
                }
            }
        }
    )
}

fn render_todo_component(context: Scope) -> Element {
    context.render(rsx! {
        p{"Component is TODO."}
    })
}