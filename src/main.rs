mod config;
mod errors;
mod ping;
mod templates;

use std::{collections::HashMap, env};

use anyhow::Result;
use axum::{
    extract::{Query, State},
    routing::get,
    Router,
};
use config::Config;
use env_logger::Env;
use errors::ErrorResponse;
use log::info;
use ping::ping;
use templates::{Index, Server, ServerError};
use tower_http::services::ServeDir;

async fn index(State(config): State<Config>) -> Result<Index, ErrorResponse> {
    Ok(config.into())
}

async fn server(Query(params): Query<HashMap<String, String>>) -> Result<Server, ServerError> {
    let host = params
        .get("host")
        .ok_or_else(|| anyhow::anyhow!("host must be specified"))?;
    let port = params.get("port").map(|p| p.parse()).transpose()?;
    let resp = ping(host, port).await?;
    Ok(resp.into())
}

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    let cfgfile = env::args().nth(1).unwrap_or_else(|| "config.toml".into());
    let cfg = Config::parse(cfgfile)?;

    let app = Router::new()
        .route("/", get(index))
        .route("/server", get(server))
        .with_state(cfg.clone())
        .nest_service("/static/css", ServeDir::new("css"));

    let addr = cfg.address.unwrap_or("127.0.0.1:8080".into());
    let listener = tokio::net::TcpListener::bind(&addr).await?;

    info!("Listening on {addr} ...");
    axum::serve(listener, app).await?;

    Ok(())
}
