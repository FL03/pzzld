/*
    Appellation: primitives <module>
    Creator: FL03 <jo3mccain@icloud.com>
    Description:
        ... Summary ...
*/
pub use self::{constants::*, types::*};
use scsys::prelude::AsyncResult;
use serde::{Deserialize, Serialize};
use strum::{EnumString, EnumVariantNames};

#[derive(
    Clone,
    Copy,
    Debug,
    Default,
    Deserialize,
    EnumString,
    EnumVariantNames,
    Eq,
    Hash,
    PartialEq,
    Serialize,
)]
pub enum Language {
    #[default]
    English,
    French,
}

impl std::convert::From<&Self> for Language {
    fn from(data: &Self) -> Self {
        data.clone()
    }
}

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct BIP0039(pub Vec<String>);

impl BIP0039 {
    pub fn new(data: Vec<String>) -> Self {
        Self(data)
    }
    pub async fn fetch(lang: Language) -> AsyncResult<Self> {
        let response = reqwest::get(format!("{}/{}.txt", BIP0039_WORDLIST_ENDPOINT, "english"))
            .await?
            .text()
            .await?;
        let mut data = response.split("\n").collect::<Vec<_>>();
        data.retain(|&x| x != "");
        let res = data.iter().map(|i| i.to_string()).collect();
        Ok(Self::new(res))
    }
    pub fn from_file() -> Self {
        let mut data = crate::extract_file_from_path("./BIP0039/english.txt");
        data.retain(|x| x != &"".to_string());
        Self::new(data)
    }
}

impl Default for BIP0039 {
    fn default() -> Self {
        Self::from_file()
    }
}

mod constants {
    /// Define the valid sizes of generated access grants
    pub const ACCESS_GRANT_VALID_BIT_SIZES: [usize; 5] = [128, 160, 192, 224, 256];
    /// Define the default filepath for locating the BIP0039 english text file
    pub const PATH_TO_BIP0039_DATA: &str = "**/BIP0039/english.txt";
    /// Define the endpoint pointing to BIP0039 Mnemonics
    pub const BIP0039_WORDLIST_ENDPOINT: &str =
        "https://raw.githubusercontent.com/bitcoin/bips/master/bip-0039";
}

mod types {
    use secp256k1::{PublicKey, SecretKey};

    /// Type alias for a tuple ([secp256k1::SecretKey], [secp256k1::PublicKey])
    pub type SecpKeypair = (SecretKey, PublicKey);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_wordlist_english() {
        let actual = BIP0039::fetch(Language::English).await.ok().unwrap();
        let expected = actual.clone();
        assert_eq!(actual, expected)
    }
}
