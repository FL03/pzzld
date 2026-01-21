/*
    Appellation: services <module>
    Contrib: @FL03
*/
use super::{DatabaseConfig, TracingConfig};

#[derive(
    Clone,
    Debug,
    Default,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
    serde::Deserialize,
    serde::Serialize,
)]
#[serde(default)]
pub struct ServicesConfig {
    pub database: DatabaseConfig,
    pub tracing: TracingConfig,
}

impl ServicesConfig {
    /// initialize a new instance of the services configuration
    pub fn new() -> Self {
        Self {
            database: DatabaseConfig::new(),
            tracing: TracingConfig::new(crate::config::LogLevel::Info),
        }
    }
    /// returns a reference to the database configuration
    pub const fn database(&self) -> &DatabaseConfig {
        &self.database
    }
    /// returns a mutable reference to the database configuration
    pub const fn database_mut(&mut self) -> &mut DatabaseConfig {
        &mut self.database
    }
    /// returns a reference to the tracing configuration
    pub const fn tracing(&self) -> &TracingConfig {
        &self.tracing
    }
    /// returns a mutable reference to the tracing configuration
    pub const fn tracing_mut(&mut self) -> &mut TracingConfig {
        &mut self.tracing
    }
    /// update the database configuration
    pub fn set_database(&mut self, database: DatabaseConfig) {
        self.database = database
    }
    /// update the tracing configuration
    pub const fn set_tracing(&mut self, tracing: TracingConfig) {
        self.tracing = tracing
    }
    /// consumes the current instance to set the database configuration
    pub fn with_database(self, database: DatabaseConfig) -> Self {
        Self { database, ..self }
    }
    /// consumes the current instance to set the tracing configuration
    pub fn with_tracing(self, tracing: TracingConfig) -> Self {
        Self { tracing, ..self }
    }
}

impl core::fmt::Display for ServicesConfig {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write! { f, "{{ database: {}, tracing: {} }}", self.database, self.tracing }
    }
}
