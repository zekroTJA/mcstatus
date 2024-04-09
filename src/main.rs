mod errors;
mod ping;
mod templates;

use anyhow::Result;
use axum::{routing::get, Router};
use env_logger::Env;
use errors::ErrorResponse;
use ping::ping;
use templates::Index;
use tower_http::services::ServeDir;

async fn index() -> Result<Index, ErrorResponse> {
    dbg!(ping("mc.zekro.de").await?.sample);
    Ok(Index {})
}

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    let app = Router::new()
        .route("/", get(index))
        .nest_service("/static/css", ServeDir::new("css"));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:8080").await?;
    axum::serve(listener, app).await?;

    Ok(())
}
