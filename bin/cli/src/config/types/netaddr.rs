/*
    Appellation: server_addr <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/

fn default_ip() -> String {
    core::net::IpAddr::V4(core::net::Ipv4Addr::LOCALHOST).to_string()
}

const fn default_port() -> u16 {
    NetAddr::DEFAULT_PORT
}

#[derive(
    Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[serde(default)]
pub struct NetAddr {
    #[serde(default = "default_ip")]
    pub host: String,
    #[serde(default = "default_port")]
    pub port: u16,
}

impl NetAddr {
    pub const LOCALHOST: &'static str = "0.0.0.0";
    pub const DEFAULT_HOST: &'static str = "0.0.0.0";
    pub const DEFAULT_PORT: u16 = 8080;

    /// initialize a new instance of the network address from the given components
    pub fn new<H>(host: H, port: u16) -> Self
    where
        H: ToString,
    {
        Self {
            host: host.to_string(),
            port,
        }
    }

    pub fn from_ip_with_port(ip: core::net::IpAddr, port: u16) -> Self {
        Self {
            host: ip.to_string(),
            port,
        }
    }
    /// convert the given socket address into a valid network address
    pub fn from_socket_addr(addr: core::net::SocketAddr) -> Self {
        Self {
            host: addr.ip().to_string(),
            port: addr.port(),
        }
    }
    /// initialize a new instance of the network address bound to localhost and the given port
    pub fn localhost(port: u16) -> Self {
        Self {
            host: Self::LOCALHOST.to_string(),
            port,
        }
    }
    /// convert a reference of the network address into a [`SocketAddr`](core::net::SocketAddr)
    pub fn as_socket_addr(&self) -> core::net::SocketAddr {
        self.to_string().parse().unwrap()
    }
    #[cfg(feature = "net")]
    /// initialize a new listener, bound to the configured address
    pub async fn bind(&self) -> std::io::Result<tokio::net::TcpListener> {
        tokio::net::TcpListener::bind(&self.as_socket_addr()).await
    }
    /// returns the ip of the address
    pub fn ip(&self) -> core::net::IpAddr {
        self.as_socket_addr().ip()
    }
    /// returns the host
    pub fn host(&self) -> &str {
        &self.host
    }
    /// returns a copy to the port
    pub fn port(&self) -> u16 {
        self.port
    }
    /// update the host of the address
    pub fn set_host<T>(&mut self, host: T)
    where
        T: ToString,
    {
        self.host = host.to_string();
    }
    /// update the port of the address
    pub fn set_port(&mut self, port: u16) {
        self.port = port;
    }
    /// consumes the address to create another with the given host
    pub fn with_host<T>(self, host: T) -> Self
    where
        T: ToString,
    {
        Self {
            host: host.to_string(),
            ..self
        }
    }
    /// consumes the address to create another with the given port
    pub fn with_port(self, port: u16) -> Self {
        Self { port, ..self }
    }
}

impl Default for NetAddr {
    fn default() -> Self {
        Self::localhost(Self::DEFAULT_PORT)
    }
}

impl core::fmt::Display for NetAddr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{host}:{port}", host = self.host, port = self.port)
    }
}

impl core::str::FromStr for NetAddr {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // parse as a url to get the scheme, host and port
        let url = s.parse::<url::Url>()?;
        let host = url.host_str().expect("Failed to parse host");
        let port = url.port().expect("failed to parse port");

        let res = Self {
            host: host.to_string(),
            port,
        };
        Ok(res)
    }
}

impl From<core::net::SocketAddr> for NetAddr {
    fn from(addr: core::net::SocketAddr) -> Self {
        Self::from_socket_addr(addr)
    }
}
