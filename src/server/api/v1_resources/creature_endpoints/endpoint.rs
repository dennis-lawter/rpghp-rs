use std::sync::Arc;

use poem::web::Data;
use poem_openapi::OpenApi;
use poem_openapi::param::Path;
use poem_openapi::payload::Json;

use super::super::auth::ApiV1AuthScheme;
use super::super::auth::ApiV1AuthSchemeOptional;
use super::requests::CreatureCreateRequest;
use super::responses::CreatureCreateResponse;
use super::responses::CreatureGetResponse;
use super::responses::CreatureListResponse;
use super::views::CreatureView;
use crate::server::api::v1_resources::error_handling::FromDomainError;
use crate::server::api::view::View;
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
                &auth.token(),
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
        let opt_token = auth.opt_token();
        match state
            .domain
            .get_all_creatures_for_session(&session_id, opt_token.as_ref())
            .await
        {
            Ok(creatures) => {
                let views: Vec<CreatureView> = if auth.auth_provided() {
                    creatures.iter().map(CreatureView::from_record).collect()
                } else {
                    creatures
                        .iter()
                        .map(CreatureView::from_record)
                        .map(CreatureView::restricted_view)
                        .collect()
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
        let opt_token = auth.opt_token();
        let record = match state
            .domain
            .get_creature(&session_id, &creature_id, opt_token.as_ref())
            .await
        {
            Ok(record) => record,
            Err(err) => return CreatureGetResponse::from_domain_error(&err),
        };
        let view = if auth.auth_provided() {
            CreatureView::from_record(&record)
        } else {
            CreatureView::from_record(&record).restricted_view()
        };
        CreatureGetResponse::Ok(Json(view))
    }
}
