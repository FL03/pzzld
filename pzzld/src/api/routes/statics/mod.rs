/*
   Appellation: statics <routes>
   Contrib: FL03 <jo3mccain@icloud.com>
   Description: ... Summary ...
*/
pub use self::wasm::*;

pub(crate) mod wasm;

use axum::{routing::get, Router};

pub fn router() -> Router {
    Router::new()
        .route("/pkg", get(file_handler))
}
