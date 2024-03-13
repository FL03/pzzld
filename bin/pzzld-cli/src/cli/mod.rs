/*
    Appellation: cli <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
pub use self::opts::Options;

pub(crate) mod opts;

pub mod args;

use clap::Parser;
use serde::{Deserialize, Serialize};

pub fn new() -> Cli {
    Cli::parse()
}

#[derive(Clone, Debug, Deserialize, Eq, Hash, Ord, Parser, PartialEq, PartialOrd, Serialize)]
#[clap(about, author, long_about = None, version)]
#[command(arg_required_else_help(true), allow_missing_positional(true))]
pub struct Cli {
    #[clap(subcommand)]
    pub command: Option<Options>,
    #[arg(action = clap::ArgAction::SetTrue, long, short)]
    pub verbose: bool,
}

impl std::fmt::Display for Cli {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
