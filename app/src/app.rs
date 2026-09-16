use axum::{
    Router,
    routing::get,
};
use tower_http::trace::TraceLayer;
use crate::tester::{api_callback, spotify_login, };

pub fn app() -> Router {
    let default_routes = Router::new()
        .nest("/api", Router::new()
            .route("/nothing", get(|| async { "Hello, World!" })));

    let external_routes = Router::new()
        .nest("/ext_api/", Router::new()
            .route("/login", get(spotify_login))
            .route("/callback/:provider", get(api_callback)));
    Router::new()
        .route("/", get(|| async { "Hello, World!" }))
            .merge(default_routes)
            .merge(external_routes)
        .layer(TraceLayer::new_for_http())
}