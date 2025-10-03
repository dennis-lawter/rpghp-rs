#[allow(unused_imports)]
use crate::prelude::*;

use crate::domain::DomainError;
use crate::domain::records::init_group::InitGroupRecord;

use std::sync::Arc;

use poem::web::Data;
use poem_openapi::ApiResponse;
use poem_openapi::Object;
use poem_openapi::OpenApi;
use poem_openapi::param::Path;
use poem_openapi::payload::Json;

use crate::server::shared_state::SharedState;

use super::ApiV1AuthScheme;
use super::ApiV1AuthSchemeOptional;
use super::View;

pub struct ApiInitGroupRoutesV1;
#[OpenApi]
impl ApiInitGroupRoutesV1 {
    #[oai(path = "/session/:session_id/init-group", method = "post")]
    async fn create_init_group(
        &self,
        state: Data<&Arc<SharedState>>,
        session_id: Path<String>,
        data: Json<CreateInitGroupRequest>,
        auth: ApiV1AuthScheme,
    ) -> CreateInitGroupResponse {
        todo!()
    }
}

// Create

#[derive(serde::Deserialize, poem_openapi::Object)]
struct CreateInitGroupRequest {
    rank: i64,
}

#[derive(ApiResponse)]
enum CreateInitGroupResponse {
    #[oai(status = 201)]
    Created,
}
