use std::env;
use crate::{Definition, ApiConnection, AuthMethod};

pub async fn spotify_configuration() -> ApiConnection {
    ApiConnection::new(
        Definition::new(
            "Spotify".to_string(),
            Some("https://accounts.spotify.com/api/".to_string()),
            Some("https://accounts.spotify.com/authorize".to_string()),
            Some("http://127.0.0.1:3000/external-api/callback/spotify".to_string()),
        ),
        AuthMethod::OAuth2 {
            client_id: env::var("SPOTIFY_CLIENT_ID").unwrap(),
            client_secret: env::var("SPOTIFY_CLIENT_SECRET").unwrap(),
        },
        Some(vec![
            "user-read-currently-playing".parse().unwrap(),
            "user-read-playback-state".parse().unwrap(),
            "user-modify-playback-state".parse().unwrap(),
        ])
    )
}

