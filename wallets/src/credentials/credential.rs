/*
    Appellation: credentials <module>
    Creator: FL03 <jo3mccain@icloud.com>
    Description:
        ... Summary ...
*/
use scsys::prelude::{AsyncResult, Timestamp};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct Credential {
    pub account: String,
    pub created: Timestamp,
    pub data: Vec<String>,
}

impl Credential {
    pub fn new(account: String, data: Vec<String>) -> Self {
        Self {
            account,
            created: Timestamp::default(),
            data,
        }
    }

    pub fn save_to_file(&self, path: &str) -> AsyncResult<Self> {
        crate::save_to_file(self.clone(), path)
    }
}

#[cfg(test)]
mod tests {
    use super::Credential;

    #[test]
    fn test_default_credential() {
        let actual = Credential::default();
        let expected = actual.clone();
        assert_eq!(actual, expected)
    }
}
