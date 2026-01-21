/*
    Appellation: basic <example>
    Created At: 2026.01.21:10:04:38
    Contrib: @FL03
*/

fn main() -> pzzld::Result<()> {
    // setup tracing
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .with_max_level(tracing::Level::TRACE)
        .with_target(false)
        .with_timer(tracing_subscriber::fmt::time::uptime())
        .init();
    // log a greeting
    tracing::info!("Hello, Pzzld SDK!");

    Ok(())
}
