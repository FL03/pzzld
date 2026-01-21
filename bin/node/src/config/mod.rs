/*
    Appellation: cnf <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
//! # Configuration
//!
//! This module implements the configuration settings for the platform. It is composed of several
//! sub-modules that define the various settings that can be configured.
#[doc(inline)]
pub use self::{consts::*, kinds::*, settings::*, types::*, utils::*};

mod consts;
mod settings;
mod utils;

pub mod kinds {
    #[doc(inline)]
    pub use self::{database::*, network::*, scope::*, services::*, tracing::*, workspace::*};

    mod database;
    mod network;
    mod scope;
    mod services;
    mod tracing;
    mod workspace;
}

pub mod types {
    #[doc(inline)]
    pub use self::{database_link::*, database_url::*, log_level::*, mode::*, netaddr::*};

    mod database_link;
    mod database_url;
    mod log_level;
    mod mode;
    mod netaddr;
}

pub(crate) mod prelude {
    pub use super::consts::*;
    pub use super::kinds::*;
    pub use super::settings::*;
    pub use super::types::*;
}
