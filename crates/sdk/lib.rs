/*
    Appellation: lib <module>
    Created At: 2026.01.20:14:52:56
    Contrib: @FL03
*/
//! the core modules supporting the `contained` crate focused on establishing foundational
//! primitives and utilities for getter, setters, and wrappers.
#![crate_type = "lib"]
#![allow(
    clippy::missing_docs_in_private_items,
    clippy::missing_errors_doc,
    clippy::missing_safety_doc,
    clippy::module_inception,
    clippy::needless_doctest_main,
    clippy::upper_case_acronyms
)]
#![cfg_attr(not(feature = "std"), no_std)]
#![cfg_attr(all(feature = "alloc", feature = "nightly"), feature(allocator_api))]
// compilation checks
#[cfg(not(any(feature = "std", feature = "alloc")))]
compile_error! { "Either the 'alloc' feature or the 'std' feature must be enabled for this crate." }
// external crates
#[cfg(feature = "alloc")]
extern crate alloc;
// re-exports
pub use pzzld_core::*;
// prelude
#[doc(hidden)]
pub mod prelude {
    pub use pzzld_core::prelude::*;
}
