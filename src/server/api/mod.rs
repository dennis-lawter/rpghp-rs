use action::session_routes::ApiSessionRoutesV1;
use poem::IntoEndpoint;
use poem::Route;
use poem_openapi::ContactObject;
use poem_openapi::OpenApi;
use poem_openapi::OpenApiService;

mod action;
mod domain;
mod render;
pub mod shared_state;

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
    pub fn create_route() -> Route {
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
        Route::new()
            .nest("/v1", v1.into_endpoint())
            .nest("/docs/v1", rapidoc)
    }
}
