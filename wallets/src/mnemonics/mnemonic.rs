/*
    Appellation: mnemonic <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... Summary ...
*/
use crate::{generate_collection_from_reference, mnemonics::Passphrase, Language, BIP0039};
use scsys::AsyncResult;
use serde::{Deserialize, Serialize};
use std::convert::From;

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct Mnemonic {
    pub mnemonic: Vec<String>,
    pub passphrase: Passphrase,
}

impl Mnemonic {
    pub fn new(mnemonic: Option<Vec<String>>, passphrase: Option<Passphrase>) -> Self {
        Self {
            mnemonic: mnemonic.unwrap_or_default(),
            passphrase: passphrase.unwrap_or_default(),
        }
    }
    pub async fn generate(&mut self, lang: Option<Language>) -> AsyncResult<&Self> {
        self.mnemonic = generate_collection_from_reference(BIP0039::fetch(lang).await?.into(), 12);
        Ok(self)
    }
    pub fn passphrase(&self) -> String {
        self.passphrase.passphrase().clone()
    }
    pub fn salt(&self) -> String {
        let salt = String::new();
        self.passphrase() + salt.as_str()
    }
}

impl Default for Mnemonic {
    fn default() -> Self {
        let passphrase = Passphrase::generate(12);
        Self::from(passphrase)
    }
}

impl From<Passphrase> for Mnemonic {
    fn from(data: Passphrase) -> Self {
        let mnemonic = generate_collection_from_reference(BIP0039::default().into(), 12);
        Self::new(Some(mnemonic), Some(data))
    }
}

#[cfg(test)]
mod tests {
    use crate::mnemonics::{Mnemonic, Passphrase};

    #[test]
    fn test_mnemonic() {
        let a = Mnemonic::from(Passphrase::default());
        let b = Mnemonic::default();
        assert_ne!(a, b);
        assert_eq!(a.passphrase().len(), b.passphrase().len())
    }
}
