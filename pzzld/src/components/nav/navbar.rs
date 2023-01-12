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
            div { class: "flex flex-nowrap items-center justify-center bg-transparent prose prose-invert min-w-full max-w-screen my-0 p-3 sticky top-0 z-50",
                a { class: "flex title-font font-medium items-center text-white mb-4 md:mb-0", href: "/",
                    StacksIcon {}
                    span { class: "ml-3 text-xl", "{cx.props.banner}"}
                }
                nav { class: "md:ml-auto flex flex-wrap items-center text-base justify-center",
                    a { class: "mr-5 hover:text-white", "Apps"}
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
