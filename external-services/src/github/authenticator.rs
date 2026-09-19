use std::env;
use crate::{Definition, ApiConnection, AuthMethod};


pub async fn configuration() -> ApiConnection {
    ApiConnection::new(
        Definition::new("Github".to_string(), None, None, None),
        AuthMethod::Bearer {
            token: env::var("GIT_HUB_PERSONAAL_ACCESS_TOKEN").unwrap(),
        }, None
    )
}
