/*
    Appellation: settings <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... Summary ...
*/
use scsys::prelude::config::{Config, Environment};
use scsys::Hashable;
use scsys::{prelude::*, try_collect_config_files, ConfigResult};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, Hash, Hashable, PartialEq, Serialize)]
pub struct Settings {
    pub client_id: String,
    pub client_secret: String,
    pub logger: Logger,
    pub server: Server,
}

impl Settings {
    pub fn new() -> Self {
        Self { client_id: String::new(), client_secret: String::new(), logger: Logger::default(), server: Server::default(), }
    }
    pub fn build() -> ConfigResult<Self> {
        let mut builder = Config::builder()
            .add_source(Environment::default().separator("__"))
            .set_default("logger.level", "info")?
            .set_default("server.host", "127.0.0.1")?
            .set_default("server.port", 8080)?;

        if let Ok(f) = try_collect_config_files("**/Puzzled.toml", false) {
            builder = builder.add_source(f);
        }

        if let Ok(cid) = std::env::var("CLIENT_ID") {
            builder = builder.set_override("client_id", cid)?;
        }

        if let Ok(sk) = std::env::var("CLIENT_SECRET") {
            builder = builder.set_override("client_secret", sk)?;
        }

        if let Ok(lvl) = std::env::var("RUST_LOG") {
            builder = builder.set_override("logger.level", lvl)?;
        }
        if let Ok(port) = std::env::var("SERVER_PORT") {
            builder = builder.set_override("server.port", port)?;
        }

        builder.build()?.try_deserialize()
    }
}

impl Configurable for Settings {
    type Settings = Self;

    fn settings(&self) -> &Self::Settings {
        self
    }
}

impl Default for Settings {
    fn default() -> Self {
        Self::build().unwrap_or_else(|_| Self::new())
    }
}

impl std::fmt::Display for Settings {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", serde_json::to_string_pretty(&self).unwrap())
    }
}