/*
    Appellation: dblink <module>
    Contrib: @FL03
*/
use crate::config::DatabaseUrl;
use url::Url;

/// The [`DatabaseLink`] implementation enumerates the various compatible representations of a
/// connection or link to a database instance.
#[derive(
    Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[serde(untagged)]
pub enum DatabaseLink {
    DatabaseUrl(DatabaseUrl),
    String(String),
    Url(Url),
}

impl DatabaseLink {
    pub fn from_str<S>(s: S) -> Self
    where
        S: ToString,
    {
        Self::String(s.to_string())
    }

    pub const fn from_db_url(url: DatabaseUrl) -> Self {
        Self::DatabaseUrl(url)
    }
    /// convert the instance into a [`DatabaseUrl`]
    pub fn as_db_url(&self) -> DatabaseUrl {
        match self {
            Self::DatabaseUrl(url) => url.clone(),
            Self::String(url) => url.parse().expect("Failed to parse database url"),
            Self::Url(url) => url.as_str().parse().expect("Failed to parse database url"),
        }
    }
    /// convert the instance into a string representation
    pub fn to_string(&self) -> String {
        match self {
            Self::DatabaseUrl(url) => url.to_string(),
            Self::String(url) => url.to_string(),
            Self::Url(url) => url.to_string(),
        }
    }
    /// convert the instance into a [`url::Url`]
    pub fn as_url(&self) -> url::Url {
        self.to_string()
            .parse()
            .expect("Failed to parse database url")
    }
}

impl Default for DatabaseLink {
    fn default() -> Self {
        Self::DatabaseUrl(DatabaseUrl::default())
    }
}

impl From<url::Url> for DatabaseLink {
    fn from(url: url::Url) -> Self {
        DatabaseLink::String(url.to_string())
    }
}

impl From<DatabaseLink> for config::Value {
    fn from(link: DatabaseLink) -> Self {
        link.as_db_url().into()
    }
}
impl From<DatabaseLink> for DatabaseUrl {
    fn from(link: DatabaseLink) -> Self {
        link.as_db_url()
    }
}

impl From<DatabaseUrl> for DatabaseLink {
    fn from(url: DatabaseUrl) -> Self {
        DatabaseLink::DatabaseUrl(url)
    }
}
