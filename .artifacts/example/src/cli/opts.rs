/*
    Appellation: interface <module>
    Contrib: @FL03
*/
#[doc(inline)]
pub use self::{add::*, build::*};

mod add;
mod build;

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, clap::Subcommand)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Deserialize, serde::Serialize),
    serde(rename_all = "kebab-case")
)]
pub enum Opts {
    Add(AddCmd),
    Build(BuildCmd),
}
