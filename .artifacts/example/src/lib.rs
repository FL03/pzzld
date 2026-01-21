/*
    Appellation: lib <library>
    Contrib: @FL03
*/
extern crate wasmtime as wt;
extern crate wasmtime_wasi as wtw;

#[doc(inline)]
pub use self::{cli::Cli, platform::*};

pub mod actors;
pub mod cli;
pub mod platform;

pub mod prelude {}

wt::component::bindgen!({
    path: "assets/cmp/add/add.wit",
    world: "wasiclap",
    async: true
});

pub(crate) fn default_workdir() -> std::path::PathBuf {
    std::env::current_dir()
        .expect("Failed to get current directory")
        .join("assets")
}

pub(crate) fn wasmtime_config() -> wt::Config {
    let mut config = wt::Config::default();
    config.async_support(true);
    config.wasm_component_model(true);
    config
}

pub mod utils {}
