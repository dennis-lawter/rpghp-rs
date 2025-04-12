use poem::web::Data;
use poem_openapi::ApiResponse;
use poem_openapi::Object;
use poem_openapi::OpenApi;
use poem_openapi::param::Path;
use poem_openapi::payload::Json;
use uuid::Uuid;

use crate::server::api::api_shared_state::ApiSharedState;
use crate::server::api::domain::Record;
use crate::server::api::domain::RecordQueryError;
use crate::server::api::domain::creature::CreatureRecord;
use crate::server::api::domain::session::SessionRecord;

use super::ApiV1AuthScheme;
use super::ApiV1AuthSchemeOptional;
use super::View;

pub struct ApiCreatureRoutesV1;
#[OpenApi]
impl ApiCreatureRoutesV1 {
    #[oai(path = "/session/:session_id/creature", method = "post")]
    async fn create_creature(
        &self,
        state: Data<&ApiSharedState>,
        session_id: Path<String>,
        data: Json<CreateCreatureRequest>,
        auth: ApiV1AuthScheme,
    ) -> CreatureCreateResponse {
        let session =
            match SessionRecord::find_by_id_and_secret(&session_id, &auth.0.token, &state.pool)
                .await
            {
                Err(RecordQueryError::Forbidden) => return CreatureCreateResponse::Forbidden,
                Err(RecordQueryError::NotFound) => return CreatureCreateResponse::NotFound,
                Err(RecordQueryError::Unauthorized) => return CreatureCreateResponse::Unauthorized,
                Ok(session) => session,
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

    #[oai(path = "/session/:session_id/creature", method = "get")]
    async fn list_creature(
        &self,
        state: Data<&ApiSharedState>,
        session_id: Path<String>,
        auth: ApiV1AuthSchemeOptional,
    ) -> CreatureListResponse {
        let session = match &auth {
            ApiV1AuthSchemeOptional::NoAuth => {
                // When no auth is provided, you'll have a more restricted view...
                // However, obviously, you can skip much of the auth for the session object.
                // We'll just look your session up by id or 404 you.
                let session_id = match Uuid::parse_str(&session_id) {
                    Ok(uuid) => uuid,
                    _ => return CreatureListResponse::NotFound,
                };
                match SessionRecord::find_by_id(&state.pool, &session_id).await {
                    Ok(Some(session)) => session,
                    _ => return CreatureListResponse::NotFound,
                }
            }
            ApiV1AuthSchemeOptional::Bearer(token) => {
                match SessionRecord::find_by_id_and_secret(&session_id, &token.0.token, &state.pool)
                    .await
                {
                    Ok(session) => session,
                    Err(RecordQueryError::Forbidden) => return CreatureListResponse::Forbidden,
                    Err(RecordQueryError::NotFound) => return CreatureListResponse::NotFound,
                    Err(RecordQueryError::Unauthorized) => {
                        return CreatureListResponse::Unauthorized;
                    }
                }
            }
        };

        let creatures = match CreatureRecord::find_by_session_id(
            &state.pool,
            &session.rpghp_session_id,
        )
        .await
        {
            Ok(creatures) => creatures,
            Err(_) => return CreatureListResponse::NotFound,
        };

        let views: Vec<CreatureView> = match &auth {
            ApiV1AuthSchemeOptional::Bearer(_) => {
                creatures.iter().map(CreatureView::from_record).collect()
            }
            ApiV1AuthSchemeOptional::NoAuth => creatures
                .iter()
                .map(CreatureView::from_record)
                .map(CreatureView::simplified_if_hp_hidden)
                .collect(),
        };

        CreatureListResponse::Ok(Json(views))
    }

    #[oai(path = "/session/:session_id/creature/:creature_id", method = "get")]
    async fn get_creature(
        &self,
        _state: Data<&ApiSharedState>,
        session_id: Path<String>,
        creature_id: Path<String>,
        _auth: ApiV1AuthSchemeOptional,
    ) -> CreatureGetResponse {
        let _id = match Uuid::parse_str(&session_id) {
            Ok(uuid) => uuid,
            _ => return CreatureGetResponse::NotFound,
        };
        let _creature_id = match Uuid::parse_str(&creature_id) {
            Ok(uuid) => uuid,
            _ => return CreatureGetResponse::NotFound,
        };
        todo!()
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

    #[oai(status = 401)]
    Unauthorized,

    #[oai(status = 403)]
    Forbidden,

    #[oai(status = 404)]
    NotFound,
}

#[derive(ApiResponse)]
enum CreatureListResponse {
    #[oai(status = 200)]
    Ok(Json<Vec<CreatureView>>),

    #[oai(status = 401)]
    Unauthorized,

    #[oai(status = 403)]
    Forbidden,

    #[oai(status = 404)]
    NotFound,
}

#[derive(ApiResponse)]
enum CreatureGetResponse {
    #[oai(status = 404)]
    NotFound,
}

#[derive(Object, serde::Serialize)]
struct CreatureView {
    creature_id: String,
    creature_name: String,
    max_hp: Option<i32>,
    curr_hp: Option<i32>,
    approx_hp: f32,
    hp_hidden: bool,
}
impl super::View<CreatureRecord> for CreatureView {
    fn from_record(record: &CreatureRecord) -> Self {
        let id = format!("{}", record.rpghp_creature_id);
        let approx_hp = record.curr_hp as f32 / record.max_hp as f32;
        Self {
            creature_id: id,
            creature_name: record.creature_name.clone(),
            max_hp: Some(record.max_hp),
            curr_hp: Some(record.curr_hp),
            approx_hp,
            hp_hidden: record.hp_hidden,
        }
    }
}
impl CreatureView {
    fn simplified_if_hp_hidden(self) -> Self {
        if self.hp_hidden {
            Self {
                creature_id: self.creature_id,
                creature_name: self.creature_name,
                max_hp: None,
                curr_hp: None,
                approx_hp: self.approx_hp,
                hp_hidden: self.hp_hidden,
            }
        } else {
            self
        }
    }
}
