/*
    Appellation: pzzld-cli <library>
    Created At: 2026.01.10:16:25:04
    Contrib: @FL03
*/
//! app-specific modules supporting the command line-interface for `arachnid`

// macros
#[macro_use]
pub(crate) mod macros {
    #[macro_use]
    mod fmt;
    #[macro_use]
    mod gsw;
}
// modules
pub mod api;
pub mod cli;
pub mod config;
pub mod context;
// re-exports
#[doc(inline)]
pub use self::{
    cli::{Cli, cli},
    config::Settings,
    context::Context,
};
// prelude
#[doc(hidden)]
pub mod prelude {
    pub use crate::cli::prelude::*;
    pub use crate::config::prelude::*;
    pub use crate::context::Context;
}
