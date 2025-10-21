use poem_openapi::Object;

use crate::domain::entity::session::SessionEntity;
use crate::server::api::view::FromEntity;
use crate::server::api::view::View;

#[derive(Object, serde::Serialize, Clone, Debug)]
pub struct SessionWithoutSecretView {
    pub session_id: String,
}
impl View for SessionWithoutSecretView {}
impl FromEntity<SessionEntity> for SessionWithoutSecretView {
    fn from_entity(entity: &SessionEntity) -> Self {
        let rpghp_session_id = format!("{}", entity.rpghp_session_id);
        Self {
            session_id: rpghp_session_id,
        }
    }
}

#[derive(Object, serde::Serialize, Clone, Debug)]
pub struct SessionWithSecretView {
    pub session_id: String,
    pub secret: String,
}
impl View for SessionWithSecretView {}
impl FromEntity<SessionEntity> for SessionWithSecretView {
    fn from_entity(session: &SessionEntity) -> Self {
        let rpghp_session_id = format!("{}", session.rpghp_session_id);
        let secret = format!("{}", session.secret);
        Self {
            session_id: rpghp_session_id,
            secret,
        }
    }
}
