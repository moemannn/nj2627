mod app;
mod services;
mod routes;
use std::env;
use std::net::SocketAddr;

use app::app;

use axum::{extract::ConnectInfo, response::Redirect, Json, Router};
use tower_http::trace::TraceLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() {
    load_env_files();

    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        // .with_file(true)
        // .with_line_number(true)
        .init();

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();

    tracing::debug!("listening on {}", listener.local_addr().unwrap());

    axum::serve(listener, app()).await;
}

fn load_env_files(){
    match dotenvy::from_filename(".env.auth_methods") {
        Ok(path) => tracing::info!("Loaded .auth_methods from {:?}", path),
        Err(err) => tracing::warn!("Failed to load .auth_methods file: {}", err),
    }

    match dotenvy::dotenv(){
        Ok(path) => tracing::info!("Loaded .env from {:?}", path),
        Err(err) => tracing::warn!("Failed to load .env file: {}", err),
    }
}