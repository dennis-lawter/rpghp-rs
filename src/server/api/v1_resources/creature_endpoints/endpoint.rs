use poem_openapi::OpenApi;
use poem_openapi::param::Path;
use poem_openapi::payload::Json;

use super::super::auth::ApiAuthScheme;
use super::super::auth::ApiOptAuthScheme;
use super::requests::CreateCreatureRequest;
use super::responses::CreateCreatureResponse;
use super::responses::GetCreatureResponse;
use super::responses::ListCreatureResponse;
use super::views::CreatureView;
use crate::server::api::SharedStateCtx;
use crate::server::api::v1_resources::error_handling::FromDomainError;
use crate::server::api::view::FromEntity;

pub struct ApiCreatureRoutesV1;
#[OpenApi]
impl ApiCreatureRoutesV1 {
    #[oai(
        path = "/session/:session_id/init_group/:init_group_id/creature",
        method = "post"
    )]
    async fn create_creature(
        &self,
        state: SharedStateCtx<'_>,
        session_id: Path<String>,
        init_group_id: Path<String>,
        data: Json<CreateCreatureRequest>,
        auth: ApiAuthScheme,
    ) -> CreateCreatureResponse {
        match state
            .domain
            .creature_service
            .create_creature(
                &session_id,
                &auth.token(),
                &init_group_id,
                &data.creature_name,
                data.max_hp,
                data.curr_hp,
                data.hp_hidden,
                data.icon.clone(),
            )
            .await
        {
            Ok(_) => CreateCreatureResponse::Created,
            Err(err) => CreateCreatureResponse::from_domain_error(&err),
        }
    }

    #[oai(path = "/session/:session_id/creature", method = "get")]
    async fn list_creature(
        &self,
        state: SharedStateCtx<'_>,
        session_id: Path<String>,
        auth: ApiOptAuthScheme,
    ) -> ListCreatureResponse {
        let opt_token = auth.opt_token();
        match state
            .domain
            .creature_service
            .get_all_creatures_for_session(&session_id, opt_token.as_ref())
            .await
        {
            Ok(creatures) => {
                let views: Vec<CreatureView> = if auth.auth_provided() {
                    creatures.iter().map(CreatureView::from_entity).collect()
                } else {
                    creatures
                        .iter()
                        .map(CreatureView::from_entity)
                        .map(CreatureView::without_hp_details)
                        .collect()
                };
                ListCreatureResponse::Ok(Json(views))
            }
            Err(err) => ListCreatureResponse::from_domain_error(&err),
        }
    }

    #[oai(path = "/session/:session_id/creature/:creature_id", method = "get")]
    async fn get_creature(
        &self,
        state: SharedStateCtx<'_>,
        session_id: Path<String>,
        creature_id: Path<String>,
        auth: ApiOptAuthScheme,
    ) -> GetCreatureResponse {
        let opt_token = auth.opt_token();
        let record = match state
            .domain
            .creature_service
            .get_creature(&session_id, &creature_id, opt_token.as_ref())
            .await
        {
            Ok(record) => record,
            Err(err) => return GetCreatureResponse::from_domain_error(&err),
        };
        let view = if auth.auth_provided() {
            CreatureView::from_entity(&record)
        } else {
            CreatureView::from_entity(&record).without_hp_details()
        };
        GetCreatureResponse::Ok(Json(view))
    }
}
