/*
    Appellation: xtask <binary>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... Summary ...
*/
pub use self::utils::*;

pub(crate) mod utils;

pub mod cli;

use clap::{arg, command, value_parser, Arg, Command};
use std::path::PathBuf;

fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();
    tracing::info!("Welcome to xtask...");
    cli()?;

    let handle = std::thread::spawn(move || {
        cli::handle().join().unwrap();
    });
    handle.join().ok().unwrap();

    Ok(())
}

pub fn cli() -> anyhow::Result<()> {
    let matches = command!()
        .propagate_version(true)
        .subcommand_required(false)
        .arg_required_else_help(true)
        .subcommand(
            Command::new("command")
                .about("Various integrations")
                .arg(
                    arg!(--nix [NIX] "Interact with the nix package manager")
            ),
        )
        .arg(
            arg!(
                -c --config <FILE> "Sets a custom config file"
            )
            // We don't have syntax yet for optional options, so manually calling `required`
            .required(false)
            .value_parser(value_parser!(PathBuf)),
        )
        .arg(arg!(
            -d --debug ... "Turn debugging information on"
        ))
        .arg(
            arg!(-p --port <PORT>)
                .help("Network port to use")
                .value_parser(value_parser!(u16).range(1..)).default_value("8080"),
        )
        .get_matches();
    
    let port: u16 = *matches
        .get_one::<u16>("PORT")
        .unwrap();
    
    println!("{:?}", port);
    Ok(())
}

///
pub type Bundle<T = String> = std::collections::HashMap<T, Vec<Vec<T>>>;

///
#[macro_export]
macro_rules! cmd {
    ($(
        $x:expr;
        [ $( $y:expr ),* ]
    );*) => {
        {
            $(
                let mut cmd = std::process::Command::new($x);
                cmd.current_dir(scsys_xtask::project_root());
                let mut tmp = Vec::new();
                $(
                    tmp.push($y);
                )*
                cmd.args(tmp.as_slice()).status().expect("");
            )*
        }
    };
}