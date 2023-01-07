/*
    Appellation: pzzld <library>
    Contrib: FL03 <jo3mccain@icloud.com> (https://github.com/FL03)
    Description: ... Summary ...
*/
use gloo::net::http::{Request, Response};
use wasm_bindgen::prelude::*;

pub mod api;


pub type JsResult<T = ()> = Result<T, JsError>;

pub async fn fetch(req: Request) -> JsResult<Response> {
    let res = req.send().await?;
    Ok(res)
}

