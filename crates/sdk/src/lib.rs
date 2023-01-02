/*
    Appellation: pzzld-sdk <library>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... Summary ...
*/
#[cfg(feature = "gateways")]
pub use pzzld_gateways as gateways;
#[cfg(feature = "wallets")]
pub use pzzld_wallets as wallets;

pub mod prelude {
    #[cfg(feature = "gateways")]
    pub use super::gateways::*;
    #[cfg(feature = "gateways")]
    pub use super::gateways::{api::*, config::*, middleware::*, states::*};
    #[cfg(feature = "wallets")]
    pub use super::wallets::*;
    pub use super::*;
}
