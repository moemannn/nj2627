use axum::Router;
use axum::routing::{get, Route};
use crate::services::{external_service_callback, spotify_login};

pub fn external_service_routes() -> Router {
    Router::new()
            .nest("/external-api", Router::new()
                .merge(spotify_routes())
                .route("/callback/:provider", get(external_service_callback)))
}

fn spotify_routes() -> Router {
    Router::new()
        .route("/login", get(spotify_login))
}