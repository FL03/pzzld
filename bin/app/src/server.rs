/*
    Appellation: server <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... Summary ...
*/
use hyper::server::conn::AddrIncoming;
use scsys::AsyncResult;
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
    /// Create a new [std::net::SocketAddr] instance from the provided port
    pub fn address(&self) -> SocketAddr {
        SocketAddr::from(([127, 0, 0, 1], self.port))
    }
    /// Creates a new builder instance with the address created from the given port
    pub fn builder(&self) -> hyper::server::Builder<AddrIncoming> {
        hyper::Server::bind(&self.address())
    }
    /// Serves the client 
    pub async fn serve(&self, client: axum::Router) -> AsyncResult {
        self.builder()
            .serve(client.into_make_service())
            .with_graceful_shutdown(crate::api::shutdown())
            .await?;
        Ok(())
    }
}

impl Default for BackendServer {
    fn default() -> Self {
        Self::new(None)
    }
}
