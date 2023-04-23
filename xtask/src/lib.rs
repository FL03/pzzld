/*
    Appellation: xtask <library>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... Summary ...
*/
pub use self::{commands::*, context::*, primitives::*, utils::*};

mod commands;
mod context;
mod utils;

pub mod cli;
pub mod pipes;
pub mod server;

use cli::{CommandLineInterface, Opts};
use anyhow::Result;
use serde::{Deserialize, Serialize};

///
#[macro_export]
macro_rules! cmd {
    ($(
        $x:expr;
        [ $( $y:expr ),* ]
    );*) => {
        {
            $(
                let mut cmd = std::process::Command::new($x);
                cmd.current_dir(project_root());
                let mut tmp = Vec::new();
                $(
                    tmp.push($y);
                )*
                cmd.args(tmp.as_slice()).status().expect("");
            )*
        }
    };
}


#[derive(Clone, Debug, Default, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct Xtask {
    ctx: Context
}

impl Xtask {
    pub fn new(ctx: Context) -> Self {
        Self { ctx }
    }
    pub fn handle_cli(&self, cli: CommandLineInterface) -> Result<()> {
        let release = cli.release();
        let workspace = cli.workspace();
        if let Some(opts) = cli.cmd().clone() {
            match opts {
                Opts::Auto(_opt) => {
                    tracing::info!("Initializing the automatic pipeline");
                    command("cargo", vec!["fmt", "--all"])?;
                    command("cargo", vec!["clippy", "--all", "--allow-dirty", "--fix"])?;
                    command("cargo", vec!["build", "--workspace"])?;
                    command("cargo", vec!["test", "--all", "--allow-dirty"])?;
                },
                Opts::Build(_opt) => {
                    tracing::info!("Building the target...");
                    let mut args = vec!["build"];

                    if release {
                        args.push("--release");
                    }
                    if workspace {
                        args.push("--workspace");
                    }
                    command("cargo", args)?;
                },
                Opts::Serve(_opt) => {
                    tracing::info!("Serving the target...");
                },
                Opts::Setup(_opt) => {
                    tracing::info!("Setting up the workspace");
                    setup(true, false)?;
                },
                Opts::Test(_opt) => {
                    tracing::info!("Testing the target(s)");
                }
            }
        }
        Ok(())
    }
    pub fn init(self) -> Self {
        tracing_subscriber::fmt::init();
        self
    }
    pub async fn run(&self) -> Result<()> {
        let cli = CommandLineInterface::new();
        self.handle_cli(cli)?;
        
        Ok(())
    }
}



mod primitives {
    ///
    pub type Bundle<T = String> = std::collections::HashMap<T, Vec<Vec<T>>>;
}
