use uuid::Uuid;

use crate::domain::entity::Entity;

pub struct SessionEntity {
    pub id: Uuid,
    pub secret: Uuid,
}
impl Entity for SessionEntity {}
impl SessionEntity {
    pub fn new() -> Self {
        Self {
            id: Uuid::new_v4(),
            secret: Uuid::new_v4(),
        }
    }
}
