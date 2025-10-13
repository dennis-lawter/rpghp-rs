use std::sync::Arc;

use poem::web::Data;
use poem_openapi::ApiResponse;
use poem_openapi::Object;
use poem_openapi::OpenApi;
use poem_openapi::param::Path;
use poem_openapi::payload::Json;

use super::View;
use crate::domain::DomainError;
use crate::domain::records::session::SessionRecord;
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
        auth: super::ApiV1AuthScheme,
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

// Create

#[derive(ApiResponse)]
enum SessionCreateResponse {
    #[oai(status = 201)]
    Ok(Json<SessionWithSecretView>),
    #[oai(status = 404)]
    NotFound,
}
impl SessionCreateResponse {
    const fn from_domain_error(_err: &DomainError) -> Self {
        // match err {
        //     DomainError::NotFound => Self::NotFound,
        //     // DomainError::Unauthorized => Self::NotFound,
        //     DomainError::Forbidden => Self::NotFound,
        //     DomainError::SqlxError(_) => Self::NotFound,
        //     DomainError::InvalidUuid(_) => Self::NotFound,
        // }
        Self::NotFound
    }
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

// Get

#[derive(ApiResponse)]
enum SessionGetResponse {
    #[oai(status = 200)]
    Ok(Json<SessionView>),
    #[oai(status = 404)]
    NotFound,
}
impl SessionGetResponse {
    const fn from_domain_error(_err: &DomainError) -> Self {
        // match err {
        //     DomainError::NotFound => Self::NotFound,
        //     // DomainError::Unauthorized => Self::NotFound,
        //     DomainError::Forbidden => Self::NotFound,
        //     DomainError::SqlxError(_) => Self::NotFound,
        //     DomainError::InvalidUuid(_) => Self::NotFound,
        // }
        Self::NotFound
    }
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

// Delete

#[derive(ApiResponse)]
enum SessionDeleteResponse {
    #[oai(status = 200)]
    Ok,
    #[oai(status = 400)]
    BadRequest,
    // #[oai(status = 401)]
    // Unauthorized,
    #[oai(status = 403)]
    Forbidden,
    #[oai(status = 404)]
    NotFound,
    #[oai(status = 500)]
    InternalError,
}
impl SessionDeleteResponse {
    const fn from_domain_error(err: &DomainError) -> Self {
        match err {
            DomainError::NotFound => Self::NotFound,
            // DomainError::Unauthorized => Self::Unauthorized,
            DomainError::Forbidden => Self::Forbidden,
            DomainError::SqlxError(_) => Self::InternalError,
            DomainError::InvalidUuid(_) => Self::BadRequest,
        }
    }
}
