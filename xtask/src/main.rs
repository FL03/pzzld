/*
    Appellation: xtask <binary>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... Summary ...
*/
pub use self::{commands::*, utils::*};

pub(crate) mod commands;
pub(crate) mod utils;

use axum::{
    body::{boxed, Body, BoxBody},
    http::{Request, Response, StatusCode, Uri},
    response::IntoResponse,
    routing::get_service
};
use clap::{arg, command, value_parser, ArgAction, ArgMatches, Command};
use std::path::PathBuf;
use tower::ServiceExt;
use tower_http::services::ServeDir;

#[tokio::main(flavor = "current_thread")]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();
    tracing::info!("Welcome to xtask...");
    wasm_server().await?;

    cli()?;

    Ok(())
}

pub fn base_matches(sc: Command) -> ArgMatches {
    command!()
        .arg(
            arg!(
                config: -c --config <FILE> "Sets a custom config file"
            )
            // We don't have syntax yet for optional options, so manually calling `required`
            .required(false)
            .value_parser(value_parser!(PathBuf)),
        )
        .arg(
            arg!(debug:
                -d --debug ... "Turn debugging information on"
            )
            .action(ArgAction::SetTrue),
        )
        .arg(
            arg!(port: -p --port <PORT>)
                .help("Network port to use")
                .value_parser(value_parser!(u16).range(1..))
                .default_value("8080"),
        )
        .arg_required_else_help(true)
        .propagate_version(true)
        .subcommand(sc)
        .subcommand_required(false)
        .get_matches()
}

pub fn cli() -> anyhow::Result<()> {
    let matches = base_matches(
        Command::new("actor")
            .about("Various integrations")
            .arg(arg!(build: -b --build <BUILD> "Build the workspace"))
            .arg(arg!(build: -b --build <BUILD> "Build the workspace"))
    );

    let port: u16 = *matches.get_one::<u16>("PORT").unwrap();

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

pub async fn wasm_server() -> anyhow::Result<()> {
    let serve_dir = get_service(ServeDir::new("/pkg")).handle_error(handle_error);
    let app = axum::Router::new()
        .nest_service("/", serve_dir);
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
    Ok(())
}

async fn handle_error(_err: std::io::Error) -> impl IntoResponse {
    (StatusCode::INTERNAL_SERVER_ERROR, "Something went wrong...")
}

pub async fn file_handler(uri: Uri) -> Result<Response<BoxBody>, (StatusCode, String)> {
    let res = get_static_file(uri.clone()).await?;
    println!("{:?}", res);

    if res.status() == StatusCode::NOT_FOUND {
        // try with `.html`
        // TODO: handle if the Uri has query parameters
        match format!("{}.html", uri).parse() {
            Ok(uri_html) => get_static_file(uri_html).await,
            Err(_) => Err((StatusCode::INTERNAL_SERVER_ERROR, "Invalid URI".to_string())),
        }
    } else {
        Ok(res)
    }
}

async fn get_static_file(uri: Uri) -> Result<Response<BoxBody>, (StatusCode, String)> {
    let req = Request::builder().uri(uri).body(Body::empty()).unwrap();

    // `ServeDir` implements `tower::Service` so we can call it with `tower::ServiceExt::oneshot`
    // When run normally, the root is the workspace root
    match ServeDir::new("../pzzld").oneshot(req).await {
        Ok(res) => Ok(res.map(boxed)),
        Err(err) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Something went wrong: {}", err),
        )),
    }
}