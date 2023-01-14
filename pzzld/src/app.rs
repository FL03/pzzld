/*
    Appellation: application <module>
    Contrib: FL03 <jo3mccain@icloud.com> (https://github.com/FL03)
    Description: ... Summary ...
*/
use crate::components::{headers::hero::Hero, nav::navbar::NavBar};
use dioxus::prelude::*;

#[derive(Clone, Debug, Default, Eq, Hash, PartialEq, Props)]
pub struct ApplicationScope {
    pub name: String,
    pub content: String,
}

/// The base application object to be launched
pub fn app(cx: Scope) -> Element {
    let banner = "Puzzled".to_string();

    cx.render(rsx!(
        div { class: "bg-gradient-to-br from-zinc-900 via-zinc-800 to-zinc-900 flex flex-col items-center justify-center m-0 p-0 z-0 min-h-screen min-w-full max-w-screen",
            header { class: "flex min-w-full max-w-screen body-font prose prose-invert",
                NavBar { banner: banner.clone() }
            }
            main { class: "flex flex-col grow items-center justify-center min-h-full max-h-screen min-w-full max-w-screen z-0",
                div { class: "container mx-auto flex px-5 py-24 md:flex-row flex-col items-center",
                    Hero { banner: banner },
                }
            }
        }
    ))
}
