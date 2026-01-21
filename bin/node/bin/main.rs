/*
    Appellation: pzzld-node <binary>
    Created At: 2026.01.10:16:11:29
    Contrib: @FL03
*/
use pzzld_node::{Cli, Context, Settings};

#[cfg_attr(feature = "multi-thread", tokio::main)]
#[cfg_attr(not(feature = "multi-thread"), tokio::main(flavor = "current_thread"))]
async fn main() -> anyhow::Result<()> {
    // build the settings
    let settings = Settings::build()?;
    // initialize the tracing subsystem
    settings.init_tracing();
    tracing::debug! { "Platform settings: {}", settings }
    // initialize the context with the settings
    let mut context = Context::from_config(settings);
    // parse the command line arguments
    let cli = Cli::parse();
    tracing::debug! { "Parsed CLI arguments: {:?}", cli }
    // handle the CLI commands
    cli.handle(&mut context).await
}
