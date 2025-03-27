use crate::prelude::*;

use action::session_routes::ApiSessionRoutesV1;
use api_shared_state::ApiSharedState;
use poem::EndpointExt;
use poem::IntoEndpoint;
use poem::Route;
use poem::middleware::AddDataEndpoint;
use poem_openapi::ContactObject;
use poem_openapi::OpenApi;
use poem_openapi::OpenApiService;

use crate::config::Config;

mod action;
mod api_shared_state;
mod domain;
mod render;

struct ApiHelloWorldV1;
#[OpenApi]
impl ApiHelloWorldV1 {
    #[oai(path = "/hello", method = "get")]
    async fn test(&self) -> poem_openapi::payload::PlainText<String> {
        poem_openapi::payload::PlainText("Hello World".to_owned())
    }
}

pub struct Api;
impl Api {
    pub async fn create_route(cfg: &Config) -> CrateResult<AddDataEndpoint<Route, ApiSharedState>> {
        let api_shared_state = ApiSharedState::new(cfg).await?;
        let v1_endpoints = (ApiHelloWorldV1, ApiSessionRoutesV1);
        let v1 = OpenApiService::new(v1_endpoints, "RPGHP API", "1.0")
            .server("/v1")
            .contact(
                ContactObject::new()
                    .email("bytomancer@gmail.com")
                    .name("Bytomancer"),
            )
            .description("API backend for the RPGHP application.");
        let rapidoc = v1.rapidoc();
        Ok(Route::new()
            .nest("/v1", v1.into_endpoint())
            .nest("/docs/v1", rapidoc)
            .data(api_shared_state))
    }
}
