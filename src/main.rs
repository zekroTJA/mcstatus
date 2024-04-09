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
use ping::ping;
use templates::{Index, Server};
use tower_http::services::ServeDir;

async fn index(State(config): State<Config>) -> Result<Index, ErrorResponse> {
    Ok(config.into())
}

async fn server(Query(params): Query<HashMap<String, String>>) -> Result<Server, ErrorResponse> {
    let host = params.get("host").unwrap();
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
        .with_state(cfg)
        .nest_service("/static/css", ServeDir::new("css"));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:8080").await?;
    axum::serve(listener, app).await?;

    Ok(())
}
