/*
   Appellation: Pzzld <binary>
   Contrib: FL03 <jo3mccain@icloud.com>
   Description: ... Summary ...
*/
pub use self::{context::*, settings::*, states::*};

pub mod api;
pub mod cli;
pub mod server;
pub(crate) mod context;
pub(crate) mod settings;
pub(crate) mod states;

use scsys::AsyncResult;
use serde::{Deserialize, Serialize};
use std::{fmt::Display, sync::Arc};

#[tokio::main]
async fn main() -> AsyncResult {
//    dotenv::dotenv()?;
    Application::<String>::default().run().await?;

    Ok(())
}

#[derive(Clone, Debug, Default, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct Application<T: Clone + Default + Display = String> {
    pub cnf: Settings,
    pub ctx: Context,
    pub state: Arc<State<T>>,
}

impl<T: Clone + Default + Display> Application<T> {
    pub fn new(cnf: Settings) -> Self {
        let ctx = Context::new(cnf.clone());
        let state = Arc::new(State::<T>::default());
        Self { cnf, ctx, state }
    }
    pub async fn setup(&mut self) -> AsyncResult<&Self> {
        // Initialize the logger
        self.clone().cnf.logger.unwrap_or_default().setup(None);
        tracing_subscriber::fmt::init();

        tracing::info!("Success: Application initialized and awaiting commands");
        Ok(self)
    }
    pub fn set_state(&mut self, state: State<T>) -> &Self {
        self.state = Arc::new(state);
        self
    }
    pub async fn run(&mut self) -> AsyncResult<&Self> {
        self.setup().await?;
        let cli = cli::new();
        tracing::info!("Success: Commands parsed, processing requests...");
        cli.handler().await?;
        Ok(self)
    }
}

impl<T: Clone + Default + Display + Serialize> std::fmt::Display for Application<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", serde_json::to_string_pretty(&self).unwrap())
    }
}
