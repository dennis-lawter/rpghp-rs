use poem_openapi::SecurityScheme;
use poem_openapi::auth::Bearer;

#[derive(SecurityScheme)]
#[oai(ty = "bearer", key_name = "Bearer", key_in = "header")]
pub struct ApiAuthScheme(Bearer);
impl ApiAuthScheme {
    pub fn token(&self) -> String {
        self.0.token.clone()
    }
}

#[derive(SecurityScheme)]
pub enum ApiOptAuthScheme {
    Bearer(ApiAuthScheme),
    #[oai(fallback)]
    NoAuth,
}
impl ApiOptAuthScheme {
    pub fn opt_token(&self) -> Option<String> {
        match self {
            Self::Bearer(bearer_auth) => Some(bearer_auth.0.token.clone()),
            Self::NoAuth => None,
        }
    }

    pub const fn auth_provided(&self) -> bool {
        match self {
            ApiOptAuthScheme::Bearer(_) => true,
            ApiOptAuthScheme::NoAuth => false,
        }
    }
}
