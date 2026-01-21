/*
    Appellation: cli <module>
    Created At: 2026.01.10:15:58:56
    Contrib: @FL03
*/
use super::cmd::Command;
use crate::Context;

/// The root command line interface structure
#[derive(
    Clone,
    Debug,
    Default,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
    clap::Parser,
    serde::Deserialize,
    serde::Serialize,
)]
#[clap(about, author, long_about = None, version)]
#[command(arg_required_else_help(true), allow_missing_positional(true))]
pub struct Cli {
    #[clap(subcommand)]
    pub command: Option<Command>,
    #[clap(long, short = 'C', default_value_t = String::from("Puzzled.toml"))]
    pub config: String,
    #[clap(action = clap::ArgAction::Count, long, short)]
    pub release: u8,
    #[arg(action = clap::ArgAction::Count, long, short)]
    pub update: u8,
    #[arg(action = clap::ArgAction::Count, long, short)]
    pub verbose: u8,
}

impl Cli {
    #[tracing::instrument(target = "cli")]
    /// parse a new instance of the command line arguments
    pub fn parse() -> Self {
        tracing::debug! { "Parsing command line arguments..." }
        <Self as clap::Parser>::parse()
    }
    /// returns a reference to the optional command
    pub fn command(&self) -> &Option<Command> {
        &self.command
    }
    /// returns the `release` flag
    pub const fn release(&self) -> u8 {
        self.release
    }
    /// returns the `verbose` flag
    pub const fn verbose(&self) -> u8 {
        self.verbose
    }
    /// returns a copy of the `update` flag
    pub const fn update(&self) -> u8 {
        self.update
    }
    /// returns true if the release flag has been toggled at least once
    pub const fn is_release(&self) -> bool {
        self.release > 0
    }
    /// returns true if the verbose flag has been toggled at least once
    pub const fn is_verbose(&self) -> bool {
        self.verbose > 0
    }
    /// returns true if the update flag has been toggled at least once
    pub const fn is_update(&self) -> bool {
        self.update > 0
    }
    /// handle the parsed arguments w.r.t. the given context
    #[tracing::instrument(skip_all, target = "cli")]
    pub async fn handle(&self, ctx: &mut Context) -> anyhow::Result<()> {
        // handle any commands
        if let Some(cmd) = &self.command {
            tracing::info! { "Executing command: {:?}", cmd.name() }
            cmd.handle(ctx).await?;
        }
        // handle the flags
        if self.is_release() {
            tracing::info!(
                "Release flag has been toggled {n} times",
                n = self.release()
            );
        }
        if self.is_verbose() {
            tracing::info! { "Verbose flag has been toggled {n} times", n = self.verbose() }
        }

        if self.is_update() {
            tracing::info! { "Update flag has been toggled {n} times", n = self.update() }
        }
        Ok(())
    }
}
