use poem_openapi::Object;

use crate::domain::entity::init_group::InitGroupEntity;
use crate::server::api::view::FromEntity;
use crate::server::api::view::View;

#[derive(Object, serde::Serialize, Clone, Debug)]
pub struct InitGroupView {
    id: String,
    rank: i64,
}
impl View for InitGroupView {}
impl FromEntity<InitGroupEntity> for InitGroupView {
    fn from_entity(entity: &InitGroupEntity) -> Self {
        Self {
            id: format!("{}", entity.rpghp_init_group_id),
            rank: entity.rank,
        }
    }
}
