mod app;
mod tester;

use std::env;
use std::net::SocketAddr;

use app::app;

use axum::{
    Json,
    extract::ConnectInfo,
    response::Redirect,
};
use tower_http::trace::TraceLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        // .with_file(true)
        // .with_line_number(true)
        .init();

    match dotenvy::dotenv() {
        Ok(path) => tracing::info!("Loaded .env from {:?}", path),
        Err(err) => tracing::warn!("Failed to load .env file: {}", err),
    }

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();

    tracing::debug!("listening on {}", listener.local_addr().unwrap());

    axum::serve(listener, app()).await;
}

