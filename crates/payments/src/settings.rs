/*
    Appellation: settings <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... Summary ...
*/
use scsys::prelude::{
    config::{Config, Environment},
    try_collect_config_files, AsyncResult, ConfigResult, Configurable,
};
use serde::{Deserialize, Serialize};

pub struct StripeConfig {
    
}

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct PaymentConfig {
    pub access_key: String,
    pub secret_key: String,
}

impl PaymentConfig {
    pub fn new(
        access_key: String,
        secret_key: String,
    ) -> Self {
        Self {
            access_key,
            secret_key,
        }
    }
    pub fn build() -> ConfigResult<Self> {
        let mut builder = Config::builder()
            .set_default("access_key", "")?
            .set_default("secret_key", "")?
            .add_source(Environment::default().prefix("STRIPE").separator("_"));

        if let Ok(v) = try_collect_config_files("**/*.config.*", false) {
            builder = builder.add_source(v);
        };
        if let Ok(v) = std::env::var("STRIPE_ACCESS_KEY") {
            builder = builder.set_override("access_key", v)?;
        }
        if let Ok(v) = std::env::var("STRIPE_SECRET_KEY") {
            builder = builder.set_override("secret_key", v)?;
        }

        builder.build()?.try_deserialize()
    }
    
}

impl Configurable for PaymentConfig {
    type Settings = Self;

    fn settings(&self) -> &Self::Settings {
        self
    }
}

impl Default for PaymentConfig {
    fn default() -> Self {
        Self::build().ok().unwrap()
    }
}
