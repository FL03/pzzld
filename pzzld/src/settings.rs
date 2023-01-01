/*
   Appellation: settings
   Contrib: FL03 <jo3mccain@icloud.com>
   Description: ... Summary ...
*/
use scsys::prelude::config::{Config, Environment};
use scsys::Hashable;
use scsys::{prelude::*, try_collect_config_files, ConfigResult};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, Hash, Hashable, PartialEq, Serialize)]
pub struct Settings {
    #[serde(skip)]
    pub(crate) client_id: String,
    #[serde(skip)]
    pub(crate) client_secret: String,
    pub mode: String,
    pub logger: Logger,
    pub server: Server,
}

impl Settings {
    pub fn build() -> ConfigResult<Self> {
        let mut builder = Config::builder();
        // Set defaults
        builder = builder
            .set_default("mode", "production")?
            .set_default("logger.level", "info")?
            .set_default("server.host", "127.0.0.1")?
            .set_default("server.port", 8080)?;
        // Load in the .env file
        builder = builder.add_source(Environment::default().separator("__"));
        // Load in configuration files following the *.config.* pattern
        if let Ok(v) = try_collect_config_files("**/*.config.*", false) {
            builder = builder.add_source(v);
        }
        // Check for alternative environment variable representations
        if let Ok(v) = std::env::var("CLIENT_ID") {
            builder = builder.set_override("client_id", v)?;
        }
        if let Ok(v) = std::env::var("CLIENT_SECRET") {
            builder = builder.set_override("client_secret", v)?;
        }
        if let Ok(v) = std::env::var("RUST_LOG") {
            builder = builder.set_override("logger.level", v)?;
        }
        if let Ok(v) = std::env::var("SERVER_PORT") {
            builder = builder.set_override("server.port", v)?;
        }
        // Attempt to build, then deserialize the configuration
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
        match Self::build() {
            Ok(v) => v,
            Err(_) => Self {
                client_id: Default::default(),
                client_secret: Default::default(),
                mode: "production".to_string(),
                logger: Logger::default(),
                server: Server::new("127.0.0.1".to_string(), 8080),
            },
        }
    }
}

impl std::fmt::Display for Settings {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", serde_json::to_string_pretty(&self).unwrap())
    }
}
