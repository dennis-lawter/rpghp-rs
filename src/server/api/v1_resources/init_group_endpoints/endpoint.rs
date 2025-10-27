use poem_openapi::OpenApi;
use poem_openapi::param::Path;
use poem_openapi::payload::Json;

use super::super::auth::ApiAuthScheme;
use super::requests::CreateInitGroupRequest;
use crate::server::api::SharedStateCtx;
use crate::server::api::v1_resources::error_handling::FromDomainError;
use crate::server::api::v1_resources::init_group_endpoints::responses::CreateInitGroupResponse;
use crate::server::api::v1_resources::init_group_endpoints::views::InitGroupView;
use crate::server::api::view::FromEntity;

pub struct ApiInitGroupRoutesV1;
#[OpenApi]
impl ApiInitGroupRoutesV1 {
    #[oai(path = "/session/:session_id/init_group", method = "post")]
    async fn create_init_group(
        &self,
        state: SharedStateCtx<'_>,
        session_id: Path<String>,
        data: Json<CreateInitGroupRequest>,
        auth: ApiAuthScheme,
    ) -> CreateInitGroupResponse {
        match state
            .services
            .init_group
            .create_init_group(&session_id, &auth.token(), data.rank)
            .await
        {
            Ok(init_group) => {
                CreateInitGroupResponse::Created(Json(InitGroupView::from_entity(&init_group)))
            }
            Err(err) => CreateInitGroupResponse::from_domain_error(&err),
        }
    }
}
