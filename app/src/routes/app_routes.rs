use axum::Router;
use axum::routing::get;

pub fn app_routes() -> Router {
    Router::new()
        .nest("/api", Router::new()
            .route("/nothing", get(|| async { "Hello, World!" })))
}