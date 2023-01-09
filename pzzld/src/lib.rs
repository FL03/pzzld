/*
    Appellation: pzzld <library>
    Contrib: FL03 <jo3mccain@icloud.com> (https://github.com/FL03)
    Description: ... Summary ...
*/
use gloo::net::http::{Request, Response, RequestRedirect};
use wasm_bindgen::prelude::*;

pub mod api;


pub type JsResult<T = ()> = Result<T, JsError>;

pub async fn fetch(url: &str) -> JsResult<Response> {
    let res = request(url).send().await?;
    Ok(res)
}

pub fn request(url: &str) -> Request {
    Request::new(url)
}


pub async fn redirect(url: &str) -> JsResult {
    request(url).redirect(RequestRedirect::Follow).send().await?;
    Ok(())
}


