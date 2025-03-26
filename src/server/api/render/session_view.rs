use poem_openapi::ApiResponse;
use poem_openapi::Object;
use poem_openapi::payload::Json;

use crate::server::api::domain::session_record::SessionRecord;

use super::View;

#[derive(ApiResponse)]
pub enum SessionCreateResponse {
    #[oai(status = 200)]
    Ok(Json<SessionWithSecretView>),

    #[oai(status = 404)]
    NotFound,
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

#[derive(ApiResponse)]
pub enum SessionGetResponse {
    #[oai(status = 200)]
    Ok(Json<SessionView>),

    #[oai(status = 404)]
    NotFound,
}
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

#[derive(ApiResponse)]
pub enum SessionDeleteResponse {
    #[oai(status = 200)]
    Ok,

    #[oai(status = 404)]
    NotFound,
}
