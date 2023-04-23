/*
    Appellation: template-rs-dioxide <binary>
    Contrib: FL03 <jo3mccain@icloud.com> (https://github.com/FL03)
    Description: ... Summary ...
*/
pub use self::{primitives::*, utils::*};

mod primitives;
mod utils;

pub mod actors;
pub mod cmp;

use cmp::{hero::Hero, nav::navbar::NavBar};
use dioxus::prelude::*;

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Props)]
pub struct ApplicationScope {
    pub name: String,
    pub content: String,
}

impl ApplicationScope {
    pub fn new() -> Self {
        Self {
            name: env!("CARGO_PKG_NAME").to_string(),
            content: env!("CARGO_PKG_DESCRIPTION").to_string(),
        }
    }
    pub fn with_name(mut self, name: impl ToString) -> Self {
        self.name = name.to_string();
        self
    }
    pub fn with_content(mut self, content: impl ToString) -> Self {
        self.content = content.to_string();
        self
    }
}

impl Default for ApplicationScope {
    fn default() -> Self {
        Self::new()
    }
}

/// The base application object to be launched
pub fn app(cx: Scope<ApplicationScope>) -> Element {
    let img = "https://images.unsplash.com/photo-1617420207078-f9cae65824d5?ixlib=rb-4.0.3&ixid=MnwxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8&auto=format&fit=crop&w=1470&q=80".to_string();

    cx.render(rsx!(
        div { class: "bg-gradient-to-br from-zinc-900 via-zinc-800 to-zinc-900 flex flex-col items-center justify-center m-0 p-0 z-0 min-h-screen min-w-full max-w-screen",
            header { class: "flex min-w-full max-w-screen body-font prose prose-invert",
                NavBar { banner: cx.props.name.clone() }
            }
            main { class: "flex flex-col grow items-center justify-center min-h-full max-h-screen min-w-full max-w-screen z-0",
                div { class: "container mx-auto flex px-5 py-24 md:flex-row flex-col items-center",
                    Hero { banner: cx.props.name.clone(), src: img.clone() },
                    div { class: "flex flex-col md:ml-auto w-full md:w-1/2 p-8",
                        div { id: "sign-in-status" },
                        div { id: "sign-in" },
                        div { id: "account-details" },
                    }
                }
            }
        }
    ))
}
