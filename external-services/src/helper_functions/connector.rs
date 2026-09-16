/// https://joshleeb.com/posts/rust-traitobjects.html
use reqwest::{Client, Url};

pub struct ApiConnection {
    pub definition: Definition,
    pub auth_method: AuthMethod,

    pub scope: Scope,

    access_token: Option<String>,
    pub expires_at: Option<u64>,
}

pub struct Definition{
    pub id: i32,
    pub name: String,
    base_url: String,
    auth_url: String,
    redirect_url: String,
}

pub struct Scope{
    pub scopes: Vec<String>
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
    pub fn new(id: i32, name: String, base_url: String, auth_url: String, redirect_url: String
    ) -> Definition {
        Self{ id, name, base_url, auth_url, redirect_url, }
    }
}

impl ApiConnection {
    pub fn new(definition: Definition, auth_method: AuthMethod, scope: Scope,
    ) -> Self {
        let connector = Self {
            definition,
            auth_method,
            scope,
            access_token: None,
            expires_at: None,
        };

        connector
    }

    pub fn get_auth_url(&self
    ) -> String {
        let mut url = Url::parse(&*self.definition.auth_url).unwrap();

        match &self.auth_method {
            AuthMethod::OAuth2 { client_id, .. } => {
                url.query_pairs_mut()
                    .append_pair("scope", &self.scope.scopes.join(" "))
                    .append_pair("client_id", client_id)
                    .append_pair("response_type", "code")
                    .append_pair("redirect_uri", &self.definition.redirect_url);
            }
            AuthMethod::ApiKey { .. } => {}
            AuthMethod::Basic { .. } => {}
            AuthMethod::Bearer { .. } => {}
            AuthMethod::None => {}
        }

        url.as_str().to_string()
    }
}