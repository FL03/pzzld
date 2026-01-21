/*
    Appellation: db_url <module>
    Created At: 2026.01.11:07:29:08
    Contrib: @FL03
*/
use url::Url;

/// [`DatabaseUrl`] represents a structured database connection URL, providing options for the
/// scheme, host, port, username, password, and database name.
#[derive(Clone, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize)]
#[serde(default, deny_unknown_fields, rename_all = "snake_case")]
pub struct DatabaseUrl {
    #[serde(alias = "prefix", alias = "provider")]
    pub scheme: String,
    pub host: String,
    pub port: u16,
    #[serde(alias = "user")]
    pub username: String,
    pub password: String,
    pub database: String,
}

impl DatabaseUrl {
    pub const DEFAULT_URL: &'static str = "postgresql://postgres:password@localhost:5432/postgres";
    /// initializes a new instance of the db url
    pub fn new() -> Self {
        Self {
            scheme: "postgresql".to_string(),
            host: "localhost".to_string(),
            port: 5432,
            username: "postgres".to_string(),
            password: "password".to_string(),
            database: "postgres".to_string(),
        }
    }
    /// try to parse an instance from the given url
    pub fn from_url_str<T>(url: T) -> Self
    where
        T: AsRef<str>,
    {
        url.as_ref().parse().expect("Failed to parse database url")
    }
    /// load and parse the database url from the configured environment variable
    pub fn from_env(key: &str) -> Self {
        std::env::var(key)
            .unwrap_or_else(|_| Self::DEFAULT_URL.to_string())
            .parse()
            .expect("Failed to parse database url")
    }
    /// returns a reference to the scheme of the database url
    pub fn scheme(&self) -> &str {
        &self.scheme
    }
    /// returns a reference to the host of the database url
    pub fn host(&self) -> &str {
        &self.host
    }
    /// returns the configured port
    pub fn port(&self) -> u16 {
        self.port
    }
    /// returns a reference to the username
    pub fn username(&self) -> &str {
        &self.username
    }
    /// returns a reference to the password
    pub fn password(&self) -> &str {
        &self.password
    }
    /// returns a reference to the database name
    pub fn database(&self) -> &str {
        &self.database
    }
    /// update the database name
    pub fn set_database<T>(&mut self, database: T)
    where
        T: ToString,
    {
        self.database = database.to_string();
    }
    /// update the database host
    pub fn set_host<T>(&mut self, host: T)
    where
        T: ToString,
    {
        self.host = host.to_string();
    }
    /// update the port
    pub fn set_port(&mut self, port: u16) {
        self.port = port;
    }
    /// set the username
    pub fn set_username<T>(&mut self, username: T)
    where
        T: ToString,
    {
        self.username = username.to_string();
    }
    /// update the database password
    pub fn set_password<T>(&mut self, password: T)
    where
        T: ToString,
    {
        self.password = password.to_string();
    }
    /// update the scheme of the database
    pub fn set_scheme<T>(&mut self, scheme: T)
    where
        T: ToString,
    {
        self.scheme = scheme.to_string();
    }
    /// returns a new instance with the given database name
    pub fn with_database<T>(self, database: T) -> Self
    where
        T: ToString,
    {
        Self {
            database: database.to_string(),
            ..self
        }
    }
    /// returns a new instance with the given host
    pub fn with_host<T>(self, host: T) -> Self
    where
        T: ToString,
    {
        Self {
            host: host.to_string(),
            ..self
        }
    }
    /// returns a new instance with the given port
    pub fn with_port(self, port: u16) -> Self {
        Self { port, ..self }
    }
    /// returns a new instance with the given username
    pub fn with_username<T>(self, username: T) -> Self
    where
        T: ToString,
    {
        Self {
            username: username.to_string(),
            ..self
        }
    }
    /// returns a new instance with the given password
    pub fn with_password<T>(self, password: T) -> Self
    where
        T: ToString,
    {
        Self {
            password: password.to_string(),
            ..self
        }
    }
    /// returns a new instance with the given scheme
    pub fn with_scheme<T>(self, scheme: T) -> Self
    where
        T: ToString,
    {
        Self {
            scheme: scheme.to_string(),
            ..self
        }
    }
    /// returns a string representation of the url
    pub fn format(&self) -> String {
        format! {
            "{}://{}:{}@{}:{}/{}",
            self.scheme, self.username, self.password, self.host, self.port, self.database
        }
    }
    /// returns the database url as a [`Url`]
    pub fn as_url(&self) -> Url {
        self.to_string()
            .parse()
            .expect("Failed to parse database url")
    }
}

impl Default for DatabaseUrl {
    fn default() -> Self {
        Self::new()
    }
}

impl core::fmt::Debug for DatabaseUrl {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str(&self.format())
    }
}

impl core::fmt::Display for DatabaseUrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.write_str(&self.format())
    }
}

impl core::str::FromStr for DatabaseUrl {
    type Err = url::ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // parse the string as a url
        let url = s.parse::<url::Url>()?;
        // extract the values from the url
        let provider = url.scheme().to_string();
        let host = url.host_str().unwrap().to_string();
        let port = url.port().unwrap();
        let user = url.username().to_string();
        let password = url.password().unwrap().to_string();
        let database = url.path().to_string();
        // create a new instance from the parsed values
        let instance = Self {
            scheme: provider,
            host,
            port,
            username: user,
            password,
            database,
        };
        // return the instance
        Ok(instance)
    }
}

impl<'a> core::cmp::PartialEq<&'a str> for DatabaseUrl {
    fn eq(&self, other: &&'a str) -> bool {
        self.to_string() == *other
    }
}

impl core::cmp::PartialEq<String> for DatabaseUrl {
    fn eq(&self, other: &String) -> bool {
        self.to_string() == *other
    }
}

impl From<String> for DatabaseUrl {
    fn from(url: String) -> Self {
        url.parse().expect("Failed to parse database url")
    }
}

impl From<DatabaseUrl> for String {
    fn from(url: DatabaseUrl) -> Self {
        url.to_string()
    }
}

impl From<DatabaseUrl> for config::Value {
    fn from(url: DatabaseUrl) -> Self {
        url.to_string().into()
    }
}

impl From<config::Value> for DatabaseUrl {
    fn from(value: config::Value) -> Self {
        value
            .to_string()
            .parse()
            .expect("Failed to parse database url")
    }
}

impl From<Url> for DatabaseUrl {
    fn from(url: Url) -> Self {
        url.to_string()
            .parse()
            .expect("Failed to parse database url")
    }
}

impl From<DatabaseUrl> for Url {
    fn from(url: DatabaseUrl) -> Self {
        url.to_string()
            .parse()
            .expect("Failed to parse database url")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_database_url() {
        let str = "postgres://postgres:postgres@localhost:5432/postgres";
        let url = str.parse::<DatabaseUrl>().unwrap();
        assert_eq!(url.scheme, "postgres");
        assert_eq!(url.host, "localhost");
        assert_eq!(url.port, 5432);
        assert_eq!(url.username, "postgres");
        assert_eq!(url.password, "postgres");
        assert_eq!(url.database, "postgres");
        assert_eq!(url, str);
    }
}
