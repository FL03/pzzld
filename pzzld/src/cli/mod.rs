/*
    Appellation: cli <module>
    Contributors: FL03 <jo3mccain@icloud.com>
    Description:
        ... Summary ...
*/
pub use self::{args::*, commands::*, interface::*};

pub(crate) mod args;
pub(crate) mod commands;

pub fn new() -> CommandLineInterface {
    CommandLineInterface::default()
}

pub(crate) mod interface {
    use super::Commands;
    use clap::Parser;
    use scsys::AsyncResult;
    use serde::{Deserialize, Serialize};

    pub trait Commander: Clone + clap::Subcommand {
        fn handler(&self) -> AsyncResult<&Self>;
    }

    #[async_trait::async_trait]
    pub trait AsyncCommander: Clone + Send + Sync + clap::Subcommand {
        async fn handler(&self) -> AsyncResult<&Self>;
    }

    pub trait CLISpec: Parser {
        type Cmds: Commander;

        fn command(&self) -> Option<Self::Cmds>
        where
            Self: Sized;
        fn handler(&self) -> AsyncResult<&Self>
        where
            Self: Sized,
        {
            if let Some(cmd) = self.command() {
                cmd.handler()?;
            }
            Ok(self)
        }
    }

    #[async_trait::async_trait]
    pub trait AsyncCLISpec: Parser {
        type Cmds: AsyncCommander;

        fn command(&self) -> Option<Self::Cmds>
        where
            Self: Sized;
        async fn handler(&self) -> AsyncResult<&Self>
        where
            Self: Sized,
        {
            if let Some(cmd) = self.command().clone() {
                cmd.handler().await?;
            }
            Ok(self)
        }
    }

    #[derive(Clone, Debug, Deserialize, Eq, Hash, Parser, PartialEq, Serialize)]
    #[clap(about, author, version)]
    #[clap(long_about = "")]
    pub struct CommandLineInterface {
        #[clap(subcommand)]
        pub command: Option<Commands>,
        #[arg(action = clap::ArgAction::SetTrue, long, short)]
        pub debug: bool,
        #[clap(long, short, value_parser)]
        pub mode: Option<String>,
        #[arg(action = clap::ArgAction::SetTrue, long, short)]
        pub update: bool,
    }

    impl CommandLineInterface {
        pub async fn handler(&self) -> scsys::AsyncResult<&Self> {
            match self.command.clone() {
                None => {}
                Some(v) => {
                    v.handler().await?;
                }
            }
            Ok(self)
        }
    }

    impl Default for CommandLineInterface {
        fn default() -> Self {
            Self::parse()
        }
    }
}
