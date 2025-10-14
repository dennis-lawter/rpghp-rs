use std::sync::Arc;

use poem::EndpointExt;
use poem::IntoEndpoint;
use poem::Route;
use poem::middleware::AddDataEndpoint;
use poem_openapi::ContactObject;
use poem_openapi::OpenApiService;
use v1_resources::creature_endpoints::ApiCreatureRoutesV1;
use v1_resources::session_endpoints::ApiSessionRoutesV1;

use super::shared_state::SharedState;
use crate::config::Config;

mod v1_resources;
mod view;

pub struct Api;

impl Api {
    pub fn create_route(
        cfg: &Config,
        shared_state: Arc<SharedState>,
    ) -> AddDataEndpoint<Route, Arc<SharedState>> {
        let v1 = Self::build_v1_service(cfg);
        let docs = v1.rapidoc();

        Route::new()
            .nest("/v1", v1.into_endpoint())
            .nest("/docs/v1", docs)
            .data(shared_state)
    }

    fn build_v1_service(
        cfg: &Config
    ) -> OpenApiService<(ApiSessionRoutesV1, ApiCreatureRoutesV1), ()> {
        let v1_endpoints = (ApiSessionRoutesV1, ApiCreatureRoutesV1);

        OpenApiService::new(v1_endpoints, "RPGHP API", "1.0")
            .server("/api/v1")
            .contact(Self::contact_info(cfg))
            .description("API backend for the RPGHP application.")
    }

    fn contact_info(cfg: &Config) -> ContactObject {
        ContactObject::new()
            .email(cfg.contact_email.clone())
            .name(cfg.contact_name.clone())
    }
}
