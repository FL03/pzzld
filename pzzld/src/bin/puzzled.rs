/*
    Appellation: template-rs-dioxide <app>
    Contrib: FL03 <jo3mccain@icloud.com> (https://github.com/FL03)
    Description: ... Summary ...
*/
use pzzld::{app, ApplicationScope};

fn main() -> anyhow::Result<()> {
    let scope = ApplicationScope::new().with_name("Puzzled");
    starter(scope)
}

fn starter(scope: ApplicationScope) -> anyhow::Result<()> {
    #[cfg(target_family = "wasm")]
    dioxus_web::launch_with_props(app, scope, dioxus_web::Config::new());
    #[cfg(any(macos, unix, windows))]
    dioxus_desktop::launch_with_props(app, scope, dioxus_desktop::Config::new());
    Ok(())
}
