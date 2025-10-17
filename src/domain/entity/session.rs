use uuid::Uuid;

use crate::domain::entity::Entity;

#[derive(Debug, Clone)]
pub struct Session {
    pub id: Uuid,
    pub secret: Uuid,
}
impl Entity for Session {}
impl Session {
    pub fn new() -> Self {
        Self {
            id: Uuid::new_v4(),
            secret: Uuid::new_v4(),
        }
    }
}
