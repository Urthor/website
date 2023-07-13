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
        style {}
        head {
            // meta { http-equiv: "refresh", content: "5" }
        }
        div { Routes(context) }
    })
}

fn Routes(context: Scope) -> Element {
    context.render(rsx! {
        Router { 
            NavBar(context),
            Route { to: "/", children: Homepage(context) }
            Route { to: "/recipes",
                div { class: "flex justify-center items-center",
                    p { class: "text-red-500 text-4xl font-bold", "Recipes Footer" }
                }
                features::recipes::recipe_route(context),
                Route { to: "/:my_first_recipe", crate::pages::recipes_entry::recipe_entry_one(context) }
            }
            Route { to: "", p { class: "text-red-500 text-4xl font-bold", "404" } }
        }
    })
}

pub fn NavBar(context: Scope) -> Element {
    context.render(
        rsx! {
            nav { class: "border-4 border-stone-600 border-double flex flex-col font-serif items-start text-stone-500 p-3 fixed top-0 left-0",
                ul {
                    li { class: "border-b-2 border-b-stone-500 border-solid font-serif m-5 p-12 text-black",
                        Link {
                            to: "/",
                            "Homepage"
                        }
                    }
                    li { class: "border-b-2 border-b-stone-500 border-solid font-serif m-5 p-12 text-black",
                        Link { to: "/recipes", "Recipes" }
                    }
                }
            }
        }
    )
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
                    alt: "Chocolate fish image"
                }
            }
            div { class: "flex flex-wrap justify-center",
                ul {
                    li { class: "centred_list_item", "First Item." }
                    li { class: "centred_list_item", "Second Item." }
                    li { class: "centred_list_item", "Third Item." }
                    li { class: "centred_list_item", "Fourth Item." }
                    li { class: "centred_list_item", "Fifth Item." }
                    li { class: "centred_list_item", "Sixth Item." }
                }
            }
        }
    })
}