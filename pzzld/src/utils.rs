/*
    Appellation: utils <module>
    Contrib: FL03 <jo3mccain@icloud.com> (https://github.com/FL03)
    Description: ... Summary ...
*/
use crate::JsResult;
use gloo::net::http::{Request, Response, RequestRedirect};

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