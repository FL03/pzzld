/*
    Appellation: fluidity-core <module>
    Creator: FL03 <jo3mccain@icloud.com>
    Description:
        ... Summary ...
*/
#[doc(inline)]
pub use self::{primitives::*, utils::*};

pub(crate) mod primitives;
pub(crate) mod utils;

pub mod assets;
pub mod authorize;
pub mod credentials;
pub mod mnemonics;
pub mod wallets;
