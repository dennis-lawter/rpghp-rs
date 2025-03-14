use actix_web::HttpResponse;

use crate::records::session::SessionRecord;

use super::Dto;

#[derive(serde::Serialize)]
pub(crate) struct SessionDto {
    pub(crate) rpghp_session_id: String,
}
impl Dto<SessionRecord> for SessionDto {
    fn from_record(record: &SessionRecord) -> Self {
        let rpghp_session_id = format!("{}", record.rpghp_session_id);
        Self { rpghp_session_id }
    }

    fn to_response(&self) -> HttpResponse {
        match serde_json::to_string_pretty(self) {
            Ok(session_str) => HttpResponse::Ok().body(session_str),
            _ => HttpResponse::InternalServerError().body("An unexpected error occurred."),
        }
    }
}

#[derive(serde::Serialize)]
pub(crate) struct FullSessionDto {
    pub(crate) rpghp_session_id: String,
    pub(crate) secret: String,
}
impl Dto<SessionRecord> for FullSessionDto {
    fn from_record(record: &SessionRecord) -> Self {
        let rpghp_session_id = format!("{}", record.rpghp_session_id);
        let secret = format!("{}", record.secret);
        Self {
            rpghp_session_id,
            secret,
        }
    }

    fn to_response(&self) -> HttpResponse {
        match serde_json::to_string_pretty(self) {
            Ok(session_str) => HttpResponse::Ok().body(session_str),
            _ => HttpResponse::InternalServerError().body("An unexpected error occurred."),
        }
    }
}
