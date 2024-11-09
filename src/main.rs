mod config;
mod errors;
mod ping;
mod templates;

use anyhow::Result;
use askama_axum::IntoResponse;
use axum::{
    extract::{Query, State},
    http::{HeaderMap, Method, StatusCode},
    response::Response,
    routing::get,
    Json, Router,
};
use config::Config;
use env_logger::Env;
use errors::ErrorResponse;
use log::info;
use ping::ping;
use std::{collections::HashMap, env};
use templates::{Index, Server, ServerError};
use tower_http::{
    cors::{Any, CorsLayer},
    services::ServeDir,
};

async fn index(State(config): State<Config>) -> Result<Index, ErrorResponse> {
    Ok(config.into())
}

async fn server(
    Query(params): Query<HashMap<String, String>>,
    headers: HeaderMap,
) -> Result<Response, ServerError> {
    let host = params
        .get("host")
        .ok_or_else(|| anyhow::anyhow!("host must be specified"))?;
    let port = params.get("port").map(|p| p.parse()).transpose()?;
    let resp = ping(host, port).await?;

    let accept = headers.get("accept").map(|s| s.to_str()).transpose()?;

    Ok(match accept {
        Some("application/json") => Json(resp).into_response(),
        Some("text/html" | "*/*") | None => Server::from(resp).into_response(),
        _ => ErrorResponse::new(
            StatusCode::BAD_REQUEST,
            "unsupported requested content type",
        )
        .into_response(),
    })
}

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    let cfgfile = env::args().nth(1).unwrap_or_else(|| "config.toml".into());
    let cfg = Config::parse(cfgfile)?;

    let mut app = Router::new()
        .route("/", get(index))
        .route("/server", get(server))
        .with_state(cfg.clone())
        .nest_service("/static/css", ServeDir::new("css"));

    if let Some(ref cors) = cfg.cors {
        let cors_layer = CorsLayer::new()
            .allow_headers(Any)
            .allow_methods([Method::GET])
            .allow_origin(
                cors.allowed_origins
                    .iter()
                    .map(|v| v.parse())
                    .collect::<Result<Vec<_>, _>>()?,
            );

        app = app.layer(cors_layer);
    }

    let addr = cfg.address.unwrap_or("127.0.0.1:8080".into());
    let listener = tokio::net::TcpListener::bind(&addr).await?;

    info!("Listening on {addr} ...");
    axum::serve(listener, app).await?;

    Ok(())
}
