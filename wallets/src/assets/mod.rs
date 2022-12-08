/*
    Appellation: assets <module>
    Creator: FL03 <jo3mccain@icloud.com>
    Description:
        ... Summary ...
*/
pub use digital::*;

mod asset;
mod digital;

/// Defines the minimum framework necessary for the system to recognize a currency
pub trait Currency {
    fn name(&self) -> String
    where
        Self: Sized;
    fn symbol(&self) -> String
    where
        Self: Sized;
}

/// Describes the two versions of currencies
#[derive(Clone, Debug, Hash, PartialEq, serde::Deserialize, serde::Serialize)]
pub enum CurrencyType<C = Box<dyn Currency>> {
    Fiat(C),
    Token(C),
}
