/*
    Appellation: pzzld <library>
    Contrib: FL03 <jo3mccain@icloud.com> (https://github.com/FL03)
    Description: ... Summary ...
*/
pub use self::{primitives::*, utils::*};

pub(crate) mod utils;

pub mod api;



pub(crate) mod primitives {
    use wasm_bindgen::prelude::JsError;

    /// Type alias for a [Result] of type T and error [JsError]
    pub type JsResult<T = ()> = Result<T, JsError>;
}
