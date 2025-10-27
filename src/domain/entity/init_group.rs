use uuid::Uuid;

use crate::domain::entity::Entity;

pub struct InitGroupEntity {
    pub rpghp_init_group_id: Uuid,
    pub session_id: Uuid,
    pub rank: i64,
}
impl Entity for InitGroupEntity {}
