/*
    Appellation: server <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... Summary ...
*/
use hyper::server::{conn::AddrIncoming, Builder};
use scsys::AsyncResult;
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct Server {
    pub host: [u8; 4],
    pub port: u16,
}

impl Server {
    pub fn new(host: Option<[u8; 4]>, port: Option<u16>) -> Self {
        Self {
            host: host.unwrap_or([127, 0, 0, 1]),
            port: port.unwrap_or(8080),
        }
    }
    /// Create a new [std::net::SocketAddr] instance from the provided port
    pub fn address(&self) -> SocketAddr {
        SocketAddr::from(([127, 0, 0, 1], self.port))
    }
    /// Creates a new builder instance with the address created from the given port
    pub fn builder(&self) -> Builder<AddrIncoming> {
        hyper::Server::bind(&self.address())
    }

    /// Serves the client
    pub async fn serve(&self, client: axum::Router) -> AsyncResult {
        self.builder()
            .serve(client.into_make_service())
            .with_graceful_shutdown(crate::signals::shutdown::shutdown())
            .await?;
        Ok(())
    }
}

impl Default for Server {
    fn default() -> Self {
        Self::new(Some([127, 0, 0, 1]), Some(8080))
    }
}

impl From<([u8; 4], u16)> for Server {
    fn from(data: ([u8; 4], u16)) -> Self {
        Self::new(Some(data.0), Some(data.1))
    }
}

impl From<u16> for Server {
    fn from(data: u16) -> Self {
        Self::new(None, Some(data))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_server() {
        let server = Server::default();
        assert_eq!(server.clone().host, [127, 0, 0, 1]);
        assert_eq!(server.clone().port, 8080);
    }
}
