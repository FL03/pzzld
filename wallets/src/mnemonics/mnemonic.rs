/*
    Appellation: mnemonic <module>
    Creator: FL03 <jo3mccain@icloud.com>
    Description:
        ... Summary ...
*/
use crate::{generate_collection_from_reference, mnemonics::Passphrase};

#[derive(Clone, Debug, Hash, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct Mnemonic {
    pub mnemonic: Vec<String>,
    pub passphrase: Passphrase,
}

impl Mnemonic {
    fn constructor(mnemonic: Vec<String>, passphrase: Passphrase) -> Self {
        Self {
            mnemonic,
            passphrase,
        }
    }
    pub fn new(passphrase: Passphrase) -> Self {
        let mnemonic = generate_collection_from_reference(crate::BIP0039::default().0, 12);
        Self::constructor(mnemonic, passphrase)
    }
    pub fn passphrase(&self) -> String {
        self.passphrase.0.clone()
    }
    pub fn salt(&self) -> String {
        let salt = String::new();
        self.passphrase() + salt.as_str()
    }
}

#[cfg(test)]
mod tests {
    use crate::mnemonics::{Mnemonic, Passphrase};

    #[test]
    fn test_mnemonic() {
        let a = Mnemonic::new(Passphrase::default());
        let b = Mnemonic::new(Passphrase::default());
        assert_ne!(a, b)
    }
}
