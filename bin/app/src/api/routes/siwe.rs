/*
   Appellation: siwe <module>
   Contrib: FL03 <jo3mccain@icloud.com>
   Description: ... Summary ...
*/
use axum::{
    extract::{self, Extension, Query},
    routing::{get, post},
    Json, Router,
};
use scsys::Timestamp;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

pub fn router() -> Router {
    Router::new()
        .route("/siwe/nonce", get(generate_nonce))
        // .route("/siwe/verify/", post(validate_message))
}

pub async fn generate_nonce() -> Json<Value> {
    let nonce = siwe::generate_nonce();
    let payload = json!({ "nonce": nonce });

    Json(payload)
}

#[derive(Clone, Debug, Default, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct SiweMessage {
    pub domain: String,
    pub address: [u8; 20],
    pub statement: Option<String>,
    pub uri: String,
    pub version: String,
    pub chain_id: u64,
    pub nonce: String,
    pub issued_at: Timestamp,
    pub expiration_time: Option<Timestamp>,
    pub not_before: Option<Timestamp>,
    pub request_id: Option<String>,
    pub resources: Vec<Timestamp>,
}

#[derive(Clone, Debug, Default, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct MessageValidation {
    pub message: SiweMessage,
    pub signature: Vec<u8>
}

pub async fn validate_message(extract::Json(message): extract::Json<MessageValidation>) -> bool {
    let res = json!({"nonce": "", "message": message});

    true
}
