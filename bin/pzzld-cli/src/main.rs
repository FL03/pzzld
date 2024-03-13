/*
    Appellation: pzzld-cli <binary>
    Contrib: FL03 <jo3mccain@icloud.com> (https://github.com/FL03)
*/

pub mod cli;

use pzzld::prelude::BoxResult;

fn main() -> BoxResult {
    let cli = cli::new();

    println!("{}", &cli);
    Ok(())
}
