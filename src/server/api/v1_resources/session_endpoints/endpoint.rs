use poem_openapi::OpenApi;
use poem_openapi::param::Path;
use poem_openapi::payload::Json;

use super::super::auth::ApiV1AuthScheme;
use super::responses::SessionCreateResponse;
use super::responses::SessionDeleteResponse;
use super::responses::SessionGetResponse;
use super::views::SessionView;
use super::views::SessionWithSecretView;
use crate::server::api::SharedStateCtx;
use crate::server::api::v1_resources::error_handling::FromDomainError;
use crate::server::api::view::View;

pub struct ApiSessionRoutesV1;
#[OpenApi]
impl ApiSessionRoutesV1 {
    #[oai(path = "/session", method = "post")]
    async fn create_session(
        &self,
        state: SharedStateCtx<'_>,
    ) -> SessionCreateResponse {
        match state.domain.session_service.create_session().await {
            Ok(entity) => {
                let view = SessionWithSecretView::from_entity(&entity);
                SessionCreateResponse::Ok(Json(view))
            }
            Err(err) => SessionCreateResponse::from_domain_error(&err),
        }
    }

    #[oai(path = "/session/:session_id", method = "get")]
    async fn get_session(
        &self,
        state: SharedStateCtx<'_>,
        session_id: Path<String>,
    ) -> SessionGetResponse {
        match state.domain.session_service.get_session(&session_id).await {
            Ok(record) => {
                let view = SessionView::from_entity(&record);
                SessionGetResponse::Ok(Json(view))
            }
            Err(err) => SessionGetResponse::from_domain_error(&err),
        }
    }

    #[oai(path = "/session/:session_id", method = "delete")]
    async fn delete_session(
        &self,
        state: SharedStateCtx<'_>,
        session_id: Path<String>,
        auth: ApiV1AuthScheme,
    ) -> SessionDeleteResponse {
        match state
            .domain
            .session_service
            .delete_session(&session_id, &auth.token())
            .await
        {
            Ok(()) => SessionDeleteResponse::Ok,
            Err(err) => SessionDeleteResponse::from_domain_error(&err),
        }
    }
}
