use poem::web::Data;
use poem_openapi::ApiResponse;
use poem_openapi::OpenApi;
use poem_openapi::param::Path;
use poem_openapi::payload::Json;
use uuid::Uuid;

use crate::server::api::api_shared_state::ApiSharedState;
use crate::server::api::domain::Record;
use crate::server::api::domain::creature::CreatureRecord;
use crate::server::api::domain::session::SessionRecord;

pub struct ApiCreatureRoutesV1;
#[OpenApi]
impl ApiCreatureRoutesV1 {
    #[oai(path = "/session/:session_id/creature", method = "post")]
    async fn create_session(
        &self,
        state: Data<&ApiSharedState>,
        session_id: Path<String>,
        data: Json<CreateCreatureRequest>,
    ) -> CreatureCreateResponse {
        let uuid = match Uuid::parse_str(&session_id) {
            Ok(uuid) => uuid,
            _ => return CreatureCreateResponse::NotFound,
        };

        let session: SessionRecord = match SessionRecord::find_by_secret(&state.pool, &uuid).await {
            Ok(Some(session)) => session,
            _ => return CreatureCreateResponse::NotFound,
        };
        let session_id = session.rpghp_session_id;

        let creature = CreatureRecord {
            rpghp_creature_id: Uuid::new_v4(),
            session_id,
            creature_name: data.creature_name.clone(),
            max_hp: data.max_hp,
            curr_hp: data.curr_hp,
            hp_hidden: data.hp_hidden,
        };

        match creature.save(&state.pool).await {
            Ok(_) => CreatureCreateResponse::Created,
            Err(_) => CreatureCreateResponse::BadRequest,
        }
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

    #[oai(status = 400)]
    BadRequest,

    #[oai(status = 404)]
    NotFound,
}
