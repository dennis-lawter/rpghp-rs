use poem_openapi::Object;

use crate::domain::entity::session::Session;
use crate::server::api::view::View;

#[derive(Object, serde::Serialize, Clone, Debug)]
pub struct SessionView {
    pub rpghp_session_id: String,
}
impl View<Session> for SessionView {
    fn from_entity(session: &Session) -> Self {
        let rpghp_session_id = format!("{}", session.id);
        Self { rpghp_session_id }
    }
}

#[derive(Object, serde::Serialize, Clone, Debug)]
pub struct SessionWithSecretView {
    pub rpghp_session_id: String,
    pub secret: String,
}
impl View<Session> for SessionWithSecretView {
    fn from_entity(session: &Session) -> Self {
        let rpghp_session_id = format!("{}", session.id);
        let secret = format!("{}", session.secret);
        Self {
            rpghp_session_id,
            secret,
        }
    }
}
