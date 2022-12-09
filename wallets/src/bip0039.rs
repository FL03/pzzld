/*
    Appellation: primitives <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... Summary ...
*/
use crate::{extract_file_from_path, BIP0039_WORDLIST_ENDPOINT, Language};
use scsys::AsyncResult;
use serde::{Deserialize, Serialize};
use std::convert::From;

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct BIP0039(Vec<String>);

impl BIP0039 {
    pub fn new(data: Vec<String>) -> Self {
        Self(data)
    }
    pub fn data(&self) -> &Vec<String> {
        &self.0
    }
    pub async fn fetch(lang: Option<Language>) -> AsyncResult<Self> {
        let response = reqwest::get(format!("{}/{}.txt", BIP0039_WORDLIST_ENDPOINT, lang.unwrap_or_default().to_string()))
            .await?
            .text()
            .await?;
        Ok(Self::from(response.split("\n").collect::<Vec<_>>()))
    }
    pub fn from_file() -> Self {
        Self::from(extract_file_from_path("./BIP0039/english.txt"))
    }
}

impl Default for BIP0039 {
    fn default() -> Self {
        Self::from_file()
    }
}

impl std::fmt::Display for BIP0039 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", serde_json::to_string(&self).unwrap().to_ascii_lowercase())
    }
}


impl From<&BIP0039> for BIP0039 {
    fn from(data: &BIP0039) -> Self {
        Self::new(data.data().clone())
    }
}


impl<T> From<Vec<T>> for BIP0039 where T: Clone + ToString {
    fn from(data: Vec<T>) -> Self {
        let mut data = data.iter().cloned().map(|i: T| i.to_string()).collect::<Vec<String>>();
        data.retain(|x| x != &"".to_string());
        Self::new(data)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_wordlist_english() {
        let a = BIP0039::fetch(None).await.ok().unwrap();
        let b = a.clone();
        assert_eq!(a, b)
    }
}
