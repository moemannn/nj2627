use axum::{
    Router,
    routing::get,
};
use tower_http::trace::TraceLayer;
use crate::routes::{app_routes, external_service_routes};

pub fn app() -> Router {
    Router::new()
        .merge(external_service_routes())
        .merge(app_routes())
        .layer(TraceLayer::new_for_http())
}
