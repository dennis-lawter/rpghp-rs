use uuid::Uuid;

use crate::domain::entity::Entity;

pub struct SessionEntity {
    pub rpghp_session_id: Uuid,
    pub secret: Uuid,
}
impl Entity for SessionEntity {}
impl SessionEntity {
    pub fn new() -> Self {
        Self {
            rpghp_session_id: Uuid::new_v4(),
            secret: Uuid::new_v4(),
        }
    }
}
