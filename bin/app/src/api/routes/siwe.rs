/*
   Appellation: siwe <module>
   Contrib: FL03 <jo3mccain@icloud.com>
   Description: ... Summary ...
*/
use axum::{
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

pub fn router() -> Router {
    Router::new()
        // .route("/auth", get(index))
        .route("/siwe/nonce", get(generate_nonce))
    // .route("/siwe/verify", post(validate_message))
}

pub async fn generate_nonce() -> Json<Value> {
    let nonce = siwe::generate_nonce();
    let payload = json!({ "nonce": nonce });

    Json(payload)
}

// pub async fn validate_message(message: siwe::Message, signature: Vec<u8>) -> Json<Value> {
//     let res = json!({"nonce": ""});

//     Json(res)
// }
