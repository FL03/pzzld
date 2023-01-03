/*
   Appellation: Pzzld <binary>
   Contrib: FL03 <jo3mccain@icloud.com>
   Description: ... Summary ...
*/
pub use self::{context::*, settings::*, states::*};

pub mod api;
pub mod cli;

pub(crate) mod context;
pub(crate) mod settings;
pub(crate) mod states;

use acme::net::servers::Server;
use acme::prelude::{AppSpec, AsyncSpawnable};
use pzzld_sdk::prelude::{Gateway, GatewayConfig};
use scsys::prelude::{AsyncResult, Contextual, Locked};
use std::sync::{Arc, Mutex};

#[tokio::main]
async fn main() -> AsyncResult {
    // Initialize, then spawn the application
    Application::default().spawn().await?;
    
    Ok(())
}

#[derive(Clone, Debug, Default)]
pub struct Application {
    pub ctx: Context,
    pub server: Arc<Server>,
    pub state: Locked<State>,
}

impl Application {
    pub fn new(cnf: Settings) -> Self {
        let gateway = GatewayConfig::build().ok().unwrap();

        let ctx = Context::new(cnf, Gateway::from(gateway));
        let server = Arc::new(Server::default());
        let state = Arc::new(Mutex::new(Default::default()));
        Self { ctx, server, state }
    }
    // Update the application state
    pub fn update_state(&mut self, state: States) -> &Self {
        self.state = Arc::new(Mutex::new(State::new(None, None, Some(state))));
        self
    }
}

#[async_trait::async_trait]
impl AsyncSpawnable for Application {
    async fn spawn(&mut self) -> AsyncResult<&Self> {
        self.setup()?;
        let cli = cli::new();
        tracing::info!("Success: Commands parsed, processing requests...");
        cli.handler().await?;
        Ok(self)
    }
}

impl AppSpec<Settings> for Application {
    type Ctx = Context;

    type State = State;

    fn init() -> Self {
        Self::new(Default::default())
    }

    fn context(&self) -> Self::Ctx {
        self.ctx.clone()
    }

    fn name(&self) -> String {
        env!("CARGO_PKG_NAME").to_string()
    }

    fn settings(&self) -> Settings {
        self.ctx.cnf.clone()
    }

    fn setup(&mut self) -> AsyncResult<&Self> {
        // Initialize the logger
        self.settings().logger.setup(None);
        tracing_subscriber::fmt::init();
        Ok(self)
    }

    fn state(&self) -> &scsys::Locked<Self::State> {
        &self.state
    }

    fn slug(&self) -> String {
        self.name().to_ascii_lowercase()
    }
}

impl Contextual for Application {
    type Cnf = Settings;

    type Ctx = Context;

    fn context(&self) -> &Self::Ctx {
        &self.ctx
    }
}

impl std::fmt::Display for Application {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            serde_json::json!({
                "name": self.name()
            })
        )
    }
}
