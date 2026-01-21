/*
    Appellation: cli <module>
    Created At: 2026.01.10:15:57:39
    Contrib: @FL03
*/
//! this module defines the command line interface for the application
#[doc(inline)]
pub use self::{cli::*, cmd::*};

mod cli;

pub mod args {
    // #[doc(inline)]
    // pub use self::*;
}

pub mod cmd;

#[doc(hidden)]
pub mod prelude {
    pub use super::cli::Cli;
    pub use super::cmd::Command;
}
/// a helper function used to load, or parse, the command line interface from the arguments
pub fn cli() -> Cli {
    Cli::parse()
}
