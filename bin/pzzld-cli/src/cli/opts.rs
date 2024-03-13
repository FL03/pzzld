/*
    Appellation: opts <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use super::args::*;
use clap::Subcommand;
use serde::{Deserialize, Serialize};
use smart_default::SmartDefault;
use strum::{Display, EnumCount, EnumIs, EnumIter};

#[derive(
    Clone,
    Debug,
    Deserialize,
    Display,
    EnumCount,
    EnumIs,
    EnumIter,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
    Serialize,
    SmartDefault,
    Subcommand,
)]
#[serde(rename_all = "lowercase")]
pub enum Options {
    Platform(PlatformCommand),
    #[default]
    Sys(SystemCommand),
}
