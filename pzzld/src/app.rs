/*
    Appellation: application <module>
    Contrib: FL03 <jo3mccain@icloud.com> (https://github.com/FL03)
    Description: ... Summary ...
*/
use crate::components::nav::navbar::NavBar;
use dioxus::prelude::*;

#[derive(Clone, Debug, Default, Eq, Hash, PartialEq, Props)]
pub struct Application {
    name: String,
}

/// The base application object to be launched
pub fn app(cx: Scope) -> Element {
    let banner = "Puzzled".to_string();

    cx.render(rsx!(
        div { class: "bg-gradient-to-br from-zinc-900 via-zinc-800 to-zinc-900 flex flex-col items-center justify-center m-0 p-0 z-0 min-h-screen min-w-full max-w-screen",
            header { class: "body-font prose prose-invert",
                NavBar { banner: banner.to_string() }
            }
            section { class: "flex flex-col grow items-center justify-center min-h-full max-h-screen min-w-full max-w-screen z-0",
                div { class: "container mx-auto flex px-5 py-24 md:flex-row flex-col items-center",
                    div { class: "lg:flex-grow md:w-1/2 lg:pr-24 md:pr-16 flex flex-col md:items-start md:text-left mb-16 md:mb-0 items-center text-center",
                        h1 { class: "title-font sm:text-4xl text-3xl mb-4 font-medium text-white",
                            br { class: "hidden lg:inline-block" }
                            "Puzzled"
                        }
                        p {
                            class: "mb-8 leading-relaxed",

                            "Welcome to Puzzled, a personal Ethereum namespace where I try out new features proposed for the scsys ecosystem."

                        }
                        div { class: "flex justify-center",
                            button {
                                class: "inline-flex text-white bg-indigo-500 border-0 py-2 px-6 focus:outline-none hover:bg-indigo-600 rounded text-lg",
                                "Learn more"
                            }
                            button {
                                class: "ml-4 inline-flex text-gray-400 bg-gray-800 border-0 py-2 px-6 focus:outline-none hover:bg-gray-700 hover:text-white rounded text-lg",
                                "Build an app"
                            }
                        }
                    }
                    div { class: "lg:max-w-lg lg:w-full md:w-1/2 w-5/6",
                        img {
                            class: "object-cover object-center rounded",
                            src: "https://i.imgur.com/oK6BLtw.png",
                            referrerpolicy:"no-referrer",
                            alt: "hero",
                        }
                    }
                }
            }
        }
    ))
}
