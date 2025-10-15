use std::sync::Arc;

use poem::web::Data;
use poem_openapi::OpenApi;
use poem_openapi::param::Path;
use poem_openapi::payload::Json;

use super::super::auth::ApiV1AuthScheme;
use super::responses::SessionCreateResponse;
use super::responses::SessionDeleteResponse;
use super::responses::SessionGetResponse;
use super::views::SessionView;
use super::views::SessionWithSecretView;
use crate::server::api::v1_resources::error_handling::FromDomainError;
use crate::server::api::view::View;
use crate::server::shared_state::SharedState;

pub struct ApiSessionRoutesV1;
#[OpenApi]
impl ApiSessionRoutesV1 {
    #[oai(path = "/session", method = "post")]
    async fn create_session(
        &self,
        state: Data<&Arc<SharedState>>,
    ) -> SessionCreateResponse {
        match state.domain.create_session().await {
            Ok(record) => {
                let view = SessionWithSecretView::from_record(&record);
                SessionCreateResponse::Ok(Json(view))
            }
            Err(err) => SessionCreateResponse::from_domain_error(&err),
        }
    }

    #[oai(path = "/session/:session_id", method = "get")]
    async fn get_session(
        &self,
        state: Data<&Arc<SharedState>>,
        session_id: Path<String>,
    ) -> SessionGetResponse {
        match state.domain.get_session(&session_id).await {
            Ok(record) => {
                let view = SessionView::from_record(&record);
                SessionGetResponse::Ok(Json(view))
            }
            Err(err) => SessionGetResponse::from_domain_error(&err),
        }
    }

    #[oai(path = "/session/:session_id", method = "delete")]
    async fn delete_session(
        &self,
        state: Data<&Arc<SharedState>>,
        session_id: Path<String>,
        auth: ApiV1AuthScheme,
    ) -> SessionDeleteResponse {
        match state
            .domain
            .delete_session(&session_id, &auth.0.token)
            .await
        {
            Ok(()) => SessionDeleteResponse::Ok,
            Err(err) => SessionDeleteResponse::from_domain_error(&err),
        }
    }
}
