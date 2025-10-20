use poem_openapi::Object;

use crate::domain::entity::session::SessionEntity;
use crate::server::api::view::View;

#[derive(Object, serde::Serialize, Clone, Debug)]
pub struct SessionView {
    pub rpghp_session_id: String,
}
impl View<SessionEntity> for SessionView {
    fn from_entity(record: &SessionEntity) -> Self {
        let rpghp_session_id = format!("{}", record.id);
        Self { rpghp_session_id }
    }
}

#[derive(Object, serde::Serialize, Clone, Debug)]
pub struct SessionWithSecretView {
    pub rpghp_session_id: String,
    pub secret: String,
}
impl View<SessionEntity> for SessionWithSecretView {
    fn from_entity(session: &SessionEntity) -> Self {
        let rpghp_session_id = format!("{}", session.id);
        let secret = format!("{}", session.secret);
        Self {
            rpghp_session_id,
            secret,
        }
    }
}
