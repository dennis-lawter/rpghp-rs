use actix_web::HttpResponse;
use actix_web::Responder;
use actix_web::delete;
use actix_web::get;
use actix_web::post;
use actix_web::put;
use actix_web::web;

use crate::AppState;
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
async fn create_session(state: web::Data<AppState>) -> impl Responder {
    let session_record = SessionRecord::new();
    match session_record.save(&state.pool).await {
        Ok(_) => {}
        Err(_) => {
            return HttpResponse::InternalServerError().body("An unexpected error occurred.");
        }
    }
    let session_record_body = serde_json::json!(session_record);
    let session_str = format!("{}", session_record_body);
    HttpResponse::Created().body(session_str)
}

#[get("/{id}")]
async fn get_session(id: web::Path<String>) -> impl Responder {
    format!("Get session {}", id)
}

#[put("/{id}")]
async fn update_session(id: web::Path<String>) -> impl Responder {
    format!("Update session {}", id)
}

#[delete("/{id}")]
async fn delete_session(id: web::Path<String>) -> impl Responder {
    format!("Delete session {}", id)
}
