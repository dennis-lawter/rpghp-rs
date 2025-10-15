use poem_openapi::Object;

use crate::domain::records::session::SessionRecord;
use crate::server::api::view::View;

#[derive(Object, serde::Serialize)]
pub struct SessionView {
    pub rpghp_session_id: String,
}
impl View<SessionRecord> for SessionView {
    fn from_record(record: &SessionRecord) -> Self {
        let rpghp_session_id = format!("{}", record.rpghp_session_id);
        Self { rpghp_session_id }
    }
}

#[derive(Object, serde::Serialize)]
pub struct SessionWithSecretView {
    pub rpghp_session_id: String,
    pub secret: String,
}
impl View<SessionRecord> for SessionWithSecretView {
    fn from_record(record: &SessionRecord) -> Self {
        let rpghp_session_id = format!("{}", record.rpghp_session_id);
        let secret = format!("{}", record.secret);
        Self {
            rpghp_session_id,
            secret,
        }
    }
}
