use actix_web::HttpResponse;
use actix_web::Responder;
use actix_web::delete;
use actix_web::get;
use actix_web::post;
use actix_web::put;
use actix_web::web;
use uuid::Uuid;

use crate::AppState;
use crate::dto::Dto;
use crate::dto::session::FullSessionDto;
use crate::dto::session::SessionDto;
use crate::records::Record;
use crate::records::session::SessionRecord;

pub(crate) fn make_session_resource() -> actix_web::Scope {
    web::scope("/session")
        .service(create_session)
        .service(get_session)
        .service(update_session)
        .service(delete_session)
}

#[post("/")]
async fn create_session(state: web::Data<AppState>) -> HttpResponse {
    let session_record = SessionRecord::new();
    match session_record.save(&state.pool).await {
        Ok(_) => {}
        Err(_) => {
            return HttpResponse::InternalServerError().body("An unexpected error occurred.");
        }
    }
    let session_dto = FullSessionDto::from_record(&session_record);

    session_dto.to_response()
}

#[get("/{session_id}/")]
async fn get_session(state: web::Data<AppState>, session_id: web::Path<String>) -> impl Responder {
    let parse = Uuid::parse_str(&session_id);
    let uuid = match parse {
        Ok(uuid) => uuid,
        Err(_) => {
            return HttpResponse::BadRequest().body("invalid session_id");
        }
    };
    let session_record = match SessionRecord::find_by_id(&state.pool, &uuid).await {
        Ok(Some(session_record)) => session_record,
        _ => return HttpResponse::NotFound().body(""),
    };

    let session_dto = SessionDto::from_record(&session_record);

    session_dto.to_response()
}

#[put("/{session_id}/")]
async fn update_session(session_id: web::Path<String>) -> impl Responder {
    let _ = session_id;
    HttpResponse::ImATeapot().body("todo")
}

#[delete("/{session_id}/")]
async fn delete_session(session_id: web::Path<String>) -> impl Responder {
    let _ = session_id;
    HttpResponse::ImATeapot().body("todo")
}
