/*
    Appellation: pzzld-wallets <library>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... Summary ...
*/
#[doc(inline)]
pub use self::{bip0039::*, lang::*, primitives::*, utils::*};

pub(crate) mod bip0039;
pub(crate) mod lang;
pub(crate) mod utils;

pub mod assets;
pub mod authorize;
pub mod credentials;
pub mod mnemonics;
pub mod wallets;

pub(crate) mod primitives {
    use secp256k1::{PublicKey, SecretKey};

    /// Define the valid sizes of generated access grants
    pub const ACCESS_GRANT_VALID_BIT_SIZES: [usize; 5] = [128, 160, 192, 224, 256];
    /// Define the default filepath for locating the BIP0039 english text file
    pub const PATH_TO_BIP0039_DATA: &str = "**/BIP0039/english.txt";
    /// Define the endpoint pointing to BIP0039 Mnemonics
    pub const BIP0039_WORDLIST_ENDPOINT: &str =
        "https://raw.githubusercontent.com/bitcoin/bips/master/bip-0039";

    

    /// Type alias for a tuple ([secp256k1::SecretKey], [secp256k1::PublicKey])
    pub type SecpKeypair = (SecretKey, PublicKey);
}