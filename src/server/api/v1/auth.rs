use poem_openapi::SecurityScheme;
use poem_openapi::auth::Bearer;

#[derive(SecurityScheme)]
#[oai(ty = "bearer", key_name = "Bearer", key_in = "header")]
pub struct ApiV1AuthScheme(Bearer);
impl ApiV1AuthScheme {
    pub fn token(&self) -> String {
        self.0.token.clone()
    }
}

#[derive(SecurityScheme)]
pub enum ApiV1AuthSchemeOptional {
    Bearer(ApiV1AuthScheme),
    #[oai(fallback)]
    NoAuth,
}
impl ApiV1AuthSchemeOptional {
    pub fn opt_token(&self) -> Option<String> {
        match self {
            Self::Bearer(bearer_auth) => Some(bearer_auth.0.token.clone()),
            Self::NoAuth => None,
        }
    }

    pub const fn auth_provided(&self) -> bool {
        match self {
            ApiV1AuthSchemeOptional::Bearer(_) => true,
            ApiV1AuthSchemeOptional::NoAuth => false,
        }
    }
}
