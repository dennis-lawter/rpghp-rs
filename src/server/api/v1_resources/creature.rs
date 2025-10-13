use std::sync::Arc;

use poem::web::Data;
use poem_openapi::ApiResponse;
use poem_openapi::Object;
use poem_openapi::OpenApi;
use poem_openapi::param::Path;
use poem_openapi::payload::Json;

use super::ApiV1AuthScheme;
use super::ApiV1AuthSchemeOptional;
use super::View;
use crate::domain::DomainError;
use crate::domain::records::creature::CreatureRecord;
use crate::server::shared_state::SharedState;

pub struct ApiCreatureRoutesV1;
#[OpenApi]
impl ApiCreatureRoutesV1 {
    #[oai(path = "/session/:session_id/creature", method = "post")]
    async fn create_creature(
        &self,
        state: Data<&Arc<SharedState>>,
        session_id: Path<String>,
        data: Json<CreatureCreateRequest>,
        auth: ApiV1AuthScheme,
    ) -> CreatureCreateResponse {
        match state
            .domain
            .create_creature(
                &session_id,
                &auth.0.token,
                &data.creature_name,
                data.max_hp,
                data.curr_hp,
                data.hp_hidden,
                data.icon.clone(),
            )
            .await
        {
            Ok(_) => CreatureCreateResponse::Created,
            Err(err) => CreatureCreateResponse::from_domain_error(&err),
        }
    }

    #[oai(path = "/session/:session_id/creature", method = "get")]
    async fn list_creature(
        &self,
        state: Data<&Arc<SharedState>>,
        session_id: Path<String>,
        auth: ApiV1AuthSchemeOptional,
    ) -> CreatureListResponse {
        let opt_secret = match &auth {
            ApiV1AuthSchemeOptional::Bearer(bearer_auth) => Some(bearer_auth.0.token.clone()),
            ApiV1AuthSchemeOptional::NoAuth => None,
        };
        match state
            .domain
            .get_all_creatures_for_session(&session_id, opt_secret.as_ref())
            .await
        {
            Ok(creatures) => {
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
            Err(err) => CreatureListResponse::from_domain_error(&err),
        }
    }

    #[oai(path = "/session/:session_id/creature/:creature_id", method = "get")]
    async fn get_creature(
        &self,
        state: Data<&Arc<SharedState>>,
        session_id: Path<String>,
        creature_id: Path<String>,
        auth: ApiV1AuthSchemeOptional,
    ) -> CreatureGetResponse {
        let opt_secret = match &auth {
            ApiV1AuthSchemeOptional::Bearer(bearer_auth) => Some(bearer_auth.0.token.clone()),
            ApiV1AuthSchemeOptional::NoAuth => None,
        };
        let record = match state
            .domain
            .get_creature(&session_id, &creature_id, opt_secret.as_ref())
            .await
        {
            Ok(record) => record,
            Err(err) => return CreatureGetResponse::from_domain_error(&err),
        };
        let view = match &auth {
            ApiV1AuthSchemeOptional::Bearer(_) => CreatureView::from_record(&record),
            ApiV1AuthSchemeOptional::NoAuth => {
                CreatureView::from_record(&record).simplified_if_hp_hidden()
            }
        };
        CreatureGetResponse::Ok(Json(view))
    }
}

// Create

#[derive(serde::Deserialize, poem_openapi::Object)]
struct CreatureCreateRequest {
    creature_name: String,
    max_hp: i32,
    curr_hp: i32,
    hp_hidden: bool,
    icon: Option<String>,
}

#[derive(ApiResponse)]
enum CreatureCreateResponse {
    #[oai(status = 201)]
    Created,
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
impl CreatureCreateResponse {
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

// List

#[derive(ApiResponse)]
enum CreatureListResponse {
    #[oai(status = 200)]
    Ok(Json<Vec<CreatureView>>),
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
impl CreatureListResponse {
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

// Get

#[derive(ApiResponse)]
enum CreatureGetResponse {
    #[oai(status = 200)]
    Ok(Json<CreatureView>),
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
impl CreatureGetResponse {
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
        #[allow(clippy::cast_precision_loss)]
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
