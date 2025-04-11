use poem::web::Data;
use poem_openapi::ApiResponse;
use poem_openapi::OpenApi;
use poem_openapi::param::Path;
use poem_openapi::payload::Json;
use uuid::Uuid;

use crate::server::api::api_shared_state::ApiSharedState;
use crate::server::api::domain::session::SessionRecord;

pub struct ApiCreatureRoutesV1;
#[OpenApi]
impl ApiCreatureRoutesV1 {
    #[oai(path = "/session/:session_id/creature", method = "post")]
    async fn create_session(
        &self,
        state: Data<&ApiSharedState>,
        session_id: Path<String>,
        _data: Json<CreateCreatureRequest>,
    ) -> CreatureCreateResponse {
        let uuid = match Uuid::parse_str(&session_id) {
            Ok(uuid) => uuid,
            _ => return CreatureCreateResponse::NotFound,
        };

        let session: SessionRecord = match SessionRecord::find_by_secret(&state.pool, &uuid).await {
            Ok(Some(session)) => session,
            _ => return CreatureCreateResponse::NotFound,
        };
        let _session_id = session.rpghp_session_id;

        // TODO: Add creature saving

        CreatureCreateResponse::Created
    }
}

#[derive(serde::Deserialize, poem_openapi::Object)]
struct CreateCreatureRequest {
    creature_name: String,
    max_hp: i32,
    curr_hp: i32,
    hp_hidden: bool,
}

#[derive(ApiResponse)]
enum CreatureCreateResponse {
    #[oai(status = 201)]
    Created,

    #[oai(status = 404)]
    NotFound,
}
