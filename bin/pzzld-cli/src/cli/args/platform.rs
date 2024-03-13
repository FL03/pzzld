/*
    Appellation: platform <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use clap::{Parser, Subcommand};
use serde::{Deserialize, Serialize};
use smart_default::SmartDefault;
use strum::{Display, EnumCount, EnumIs, EnumIter};

#[derive(
    Clone, Debug, Default, Deserialize, Eq, Hash, Ord, Parser, PartialEq, PartialOrd, Serialize,
)]
pub struct PlatformCommand {
    #[clap(subcommand)]
    pub args: Option<PlatformOpts>,
}

impl std::fmt::Display for PlatformCommand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}

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
#[strum(serialize_all = "lowercase")]
pub enum PlatformOpts {
    #[default]
    Connect {
        #[clap(long, short)]
        target: Option<String>,
    },
}

impl PlatformOpts {
    pub fn connect(target: Option<String>) -> Self {
        Self::Connect { target }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_platform_command() {
        let args = PlatformOpts::Connect {
            target: Some("10".to_string()),
        };
        assert_eq!(args.to_string(), "connect");
        println!("{}", args);
    }
}
