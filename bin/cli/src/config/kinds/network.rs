/*
    Appellation: server <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use crate::config::NetAddr;

fn _default_basepath() -> String {
    crate::config::DEFAULT_BASEPATH.to_string()
}

fn _default_max_connections() -> u16 {
    15
}

#[derive(
    Clone, Default, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[serde(default, deny_unknown_fields, rename_all = "snake_case")]
pub struct NetworkConfig {
    // #[serde(flatten)]
    pub(crate) address: NetAddr,
    #[serde(default = "_default_basepath")]
    pub(crate) basepath: String,
    #[serde(default = "_default_max_connections")]
    pub(crate) max_connections: u16,
    pub(crate) open: bool,
}

impl NetworkConfig {
    pub fn new() -> Self {
        Self {
            address: NetAddr::default(),
            basepath: _default_basepath(),
            max_connections: _default_max_connections(),
            open: false,
        }
    }

    /// returns a reference to the address
    pub const fn address(&self) -> &NetAddr {
        &self.address
    }
    /// returns a mutable reference to the address
    pub const fn address_mut(&mut self) -> &mut NetAddr {
        &mut self.address
    }
    /// returns a reference to the basepath as a str
    pub fn basepath(&self) -> &str {
        &self.basepath
    }
    /// returns the maximum number of connections
    pub const fn max_connections(&self) -> u16 {
        self.max_connections
    }
    /// returns a mutable reference to the maximum number of connections
    pub const fn max_connections_mut(&mut self) -> &mut u16 {
        &mut self.max_connections
    }
    /// consumes the current instance to create another that should open by default
    pub fn should_open(self) -> Self {
        Self { open: true, ..self }
    }
    /// consumes the instance to create another that should not open by default
    pub fn should_not_open(self) -> Self {
        Self {
            open: false,
            ..self
        }
    }
    /// consumes the current instance to create another with the given address
    pub fn with_address(self, address: NetAddr) -> Self {
        Self { address, ..self }
    }
    /// consumes the current instance to create another with the given basepath
    pub fn with_basepath(self, basepath: impl ToString) -> Self {
        Self {
            basepath: basepath.to_string(),
            ..self
        }
    }
    /// update the current address and return a mutable reference to self
    pub fn set_address(&mut self, address: NetAddr) {
        self.address = address
    }
    /// update the current basepath and return a mutable reference to self
    pub fn set_basepath<T>(&mut self, basepath: T)
    where
        T: ToString,
    {
        self.basepath = basepath.to_string()
    }
    /// update the maximum number of connections and return a mutable reference to self
    pub const fn set_max_connections(&mut self, max_connections: u16) {
        self.max_connections = max_connections
    }
    /// Returns an instance of the address as a [SocketAddr](core::net::SocketAddr)
    pub fn as_socket_addr(&self) -> core::net::SocketAddr {
        self.address.as_socket_addr()
    }
    #[cfg(feature = "full")]
    /// Binds the address to a [TcpListener](tokio::net::TcpListener)
    pub async fn bind(&self) -> std::io::Result<tokio::net::TcpListener> {
        self.address().bind().await
    }
    /// Returns the host of the address
    pub fn host(&self) -> &str {
        self.address().host()
    }
    /// Returns the ip of the address
    pub fn ip(&self) -> core::net::IpAddr {
        self.address().ip()
    }
    /// Determines if the server should open the link in a browser
    pub fn open(&self) -> bool {
        self.open
    }
    /// Returns the port of the address
    pub fn port(&self) -> u16 {
        self.address.port
    }
    /// Sets the port of the address
    pub fn set_port(&mut self, port: u16) {
        self.address.port = port;
    }
    /// consumes the current instance and returns a new instance with the port set
    pub fn with_port(self, port: u16) -> Self {
        Self {
            address: NetAddr {
                port,
                ..self.address
            },
            ..self
        }
    }
    /// Sets the host of the address
    pub fn set_host(&mut self, host: impl ToString) {
        self.address.host = host.to_string();
    }
    /// consumes the current instance and returns a new instance with the host set
    pub fn with_host(self, host: impl ToString) -> Self {
        Self {
            address: self.address.with_host(host),
            ..self
        }
    }
}

impl core::fmt::Debug for NetworkConfig {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str(serde_json::to_string_pretty(self).unwrap().as_str())
    }
}

impl core::fmt::Display for NetworkConfig {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str(serde_json::to_string(self).unwrap().as_str())
    }
}
