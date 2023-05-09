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

fn main() {
    // hot_reload_init!();
    wasm_logger::init(wasm_logger::Config::new(log::Level::Debug));
    log::error!("Testing error!");
    dioxus_web::launch(App);
}

fn App(context: Scope) -> Element {
    context.render(rsx! {
       link { rel: "stylesheet", href: "https://cdn.tailwindcss.com"}
       // script {  src: "https://cdn.tailwindcss.com" }
       render_all_routes(context)
    })
}

fn render_all_routes(context: Scope) -> Element {
    context.render(rsx! {
        Router {
        self::navbar(context)
        Route{to: "/", features::homepage::homepage_route(context)}
        Route{to: "/recipes",
                p { class: "centred-paragraph",
                    "123456"}
                features::recipes::recipe_route(context)}
        Route{to: "/recipes/my_first_recipe", crate::pages::recipes_entry::recipe_entry_one(context)}
        Route{to: "", context.render(rsx!{p{"404"}})}
        }
    })
}

fn navbar(context: Scope) -> Element {
    context.render(
        rsx! {
                   script {  src: "https://cdn.tailwindcss.com" }

                nav {
                    // class: "navbar",
                    class: "bg-amber-500 text-white",
                    Link { to: "/", class: "navbar-item", "Homepage" }
                    Link { to: "/recipes", class: "navbar-item", "Recipes" }
                    }
                }
    )
}

fn render_todo_component(context: Scope) -> Element {
    context.render(rsx! {
        p{"Component is TODO"}
    })
}
// button {
//     class: "navbar-toggler",
//     onclick: |_| {
//         log::info!("Clicked!");
//     },
//     "Click me!"
// }