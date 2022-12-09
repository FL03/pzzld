/*
    Appellation: pzzld <library>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... Summary ...
*/
#[cfg(feature = "core")]
pub use pzzld_core as core;
#[cfg(feature = "wallets")]
pub use pzzld_wallets as wallets;


pub mod prelude {
    #[cfg(feature = "core")]
    pub use super::core::*;
    #[cfg(feature = "wallets")]
    pub use super::wallets::*;
}