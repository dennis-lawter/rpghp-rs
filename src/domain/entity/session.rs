use uuid::Uuid;

use crate::domain::entity::Entity;

pub struct SessionEntity {
    pub rpghp_session_id: Uuid,
    pub secret: Uuid,
}
impl Entity for SessionEntity {}
impl SessionEntity {
    pub fn new() -> Self {
        let rpghp_session_id = Uuid::new_v4();
        let secret = Uuid::new_v4();
        Self {
            rpghp_session_id,
            secret,
        }
    }
}
