/*
    Appellation: auth <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... Summary ...
*/

pub mod oauth;
pub mod siwe;

pub fn oauth(ctx: crate::Context) -> axum::Router {
    oauth::router(ctx)
}

pub fn siwe() -> axum::Router {
    siwe::router()
}
