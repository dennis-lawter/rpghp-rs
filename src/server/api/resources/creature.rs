use std::sync::Arc;

use poem::web::Data;
use poem_openapi::ApiResponse;
use poem_openapi::Object;
use poem_openapi::OpenApi;
use poem_openapi::param::Path;
use poem_openapi::payload::Json;
use uuid::Uuid;

use crate::server::api::records::Record;
use crate::server::api::records::RecordQueryError;
use crate::server::api::records::creature::CreatureRecord;
use crate::server::api::records::session::SessionRecord;
use crate::server::shared_state::SharedState;

use super::ApiV1AuthScheme;
use super::ApiV1AuthSchemeOptional;
use super::View;

pub struct ApiCreatureRoutesV1;
#[OpenApi]
impl ApiCreatureRoutesV1 {
    #[oai(path = "/session/:session_id/creature", method = "post")]
    async fn create_creature(
        &self,
        state: Data<&Arc<SharedState>>,
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
        state: Data<&Arc<SharedState>>,
        session_id: Path<String>,
        auth: ApiV1AuthSchemeOptional,
    ) -> CreatureListResponse {
        let session = match &auth {
            ApiV1AuthSchemeOptional::NoAuth => {
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
                    Err(e) => return CreatureListResponse::from_record_query_error(&e),
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
        state: Data<&Arc<SharedState>>,
        session_id: Path<String>,
        creature_id: Path<String>,
        auth: ApiV1AuthSchemeOptional,
    ) -> CreatureGetResponse {
        let session = match &auth {
            ApiV1AuthSchemeOptional::NoAuth => {
                let session_id = match Uuid::parse_str(&session_id) {
                    Ok(uuid) => uuid,
                    _ => return CreatureGetResponse::NotFound,
                };
                match SessionRecord::find_by_id(&state.pool, &session_id).await {
                    Ok(Some(session)) => session,
                    _ => return CreatureGetResponse::NotFound,
                }
            }
            ApiV1AuthSchemeOptional::Bearer(token) => {
                match SessionRecord::find_by_id_and_secret(&session_id, &token.0.token, &state.pool)
                    .await
                {
                    Ok(session) => session,
                    Err(e) => return CreatureGetResponse::from_record_query_error(&e),
                }
            }
        };

        let creature_id = match Uuid::parse_str(&creature_id) {
            Ok(uuid) => uuid,
            _ => return CreatureGetResponse::NotFound,
        };

        let creature = match CreatureRecord::find_by_id(&state.pool, &creature_id).await {
            Ok(Some(creature)) => creature,
            _ => return CreatureGetResponse::NotFound,
        };

        if creature.session_id != session.rpghp_session_id {
            return CreatureGetResponse::Forbidden;
        }

        let creature_view = CreatureView::from_record(&creature);

        match &auth {
            ApiV1AuthSchemeOptional::Bearer(_) => CreatureGetResponse::Ok(Json(creature_view)),
            ApiV1AuthSchemeOptional::NoAuth => {
                CreatureGetResponse::Ok(Json(creature_view.simplified_if_hp_hidden()))
            }
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
impl CreatureListResponse {
    fn from_record_query_error(e: &RecordQueryError) -> Self {
        match e {
            RecordQueryError::NotFound => Self::NotFound,
            RecordQueryError::Unauthorized => Self::Unauthorized,
            RecordQueryError::Forbidden => Self::Forbidden,
        }
    }
}

#[derive(ApiResponse)]
enum CreatureGetResponse {
    #[oai(status = 200)]
    Ok(Json<CreatureView>),

    #[oai(status = 401)]
    Unauthorized,

    #[oai(status = 403)]
    Forbidden,

    #[oai(status = 404)]
    NotFound,
}
impl CreatureGetResponse {
    fn from_record_query_error(e: &RecordQueryError) -> Self {
        match e {
            RecordQueryError::NotFound => Self::NotFound,
            RecordQueryError::Unauthorized => Self::Unauthorized,
            RecordQueryError::Forbidden => Self::Forbidden,
        }
    }
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
