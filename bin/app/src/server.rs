/*
    Appellation: server <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... Summary ...
*/
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct BackendServer {
    pub port: u16
}

impl BackendServer {
    pub fn new(port: Option<u16>) -> Self {
        let port = port.unwrap_or(8080);
        Self { port }
    }
    pub fn address(&self) -> SocketAddr {
        SocketAddr::from(([127, 0, 0, 1], self.port))
    }
}

impl Default for BackendServer {
    fn default() -> Self {
        Self::new(None)
    }
}
