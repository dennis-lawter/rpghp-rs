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
use crate::domain::command::session::create::CreateSessionCommand;
use crate::domain::command::session::delete::DeleteSessionCommand;
use crate::domain::command::session::get::GetSessionCommand;
use crate::server::api::v1::error_handling::FromDomainError;
use crate::server::api::view::FromEntity;
use crate::server::shared_state::SharedState;

pub struct ApiSessionRoutesV1;
#[OpenApi]
impl ApiSessionRoutesV1 {
    #[oai(path = "/session", method = "post")]
    async fn create_session(
        &self,
        state: Data<&Arc<SharedState>>,
    ) -> SessionCreateResponse {
        let cmd = CreateSessionCommand {};
        match state.domain.session_service.create(&cmd).await {
            Ok(record) => {
                SessionCreateResponse::Ok(Json(SessionWithSecretView::from_entity(&record)))
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
        let cmd = match GetSessionCommand::new(&session_id) {
            Ok(cmd) => cmd,
            Err(err) => return SessionGetResponse::from_domain_error(&err),
        };
        match state.domain.session_service.get(&cmd).await {
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
        state: Data<&Arc<SharedState>>,
        session_id: Path<String>,
        auth: ApiV1AuthScheme,
    ) -> SessionDeleteResponse {
        let domain_auth = match DomainAuth::new(&session_id, &auth.token()) {
            Ok(valid_format_auth) => valid_format_auth,
            Err(err) => return CreatureCreateResponse::from_domain_error(&err),
        };
        let command = data.0.to_command(&domain_auth);
        match state.domain.session_service.delete(&cmd).await {
            Ok(()) => SessionDeleteResponse::Ok,
            Err(err) => SessionDeleteResponse::from_domain_error(&err),
        }
    }
}
