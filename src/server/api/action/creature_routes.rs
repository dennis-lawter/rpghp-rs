use crate::server::api::action_input::CreateCreatureRequest;
use crate::server::api::api_shared_state::ApiSharedState;
use crate::server::api::domain::session_record::SessionRecord;
use crate::server::api::render::creature_view::CreatureCreateResponse;
use poem::web::Data;
use poem_openapi::OpenApi;
use poem_openapi::param::Path;
use poem_openapi::payload::Form;
use uuid::Uuid;

pub struct ApiCreatureRoutesV1;
#[OpenApi]
impl ApiCreatureRoutesV1 {
    #[oai(path = "/session/:session_id/creature", method = "post")]
    async fn create_session(
        &self,
        state: Data<&ApiSharedState>,
        session_id: Path<String>,
        _data: Form<CreateCreatureRequest>,
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

        CreatureCreateResponse::Ok
    }
}
