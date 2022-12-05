/*
   Appellation: settings
   Contrib: FL03 <jo3mccain@icloud.com>
   Description: ... Summary ...
*/
use scsys::Hashable;
use scsys::prelude::config::{Config, Environment};
use scsys::{prelude::*, try_collect_config_files, ConfigResult};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, Hash, Hashable, PartialEq, Serialize)]
pub struct Settings {
    #[serde(skip)]
    pub(crate) client_id: String,
    #[serde(skip)]
    pub(crate) client_secret: String,
    pub mode: Option<String>,
    pub name: Option<String>,
    pub logger: Option<Logger>,
    pub server: Server,
}

impl Settings {
    pub fn build() -> ConfigResult<Self> {
        let mut builder = Config::builder()
            .add_source(Environment::default().separator("__"))
            .set_default("logger.level", Some("info"))?
            .set_default("server.host", "127.0.0.1")?
            .set_default("server.port", 9090)?;
        match try_collect_config_files("**/Puzzled.toml", false) {
            Err(_) => {}
            Ok(v) => {
                builder = builder.add_source(v);
            }
        }
        match std::env::var("CLIENT_ID") {
            Err(_) => {}
            Ok(v) => {
                builder = builder.set_override("client_id", Some(v))?;
            }
        };
        match std::env::var("CLIENT_SECRET") {
            Err(_) => {}
            Ok(v) => {
                builder = builder.set_override("client_secret", Some(v))?;
            }
        };
        match std::env::var("RUST_LOG") {
            Err(_) => {}
            Ok(v) => {
                builder = builder.set_override("logger.level", Some(v))?;
            }
        };

        match std::env::var("SERVER_PORT") {
            Err(_) => {}
            Ok(v) => {
                builder = builder.set_override("server.port", v)?;
            }
        };

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
                mode: Some("production".to_string()),
                name: Some("Flow".to_string()),
                logger: Some(Logger::default()),
                server: Server::new("127.0.0.1".to_string(), 9090),
            },
        }
    }
}

impl std::fmt::Display for Settings {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", serde_json::to_string_pretty(&self).unwrap())
    }
}
