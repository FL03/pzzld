/*
    Appellation: cli <module>
    Contrib: @FL03
*/
#[doc(inline)]
pub use self::opts::*;
#[doc(hidden)]
pub(crate) use self::utils::*;

pub mod opts;

use anyhow::Context;
use std::path::PathBuf;

/// A CLI for executing WebAssembly components that
/// implement the `example` world.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, clap::Parser)]
#[clap(about, author, name="sampler", long_about = None, version)]
#[command(arg_required_else_help(true), allow_missing_positional(true))]
pub struct Cli {
    #[clap(subcommand)]
    pub cmd: Option<Opts>,
    /// release flag
    #[arg(action = clap::ArgAction::Count, long, short)]
    pub release: u8,
    /// toggle verbose output
    #[arg(action = clap::ArgAction::Count, long, short)]
    pub verbose: u8,
    /// The path to the component.
    #[clap(long, short)]
    pub workdir: Option<PathBuf>,
}

impl Cli {
    pub fn parse() -> Self {
        clap::Parser::parse()
    }
    pub async fn handle(self, ctx: &mut crate::PlatformInner) -> anyhow::Result<()> {
        ctx.set_workdir_option(self.workdir);

        if let Some(cmd) = self.cmd {
            match cmd {
                Opts::Add(args) => args.handle(ctx).await.context("add handler failed")?,
                _ => unimplemented!(),
            }
        }
        Ok(())
    }
}

mod utils {
    use anyhow::Context;
    use wasmtime::{
        Config, Engine, Store,
        component::{Component, Linker},
    };

    pub async fn add(path: std::path::PathBuf, x: i32, y: i32) -> wasmtime::Result<i32> {
        let mut config = Config::default();
        config.wasm_component_model(true);
        config.async_support(true);
        let engine = Engine::new(&config)?;
        let mut linker = Linker::new(&engine);

        // Add the command world (aka WASI CLI) to the linker
        wtw::add_to_linker_async(&mut linker).context("Failed to link command world")?;
        let wasi_view = crate::PlatformState::new();
        let mut store = Store::new(&engine, wasi_view);

        let component = Component::from_file(&engine, path).context("Component file not found")?;

        let instance = crate::Wasiclap::instantiate_async(&mut store, &component, &linker)
            .await
            .context("Failed to instantiate the example world")?;
        instance
            .call_add(&mut store, x, y)
            .await
            .context("Failed to call add function")
    }
}
