use crate::server::api::api_shared_state::ApiSharedState;
use crate::server::api::domain::Record;
use crate::server::api::domain::session_record::SessionRecord;
use crate::server::api::render::View;
use crate::server::api::render::session_view::SessionCreateResponse;
use crate::server::api::render::session_view::SessionDeleteResponse;
use crate::server::api::render::session_view::SessionGetResponse;
use crate::server::api::render::session_view::SessionView;
use crate::server::api::render::session_view::SessionWithSecretView;

use poem::web::Data;
use poem_openapi::OpenApi;
use poem_openapi::param::Path;
use poem_openapi::payload::Json;
use uuid::Uuid;

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
        let uuid = match Uuid::parse_str(&session_id) {
            Ok(uuid) => uuid,
            Err(_) => return SessionGetResponse::NotFound,
        };

        match SessionRecord::find_by_secret_or_id(&state.pool, &uuid).await {
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
    ) -> SessionDeleteResponse {
        let uuid = match Uuid::parse_str(&session_id) {
            Ok(uuid) => uuid,
            Err(_) => return SessionDeleteResponse::NotFound,
        };

        match SessionRecord::find_by_secret(&state.pool, &uuid).await {
            Ok(Some(session_record)) => match session_record.delete(&state.pool).await {
                Ok(()) => SessionDeleteResponse::Created,
                Err(_) => SessionDeleteResponse::NotFound,
            },
            _ => SessionDeleteResponse::NotFound,
        }
    }
}
