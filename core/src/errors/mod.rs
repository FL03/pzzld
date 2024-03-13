/*
    Appellation: errors <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
//! # errors
//!
//!

///
pub type BoxError = Box<dyn std::error::Error + Send + Sync>;
///
pub type BoxResult<T = ()> = std::result::Result<T, BoxError>;
