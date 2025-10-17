use uuid::Uuid;

use crate::domain::entity::Entity;

#[derive(Debug, Clone)]
pub struct Creature {
    pub id: Uuid,
    pub session_id: Uuid,
    pub name: String,
    pub max_hp: i32,
    pub curr_hp: i32,
    pub hp_hidden: bool,
    pub icon: Option<String>,
}
impl Entity for Creature {}
impl Creature {
    pub fn new(
        session_id: Uuid,
        name: impl Into<String>,
        max_hp: i32,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            session_id,
            name: name.into(),
            max_hp,
            curr_hp: max_hp,
            hp_hidden: false,
            icon: None,
        }
    }
}
