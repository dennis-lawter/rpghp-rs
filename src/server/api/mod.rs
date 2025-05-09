#[allow(unused_imports)]
use crate::prelude::*;

use std::sync::Arc;

use super::shared_state::SharedState;
use poem::EndpointExt;
use poem::IntoEndpoint;
use poem::Route;
use poem::middleware::AddDataEndpoint;
use poem_openapi::ContactObject;
use poem_openapi::OpenApiService;
use resources::creature::ApiCreatureRoutesV1;
use resources::session::ApiSessionRoutesV1;

use crate::config::Config;

mod resources;

pub struct Api;
impl Api {
    pub async fn create_route(
        cfg: &Config,
        shared_state: Arc<SharedState>,
    ) -> CrateResult<AddDataEndpoint<Route, Arc<SharedState>>> {
        let v1_endpoints = (ApiSessionRoutesV1, ApiCreatureRoutesV1);
        let v1 = OpenApiService::new(v1_endpoints, "RPGHP API", "1.0")
            .server("/api/v1")
            .contact(
                ContactObject::new()
                    .email(cfg.contact_email.clone())
                    .name(cfg.contact_name.clone()),
            )
            .description("API backend for the RPGHP application.");
        let docs = v1.rapidoc();
        Ok(Route::new()
            .nest("/v1", v1.into_endpoint())
            .nest("/docs/v1", docs)
            .data(shared_state))
    }
}
