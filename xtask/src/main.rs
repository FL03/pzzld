use std::process::Command;
use xtask_wasm::{anyhow::Result, clap, default_dist_dir};

#[derive(clap::Parser)]
enum Opt {
    Dist(xtask_wasm::Dist),
    Watch(xtask_wasm::Watch),
    Start(xtask_wasm::DevServer),
}


fn main() -> Result<()> {
    let opt: Opt = clap::Parser::parse();


    match opt {
        Opt::Dist(dist) => {
            log::info!("Generating package...");

            dist
                .dist_dir_path("dist")
                .static_dir_path("pzzld/static")
                .app_name("pzzld")
                .run_in_workspace(true)
                .run("pzzld")?;
        }
        Opt::Watch(watch) => {
            log::info!("Watching for changes and check...");

            let mut command = Command::new("cargo");
            command.arg("check");

            watch.run(command)?;
        }
        Opt::Start(mut dev_server) => {
            log::info!("Starting the development server...");

            dev_server.arg("dist").start(default_dist_dir(false))?;
        }
    }

    Ok(())
}