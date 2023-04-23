/*
    Appellation: opts <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... Summary ...
*/
use super::args::{Auto, Build, Serve, Setup, Test};
use clap::Subcommand;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize, Subcommand)]
pub enum Opts {
    Auto(Auto),
    Build(Build),
    Serve(Serve),
    Setup(Setup),
    Test(Test)
}

