/*
    Appellation: commands <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... Summary ...
*/
use clap::Subcommand;
use scsys::AsyncResult;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Hash, PartialEq, Eq, Serialize, Subcommand)]
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
    Wallet {
        #[clap(long, short, value_parser)]
        passphrase: Option<String>,

        #[arg(action = clap::ArgAction::SetTrue, long)]
        new: bool,
    },
}

impl Commands {
    pub async fn handle_accounts(&self) -> AsyncResult<&Self> {
        if let Self::Account { address } = self.clone() {
            println!("{:?}", &address);
        };
        Ok(self)
    }
    pub async fn handle_services(&self) -> AsyncResult<&Self> {
        if let Self::Services { update } = self.clone() {
            println!("{:?}", &update);
        };
        Ok(self)
    }
    pub async fn handle_system(&self) -> AsyncResult<&Self> {
        if let Self::System { up } = self.clone() {
            if up {
                tracing::info!("Spawning the api...");
                let api = crate::api::new();
                api.serve().await?;
            }
        };
        Ok(self)
    }
    pub async fn handle_wallet(&self) -> AsyncResult<&Self> {
        if let Self::Wallet { passphrase, new } = self.clone() {
            if passphrase.is_none() && new {
                let mut mnemonic = pzzld::wallets::mnemonics::Mnemonic::new(None, None);
                mnemonic.generate(None).await?;

                println!("{:?}", mnemonic);
            }
        };

        Ok(self)
    }
    pub async fn handler(&self) -> AsyncResult<&Self> {
        tracing::info!("Processing commands issued to the cli...");
        match self {
            Self::Account { .. } => self.handle_accounts().await?,
            Self::Services { .. } => self.handle_services().await?,
            Self::System { .. } => self.handle_system().await?,
            Self::Wallet { .. } => self.handle_wallet().await?,
        };
        Ok(self)
    }
}