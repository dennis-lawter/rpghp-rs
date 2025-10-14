use poem_openapi::SecurityScheme;
use poem_openapi::auth::Bearer;

#[derive(SecurityScheme)]
#[oai(ty = "bearer", key_name = "Bearer", key_in = "header")]
pub(super) struct ApiV1AuthScheme(pub(super) Bearer);

#[derive(SecurityScheme)]
pub(super) enum ApiV1AuthSchemeOptional {
    Bearer(ApiV1AuthScheme),
    #[oai(fallback)]
    NoAuth,
}
