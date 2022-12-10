/*
    Appellation: shutdown <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... Summary ...
*/

pub async fn shutdown() {
    tracing::info!("Signal received; initiating shutdown procedures...");
    tokio::signal::ctrl_c()
        .await
        .expect("Expect shutdown signal handler");
}

#[async_trait::async_trait]
pub trait GracefulShutdown {
    async fn shutdown() {
        tokio::signal::ctrl_c().await.ok().unwrap()
    }
}
