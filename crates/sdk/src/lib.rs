/*
    Appellation: pzzld-sdk <library>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... Summary ...
*/
#[cfg(feature = "wallets")]
pub use pzzld_wallets as wallets;

pub mod prelude {
    #[cfg(feature = "wallets")]
    pub use super::wallets::*;
    pub use super::*;
}
