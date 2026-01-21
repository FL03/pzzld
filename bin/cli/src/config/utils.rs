/*
    Appellation: utils <module>
    Created At: 2026.01.11:07:21:03
    Contrib: @FL03
*/
use crate::config::LogLevel;

pub fn fmt_as_env_filter(level: LogLevel, name: &str) -> String {
    format!("{name}={level},tower_http={level}")
}

pub fn init_tracing(level: LogLevel, name: &str) {
    use tracing_subscriber::{filter::EnvFilter, util::SubscriberInitExt};

    let filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| format!("{name}={level},tower_http={level}").into());

    tracing_subscriber::fmt()
        .compact()
        .with_ansi(true)
        .with_env_filter(filter)
        .with_max_level(level.as_tracing_level())
        .with_target(true)
        .with_timer(tracing_subscriber::fmt::time::uptime())
        .finish()
        .init();
    tracing::debug!("Successfully initialized tracing with level: {level}");
}

/// Initialize the tracer with the given name
fn _init_tracing(config: &super::TracingConfig, name: &str) {
    use tracing_subscriber::{filter::EnvFilter, layer::SubscriberExt, util::SubscriberInitExt};

    let filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| {
        format!("{name}={level},tower_http={level}", level = config.level).into()
    });

    let layer = tracing_subscriber::fmt::layer()
        .compact()
        .with_ansi(true)
        .with_level(true)
        .with_line_number(false)
        .with_target(false)
        .with_thread_ids(false)
        .with_thread_names(false)
        .with_timer(tracing_subscriber::fmt::time::uptime());

    tracing_subscriber::registry()
        .with(filter)
        .with(layer)
        .init();
    tracing::trace!("Initialized tracing modules...");
}
