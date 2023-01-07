/*
    Appellation: commands <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... Summary ...
*/
use crate::command;
use super::{Builder, Runner, Setup};
use anyhow::Result;
use clap::Subcommand;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Hash, PartialEq, Serialize, Subcommand)]
pub enum Commands {
    Auto,
    Build(Builder),
    Run(Runner),
    Setup(Setup),
}

impl Commands {
    pub fn builder(&self) -> Result<&Self> {
        Builder::default().handler()?;
        Ok(self)
    }
    pub fn testing(&self) -> Result<&Self> {
        command(
            "cargo",
            vec!["test", "--all", "--all-features"],
        )?;
        Ok(self)
    }
    pub fn rustfmt(&self) -> Result<&Self> {
        command("cargo", vec!["fmt", "--all"])?;
        Ok(self)
    }
    pub fn clippy(&self) -> Result<&Self> {
        command("cargo", vec!["clippy", "--all", "--allow-dirty", "--fix"])?;
        Ok(self)
    }
    pub fn auto(&self) -> Result<&Self> {
        tracing::info!("Formatting the codespace...");
        self.rustfmt()?;
        tracing::info!("Analyzing the codespace...");
        self.clippy()?;
        tracing::info!("Initializing the build stages...");
        self.builder()?;
        tracing::info!("Testing the workspace...");
        self.testing()?;
        Ok(self)
    }
    pub fn handler(&self) -> Result<&Self> {
        tracing::info!("Processing commands issued to the cli...");
        match self {
            Self::Auto => {
                tracing::info!("Initializing the CI/CD pipeline");
                self.auto()?;
            }
            Self::Build(build) => {
                tracing::info!("Compiling the codebase...");
                build.handler()?;
            }
            Self::Run(runner) => {
                runner.handler()?;
            }
            Self::Setup(setup) => {
                tracing::info!("Setting up the environment...");
                setup.handler()?;
            }
        };
        Ok(self)
    }
}

impl Default for Commands {
    fn default() -> Self {
        Self::Setup(Default::default())
    }
}
