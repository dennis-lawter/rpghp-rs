use crate::prelude::*;

use super::api_shared_state::ApiSharedState;
use super::resources::creature::ApiCreatureRoutesV1;
use super::resources::session::ApiSessionRoutesV1;
use poem::EndpointExt;
use poem::IntoEndpoint;
use poem::Route;
use poem::middleware::AddDataEndpoint;
use poem_openapi::ContactObject;
use poem_openapi::OpenApiService;

use crate::config::Config;

pub struct Api;
impl Api {
    pub async fn create_route(cfg: &Config) -> CrateResult<AddDataEndpoint<Route, ApiSharedState>> {
        let api_shared_state = ApiSharedState::new(cfg).await?;
        let v1_endpoints = (ApiSessionRoutesV1, ApiCreatureRoutesV1);
        let v1 = OpenApiService::new(v1_endpoints, "RPGHP API", "1.0")
            .server("/api/v1")
            .contact(
                ContactObject::new()
                    .email("bytomancer@gmail.com")
                    .name("Bytomancer"),
            )
            .description("API backend for the RPGHP application.");
        let docs = v1.redoc();
        Ok(Route::new()
            .nest("/v1", v1.into_endpoint())
            .nest("/docs/v1", docs)
            .data(api_shared_state))
    }
}
