/*
    Appellation: pzzld-sdk <library>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... Summary ...
*/
#[cfg(feature = "gateway")]
pub use pzzld_gateway as gateway;
#[cfg(feature = "payments")]
pub use pzzld_payments as payments;
#[cfg(feature = "wallets")]
pub use pzzld_wallets as wallets;

pub mod prelude {
    #[cfg(feature = "gateway")]
    pub use super::gateway::*;
    #[cfg(feature = "gateway")]
    pub use super::gateway::{config::*, middleware::*, states::*};
    #[cfg(feature = "payments")]
    pub use super::payments::*;
    #[cfg(feature = "wallets")]
    pub use super::wallets::*;
    pub use super::*;
}
