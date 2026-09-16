use axum::response::Redirect;
use axum::extract::{Path, Query};
use std::collections::HashMap;

use external_services::spotify_configuration;

// http://127.0.0.1:3000/ext_api/login

pub async fn spotify_login() -> Redirect {
    let connection = spotify_configuration().await;
    Redirect::to(&*connection.get_auth_url())
}

pub async fn api_callback(
    Path(provider): Path<String>,
    Query(params): Query<HashMap<String, String>>,
){
    let code= params.get("code");

    tracing::info!("PROVIDER CALLBACK: {}", provider);
    tracing::info!("PROVIDER CODE: {}", code.unwrap());


}
