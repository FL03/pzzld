/*
   Appellation: Pzzld <binary>
   Contrib: FL03 <jo3mccain@icloud.com>
   Description: ... Summary ...
*/
pub use self::{context::*, settings::*, states::States};

pub mod api;
pub mod cli;

pub(crate) mod context;
pub(crate) mod settings;
pub(crate) mod states;

use pzzld::core::servers::Server;
use scsys::prelude::{AsyncResult, State};
use std::sync::Arc;

#[tokio::main]
async fn main() -> AsyncResult {
    Application::default().run().await?;

    Ok(())
}

#[derive(Clone, Debug, Default)]
pub struct Application {
    pub cnf: Settings,
    pub ctx: Context,
    pub server: Arc<Server>,
    pub state: Arc<State<States>>,
}

impl Application {
    pub fn new(cnf: Settings) -> Self {
        let ctx = Context::new(cnf.clone());
        let server = Arc::new(Server::default());
        let state = Arc::new(State::default());
        Self {
            cnf,
            ctx,
            server,
            state,
        }
    }
    pub async fn setup(&mut self) -> AsyncResult<&Self> {
        // Initialize the logger
        self.clone().cnf.logger.setup(None);
        tracing_subscriber::fmt::init();
        Ok(self)
    }
    pub async fn run(&mut self) -> AsyncResult<&Self> {
        self.setup().await?;
        let cli = cli::new();
        tracing::info!("Success: Commands parsed, processing requests...");
        cli.handler().await?;
        Ok(self)
    }
    pub fn update_state(&mut self, state: State<States>) -> &Self {
        self.state = Arc::new(state);
        self
    }
}

impl std::fmt::Display for Application {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            format!(
                "view the application locally at http://localhost:{}",
                self.cnf.server.port
            )
        )
    }
}
