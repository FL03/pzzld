/*
    Appellation: primitives <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... Summary ...
*/
pub use self::{constants::*, types::*};

pub(crate) mod constants {
    ///
    pub const LOCALHOST: [u8; 4] = [127, 0, 0, 1];
}

pub(crate) mod types {
    ///
    pub type AsyncError = Box<dyn std::error::Error + Send + Sync + 'static>;
    ///
    pub type BoxError = Box<dyn std::error::Error>;
}
