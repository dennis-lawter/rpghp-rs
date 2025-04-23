use std::sync::Arc;

use crate::server::api::records::RecordQueryError;
use crate::server::api::records::session::SessionRecord;
use crate::server::shared_state::SharedState;

use poem::web::Data;
use poem_openapi::ApiResponse;
use poem_openapi::Object;
use poem_openapi::OpenApi;
use poem_openapi::param::Path;
use poem_openapi::payload::Json;
use uuid::Uuid;

use super::Record;
use super::View;

pub struct ApiSessionRoutesV1;
#[OpenApi]
impl ApiSessionRoutesV1 {
    #[oai(path = "/session", method = "post")]
    async fn create_session(
        &self,
        state: Data<&Arc<SharedState>>,
    ) -> SessionCreateResponse {
        let session_record = SessionRecord::new();
        let res = session_record.save(&state.pool).await;
        let view = SessionWithSecretView::from_record(&session_record);

        match res {
            Ok(_) => SessionCreateResponse::Ok(Json(view)),
            Err(_) => SessionCreateResponse::NotFound,
        }
    }

    #[oai(path = "/session/:session_id", method = "get")]
    async fn get_session(
        &self,
        state: Data<&Arc<SharedState>>,
        session_id: Path<String>,
    ) -> SessionGetResponse {
        let session_id = match Uuid::parse_str(&session_id) {
            Ok(uuid) => uuid,
            Err(_) => return SessionGetResponse::NotFound,
        };

        match SessionRecord::find_by_id(&state.pool, &session_id).await {
            Ok(Some(session_record)) => {
                let view = SessionView::from_record(&session_record);
                SessionGetResponse::Ok(Json(view))
            }
            _ => SessionGetResponse::NotFound,
        }
    }

    #[oai(path = "/session/:session_id", method = "delete")]
    async fn delete_session(
        &self,
        state: Data<&Arc<SharedState>>,
        session_id: Path<String>,
        auth: super::ApiV1AuthScheme,
    ) -> SessionDeleteResponse {
        let session =
            match SessionRecord::find_by_id_and_secret(&session_id, &auth.0.token, &state.pool)
                .await
            {
                Err(RecordQueryError::Forbidden) => return SessionDeleteResponse::Forbidden,
                Err(RecordQueryError::NotFound) => return SessionDeleteResponse::NotFound,
                Err(RecordQueryError::Unauthorized) => return SessionDeleteResponse::Unauthorized,
                Ok(session) => session,
            };

        match session.delete(&state.pool).await {
            Ok(_) => SessionDeleteResponse::Ok,
            Err(_) => SessionDeleteResponse::InternalError,
        }
    }
}

#[derive(ApiResponse)]
enum SessionCreateResponse {
    #[oai(status = 201)]
    Ok(Json<SessionWithSecretView>),

    #[oai(status = 404)]
    NotFound,
}
#[derive(Object, serde::Serialize)]
struct SessionWithSecretView {
    pub rpghp_session_id: String,
    pub secret: String,
}
impl super::View<SessionRecord> for SessionWithSecretView {
    fn from_record(record: &SessionRecord) -> Self {
        let rpghp_session_id = format!("{}", record.rpghp_session_id);
        let secret = format!("{}", record.secret);
        Self {
            rpghp_session_id,
            secret,
        }
    }
}

#[derive(ApiResponse)]
enum SessionGetResponse {
    #[oai(status = 200)]
    Ok(Json<SessionView>),

    #[oai(status = 404)]
    NotFound,
}

#[derive(ApiResponse)]
enum SessionDeleteResponse {
    #[oai(status = 200)]
    Ok,

    #[oai(status = 401)]
    Unauthorized,

    #[oai(status = 403)]
    Forbidden,

    #[oai(status = 404)]
    NotFound,

    #[oai(status = 500)]
    InternalError,
}

#[derive(Object, serde::Serialize)]
struct SessionView {
    pub rpghp_session_id: String,
}
impl super::View<SessionRecord> for SessionView {
    fn from_record(record: &SessionRecord) -> Self {
        let rpghp_session_id = format!("{}", record.rpghp_session_id);
        Self { rpghp_session_id }
    }
}
