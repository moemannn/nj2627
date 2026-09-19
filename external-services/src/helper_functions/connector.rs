use std::ops::Deref;
/// https://joshleeb.com/posts/rust-traitobjects.html
use reqwest::{Client, Url};




pub struct ApiConnection {
    pub definition: Definition,
    pub auth_method: AuthMethod,

    pub scope: Option<Vec<String>>,

    access_token: Option<String>,
    pub expires_at: Option<u64>,
}

pub struct Definition{
    pub name: String,
    base_url: Option<String>,
    auth_url: Option<String>,
    redirect_url: Option<String>,
}

pub enum AuthMethod {
    None,
    ApiKey {
        key: String,
    },
    Basic {
        username: String,
        password: String,
    },
    Bearer {
        token: String,
    },
    OAuth2 {
        client_id: String,
        client_secret: String,
    },
}

impl Definition {
    pub fn new(name: String, base_url: Option<String>, auth_url: Option<String>, redirect_url: Option<String>
    ) -> Definition {
        Self{ name, base_url, auth_url, redirect_url, }
    }
}

impl ApiConnection {
    pub fn new(
        definition: Definition,
        auth_method: AuthMethod,
        scope: Option<Vec<String>>,
    ) -> Self {
        Self {
            definition,
            auth_method,
            scope,
            access_token: None,
            expires_at: None,
        }
    }

    pub fn get_auth_url(&self,
    ) -> String {
        let url = Url::parse(self.definition.auth_url.as_deref().unwrap()).unwrap();

        match &self.auth_method {
            AuthMethod::OAuth2 { client_id, .. } => {
                self.oauth2_auth_method(client_id, url).to_string() }
            AuthMethod::ApiKey { .. } => {"".to_owned()}
            AuthMethod::Basic { .. } => {"".to_owned()}
            AuthMethod::Bearer { token } => {
                self.bearer_auth_method(token, url).to_string()}
            AuthMethod::None => {"".to_owned()}
        }
    }
    fn oauth2_auth_method(&self, client_id: &String, mut url: Url) -> Url {
        url.query_pairs_mut()
            .append_pair("scope", &self.scope.as_ref().unwrap().join(" "))
            .append_pair("client_id", client_id)
            .append_pair("response_type", "code")
            .append_pair("redirect_uri", &self.definition.redirect_url.as_ref().unwrap());
        url
    }

    fn bearer_auth_method(&self, token: &String, mut url: Url) -> Url {
        url.query_pairs_mut()
            .append_pair("Authorization", format!("Bearer {}", token).as_str())
            .append_pair("Accept", "application/vnd.github+json");
        url
    }
}