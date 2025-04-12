use crate::server::api::api_shared_state::ApiSharedState;
use crate::server::api::domain::session::SessionRecord;

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
        state: Data<&ApiSharedState>,
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
        state: Data<&ApiSharedState>,
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
        state: Data<&ApiSharedState>,
        session_id: Path<String>,
        auth: super::ApiV1AuthScheme,
    ) -> SessionDeleteResponse {
        let session_id = match Uuid::parse_str(&session_id) {
            Ok(uuid) => uuid,
            Err(_) => return SessionDeleteResponse::NotFound,
        };

        let session = match SessionRecord::find_by_id(&state.pool, &session_id).await {
            Ok(Some(session_record)) => session_record,
            _ => return SessionDeleteResponse::NotFound,
        };

        let bearer_token = match Uuid::parse_str(&auth.0.token) {
            Ok(uuid) => uuid,
            Err(_) => return SessionDeleteResponse::Unauthorized,
        };

        if bearer_token != session.secret {
            return SessionDeleteResponse::Forbidden;
        }

        match session.delete(&state.pool).await {
            Ok(_) => SessionDeleteResponse::Ok,
            _ => SessionDeleteResponse::Forbidden,
        }
    }
}

#[derive(ApiResponse)]
enum SessionCreateResponse {
    #[oai(status = 200)]
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
