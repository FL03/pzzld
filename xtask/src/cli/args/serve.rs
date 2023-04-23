/*
    Appellation: serve <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... Summary ...
*/
use crate::project_root;
use clap::{Args, ValueEnum};
use serde::{Deserialize, Serialize};
use std::{net::SocketAddr, path::PathBuf};
use strum::{Display, EnumString, EnumVariantNames};

#[derive(
    Clone, Debug, Default, Deserialize, Display, EnumString, EnumVariantNames, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize, ValueEnum
)]
#[strum(serialize_all = "lowercase")]
pub enum Protocol {
    #[default]
    TCP
}

#[derive(Args, Clone, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize,)]
pub struct Serve {
    #[clap(default_value_t = SocketAddr::from(([0, 0, 0, 0], 8080)), long, short)]
    pub addr: SocketAddr,
    #[clap(default_value = "dist", long, short)]
    pub artifacts: PathBuf,
    #[clap(default_value_t = Protocol::TCP, long, short)]
    pub protocol: Protocol,
}

impl Default for Serve {
    fn default() -> Self {
        Self {
            addr: SocketAddr::from(([0, 0, 0, 0], 8080)),
            artifacts: project_root().join("dist"),
            protocol: Protocol::TCP,
        }
    }
}