/*
    Appellation: commands <module>
    Contributors: FL03 <jo3mccain@icloud.com>
    Description:
        ... Summary ...
*/
use clap::Subcommand;
use scsys::AsyncResult;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Hash, PartialEq, Serialize, Subcommand)]
pub enum Commands {
    Account {
        #[clap(long, short, value_parser)]
        address: String,
    },
    Services {
        #[arg(long, short)]
        update: Option<isize>,
    },
    System {
        #[arg(action = clap::ArgAction::SetTrue, long, short)]
        up: bool,
    },
}

impl Commands {
    pub async fn handler(&self) -> AsyncResult<&Self> {
        tracing::debug!("Processing commands issued to the cli...");
        match self {
            Self::Account { address } => {
                println!("{:?}", &address);
            }
            Self::Services { update } => {
                println!("{:?}", &update);
            }
            Self::System { up } => {
                if up.clone() {
                    tracing::debug!("Spawning the api...");
                    let api = crate::api::new();
                    api.serve().await?;
                }
            },
        };
        Ok(self)
    }
}
