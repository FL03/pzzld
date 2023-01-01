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

use acme::net::servers::Server;
use acme::prelude::{AppSpec, AsyncSpawable};
use scsys::prelude::{AsyncResult, Locked, State};
use std::sync::{Arc, Mutex};

#[tokio::main]
async fn main() -> AsyncResult {
    Application::default().spawn().await?;

    Ok(())
}

#[derive(Clone, Debug, Default)]
pub struct Application {
    pub cnf: Settings,
    pub ctx: Context,
    pub server: Arc<Server>,
    pub state: Locked<State<States>>,
}

impl Application {
    pub fn new(cnf: Settings) -> Self {
        let ctx = Context::new(cnf.clone());
        let server = Arc::new(Server::default());
        let state = Arc::new(Mutex::new(Default::default()));
        Self {
            cnf,
            ctx,
            server,
            state,
        }
    }
    pub fn update_state(&mut self, state: States) -> &Self {
        self.state = Arc::new(Mutex::new(State::new(None, None, Some(state))));
        self
    }
}

#[async_trait::async_trait]
impl AsyncSpawable for Application {
    async fn spawn(&mut self) -> AsyncResult<&Self> {
        self.setup()?;
        let cli = cli::new();
        tracing::info!("Success: Commands parsed, processing requests...");
        cli.handler().await?;
        Ok(self)
    }
}
impl AppSpec for Application {
    type Cnf = Settings;

    type Ctx = Context;

    type State = State<States>;

    fn init() -> Self {
        Self::new(Default::default())
    }

    fn context(&self) -> Self::Ctx {
        self.ctx.clone()
    }

    fn name(&self) -> String {
        env!("CARGO_PKG_NAME").to_string()
    }

    fn settings(&self) -> Self::Cnf {
        self.context().cnf.clone()
    }

    fn setup(&mut self) -> AsyncResult<&Self> {
        // Initialize the logger
        self.clone().cnf.logger.setup(None);
        tracing_subscriber::fmt::init();
        Ok(self)
    }

    fn state(&self) -> &scsys::Locked<Self::State> {
        &self.state
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
