/*
    Appellation: pzzld <library>
    Contrib: FL03 <jo3mccain@icloud.com> (https://github.com/FL03)
    Description: ... Summary ...
*/
pub use self::{app::*, primitives::*, utils::*};

pub(crate) mod app;
pub(crate) mod utils;

pub mod actors;
pub mod components;

pub(crate) mod primitives {
    use wasm_bindgen::prelude::JsError;

    /// Type alias for a [Result] of type T and error [JsError]
    pub type JsResult<T = ()> = Result<T, JsError>;
}
