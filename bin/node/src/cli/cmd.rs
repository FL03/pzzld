/*
    Appellation: ops <module>
    Created At: 2026.01.10:15:56:39
    Contrib: @FL03
*/
//! the subcommands for the CLI application
#[doc(inline)]
pub use self::connect::*;

mod connect;

use crate::Context;

#[derive(
    Clone,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
    clap::Subcommand,
    serde::Deserialize,
    serde::Serialize,
    strum::EnumIs,
    strum::VariantNames,
)]
#[repr(C)]
pub enum Command {
    Connect(ConnectCmd),
}

impl Command {
    /// initialize a new [`Connect`](Command::Connect) command variant using the given value
    pub const fn connect(cmd: ConnectCmd) -> Self {
        Self::Connect(cmd)
    }
    /// returns the name of the command variant
    pub fn name(&self) -> &str {
        match self {
            Self::Connect(_) => "connect",
        }
    }
    /// handle the command using the given context
    #[tracing::instrument(skip_all)]
    pub async fn handle(&self, ctx: &mut Context) -> anyhow::Result<()> {
        match self {
            Self::Connect(cmd) => cmd.handle(ctx).await,
        }
    }
}
