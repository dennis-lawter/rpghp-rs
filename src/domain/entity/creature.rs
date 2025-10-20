use uuid::Uuid;

use crate::domain::entity::Entity;

pub struct CreatureEntity {
    pub id: Uuid,
    pub session_id: Uuid,
    pub creature_name: String,
    pub max_hp: i32,
    pub curr_hp: i32,
    pub hp_hidden: bool,
    pub icon: Option<String>,
}
impl Entity for CreatureEntity {}
