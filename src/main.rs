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
mod pages;

// Main method does two things.
// #1 Dynamic rendering.  Handling side effects for unlogin state, login, bot state.
// Bot gets statically rendered HTML unhydrated.
// Unlogged in users are hydrated with unlogged in features.
// Logged in users get different hydration.

// #2 Main method calls relevant top level routes.  Route static, route 1, route 2.

fn main() {
    // hot_reload_init!();
    // TODO: understand index.html.
    wasm_logger::init(wasm_logger::Config::new(log::Level::Debug));
    dioxus_web::launch(App);
}

// The static lifetime specifier explicitly ensures the variables lifetime.
// ' is the name for the specifier.
// static indicates the variable is static.
// Static means the entire duration of the program.

fn App(context: Scope) -> Element {
    // Add 3 Feature Routes.
    // Homepage.
    // Blog.
    // Recipe.

    context.render(rsx! {
        style {include_str!("/Users/urthor/projects/hamishpoole_website/src/assets/hamishpoole_website.css")}

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

            div {
                nav {
                    class: "navbar",
                    a {
                        class: "navbar-brand",
                        href: "#",
                        "Hamish's Navbar"
                    }

                    button {
                        class: "navbar-toggler",
                        onclick: |_| {
                            log::info!("Clicked!");
                        },
                        "Click me!"
                    }
                    a {
                        class: "navbar-item",
                        href: "#",
                        "Homepage"
                    }
                    a {
                        class: "navbar-item",
                        href: "#",
                        "Recipes"
                    }
                    a {
                        class: "navbar-item",
                        href: "#",
                        "Blog"
                    }
                }
            }
        }
    })
}


fn render_todo_component(context: Scope) -> Element {
    context.render(rsx! {
        p{"Component is TODO"}
    })
}

fn render_feature_routes(context: Scope) -> Element {
    context.render(rsx! {
        Router {
        Route{to: "/", features::homepage::render_homepage(context)}
        Route{to: "/blog", render_todo_component(context)}
        Route{to: "/recipes", render_todo_component(context)}
        Route{to: "", context.render(rsx!{p{"404"}})}
         }
    })
}

fn render_nav_bar(context: Scope) -> Element {
    context.render(
        rsx! {
        nav{ class: "nav-bar",
            ul{style: "list-style-type: none; margin: 0; padding: 0; display: flex;",
                Link { to: "/" li{style:"padding-right: 10px;", "Home" } }
                li{style:"padding-right: 10px;",
                Link { to: "/blog" li{style:"padding-right: 10px;", "Blog" } }
                }
                li{style:"padding-right: 10px;",
                Link { to: "/recipes" li{style:"padding-right: 10px;", "Recipes" } }
                }
            }
        }
    }
    )
}