use poem_openapi::OpenApi;
use poem_openapi::param::Path;
use poem_openapi::payload::Json;

use super::super::auth::ApiAuthScheme;
use super::responses::CreateSessionResponse;
use super::responses::DeleteSessionResponse;
use super::responses::GetSessionResponse;
use super::views::SessionWithSecretView;
use super::views::SessionWithoutSecretView;
use crate::server::api::SharedStateCtx;
use crate::server::api::v1_resources::error_handling::FromDomainError;
use crate::server::api::view::FromEntity;

pub struct ApiSessionRoutesV1;
#[OpenApi]
impl ApiSessionRoutesV1 {
    #[oai(path = "/session", method = "post")]
    async fn create_session(
        &self,
        state: SharedStateCtx<'_>,
    ) -> CreateSessionResponse {
        match state.domain.session_service.create_session().await {
            Ok(entity) => {
                let view = SessionWithSecretView::from_entity(&entity);
                CreateSessionResponse::Ok(Json(view))
            }
            Err(err) => CreateSessionResponse::from_domain_error(&err),
        }
    }

    #[oai(path = "/session/:session_id", method = "get")]
    async fn get_session(
        &self,
        state: SharedStateCtx<'_>,
        session_id: Path<String>,
    ) -> GetSessionResponse {
        match state.domain.session_service.get_session(&session_id).await {
            Ok(record) => {
                let view = SessionWithoutSecretView::from_entity(&record);
                GetSessionResponse::Ok(Json(view))
            }
            Err(err) => GetSessionResponse::from_domain_error(&err),
        }
    }

    #[oai(path = "/session/:session_id", method = "delete")]
    async fn delete_session(
        &self,
        state: SharedStateCtx<'_>,
        session_id: Path<String>,
        auth: ApiAuthScheme,
    ) -> DeleteSessionResponse {
        match state
            .domain
            .session_service
            .delete_session(&session_id, &auth.token())
            .await
        {
            Ok(()) => DeleteSessionResponse::Ok,
            Err(err) => DeleteSessionResponse::from_domain_error(&err),
        }
    }
}
