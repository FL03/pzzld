/*
    Appellation: database <config>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use crate::config::{DatabaseLink, DatabaseUrl};

const fn default_max_connections() -> u32 {
    200
}

const fn default_pool_size() -> u32 {
    15
}

#[derive(Clone, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize)]
#[serde(default, deny_unknown_fields, rename_all = "snake_case")]
pub struct DatabaseConfig {
    #[serde(default = "default_max_connections")]
    pub max_connections: u32,
    #[serde(default = "default_pool_size")]
    pub pool_size: u32,
    #[serde(alias = "link", alias = "uri")]
    pub url: DatabaseLink,
}

impl DatabaseConfig {
    pub fn new() -> Self {
        Self {
            max_connections: default_max_connections(),
            pool_size: default_pool_size(),
            url: DatabaseLink::default(),
        }
    }
    /// returns the maximum number of connections
    pub const fn max_connections(&self) -> u32 {
        self.max_connections
    }
    /// returns the pool size for the database
    pub const fn pool_size(&self) -> u32 {
        self.pool_size
    }
    /// returns a reference to the database url
    pub fn url(&self) -> DatabaseUrl {
        self.url.as_db_url()
    }
    /// consumes the instance to create another with the given maximum number of connections
    pub fn with_max_connections(self, max_connections: u32) -> Self {
        Self {
            max_connections,
            ..self
        }
    }
    /// consumes the instance to create another with the given pool size
    pub fn with_pool_size(self, pool_size: u32) -> Self {
        Self { pool_size, ..self }
    }
    //// consumes the instance to create another with the given url
    pub fn with_url(self, url: DatabaseLink) -> Self {
        Self { url, ..self }
    }
    /// update the configured maximum number of connections
    pub fn set_max_connections(&mut self, max_connections: u32) {
        self.max_connections = max_connections;
    }
    /// update the pool size for the database
    pub fn set_pool_size(&mut self, pool_size: u32) {
        self.pool_size = pool_size;
    }
    /// update the database url
    pub fn set_url(&mut self, url: DatabaseLink) {
        self.url = url;
    }
}

impl Default for DatabaseConfig {
    fn default() -> Self {
        Self::new()
    }
}

impl core::fmt::Debug for DatabaseConfig {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str(&serde_json::to_string_pretty(self).unwrap())
    }
}

impl core::fmt::Display for DatabaseConfig {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str(&serde_json::to_string(self).unwrap())
    }
}
