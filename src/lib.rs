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
        NavBar(context)
        Route{to: "/", children: Homepage(context)}
        // Route{to: "/", children: features::homepage::homepage_route(context)}
        // Route{to: "/recipes/:my_first_recipe", children: crate::pages::recipes_entry::recipe_entry_one(context)}
        Route{to: "/recipes",
                div { class: "flex justify-center items-center",
                    p {
                    class: "text-red-500 text-4xl font-bold",
                    "Recipes Footer"
                }}
                features::recipes::recipe_route(context),
                Route {
                    to: "/:my_first_recipe",
                    crate::pages::recipes_entry::recipe_entry_one(context)}
        }
        Route{to: "", p {
            class: "text-red-500 text-4xl font-bold",
            "404"
        }}
        }
})
}

pub fn NavBar(context: Scope) -> Element {
    context.render(
        rsx! {
                nav {
                    class: "flex flex-col items-start bg-amber-500 text-white p-3 fixed top-0 left-0",
                    // Flex & flex-col arranges children in a column aligned to start of container.
                    // w gives width, padding is the amount of padding around the navbar.
                    Link { class: "text-white-600",
                        to: "/", "Homepage" }
                    br {}
                    Link { class: "navbar-item",
                        to: "/recipes",  "Recipes" }
                }
            
        }
    )
}

fn render_todo_component(context: Scope) -> Element {
    context.render(rsx! {
        p{"Component is TODO."}
    })
}

pub fn Homepage(context: Scope) -> Element {
    context.render(rsx! {
        head {}
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