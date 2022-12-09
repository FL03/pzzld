/*
    Appellation: mnemonic <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... Summary ...
*/
use crate::{generate_collection_from_reference, mnemonics::Passphrase};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct Mnemonic {
    pub mnemonic: Vec<String>,
    pub passphrase: Passphrase,
}

impl Mnemonic {
    pub fn new(passphrase: Passphrase) -> Self {
        let mnemonic =
            generate_collection_from_reference(crate::BIP0039::default().data().clone(), 12);
        Self {
            mnemonic,
            passphrase,
        }
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
        Self::new(Default::default())
    }
}

#[cfg(test)]
mod tests {
    use crate::mnemonics::{Mnemonic, Passphrase};

    #[test]
    fn test_mnemonic() {
        let a = Mnemonic::new(Passphrase::default());
        let b = Mnemonic::default();
        assert_ne!(a, b);
        assert_eq!(a.passphrase().len(), b.passphrase().len())
    }
}
