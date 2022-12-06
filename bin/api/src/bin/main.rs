/*
    Appellation: pzzld-api <binary>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... Summary ...
*/

#[tokio::main]
async fn main() -> scsys::AsyncResult {
    pzzld_api::Api::default().serve().await?;

    Ok(())
}