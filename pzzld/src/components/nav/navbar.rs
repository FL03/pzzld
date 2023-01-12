/*
    Appellation: navbar <module>
    Contrib: FL03 <jo3mccain@icloud.com> (https://github.com/FL03)
    Description: ... Summary ...
*/
#![allow(non_snake_case)]
use crate::components::icons::{RightArrowIcon, StacksIcon};
use dioxus::prelude::*;

#[derive(Clone, Debug, Default, Eq, Hash, PartialEq, Props)]

pub struct State {
    pub banner: String,
}

pub fn NavBar(cx: Scope<State>) -> Element {
    cx.render(
        rsx!(
            div { class: "container mx-auto flex flex-wrap p-5 flex-col md:flex-row items-center",
                a { class: "flex title-font font-medium items-center text-white mb-4 md:mb-0", href: "/",
                    StacksIcon {}
                    span { class: "ml-3 text-xl", "{cx.props.banner}"}
                }
                nav { class: "md:ml-auto flex flex-wrap items-center text-base justify-center",
                    a { class: "mr-5 hover:text-white", "First Link"}
                    a { class: "mr-5 hover:text-white", "Second Link"}
                    a { class: "mr-5 hover:text-white", "Third Link"}
                    a { class: "mr-5 hover:text-white", "Fourth Link"}
                }
                button {
                    class: "inline-flex items-center bg-gray-800 border-0 py-1 px-3 focus:outline-none hover:bg-gray-700 rounded text-base mt-4 md:mt-0",
                    "Button"
                    RightArrowIcon {}
                }
            }
        )
    )
}
