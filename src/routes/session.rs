use actix_web::HttpResponse;
use actix_web::Responder;
use actix_web::delete;
use actix_web::get;
use actix_web::post;
use actix_web::web;

use crate::AppState;
use crate::dto::Dto;
use crate::dto::session::SessionDto;
use crate::dto::session::SessionWithSecretDto;
use crate::records::Record;
use crate::records::session::SessionRecord;
use crate::routes::try_parsing_uuid;

pub(crate) fn make_session_resource() -> actix_web::Scope {
    web::scope("/session")
        .service(create_session)
        .service(get_session)
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
    let session_dto = SessionWithSecretDto::from_record(&session_record);

    session_dto.to_response()
}

#[get("/{session_id}/")]
async fn get_session(state: web::Data<AppState>, session_id: web::Path<String>) -> impl Responder {
    let uuid = match try_parsing_uuid(&session_id) {
        Ok(uuid) => uuid,
        Err(response) => return response,
    };

    // Get by id or secret

    // let session_record = match SessionRecord::find_by_id(&state.pool, &uuid).await {
    //     Ok(Some(session_record)) => session_record,
    //     _ => match SessionRecord::find_by_secret(&state.pool, &uuid).await {
    //         Ok(Some(session_record)) => session_record,
    //         _ => return HttpResponse::NotFound().body(""),
    //     },
    // };

    // let session_record = match (
    //     SessionRecord::find_by_id(&state.pool, &uuid).await,
    //     SessionRecord::find_by_secret(&state.pool, &uuid).await,
    // ) {
    //     (Ok(Some(session_record)), _) => session_record,
    //     (_, Ok(Some(session_record))) => session_record,
    //     _ => return HttpResponse::NotFound().body(""),
    // };

    let session_record = match SessionRecord::find_by_secret_or_id(&state.pool, &uuid).await {
        Ok(Some(session_record)) => session_record,
        _ => return HttpResponse::NotFound().body(""),
    };

    let session_dto = SessionDto::from_record(&session_record);

    session_dto.to_response()
}

#[delete("/{session_id}/")]
async fn delete_session(
    state: web::Data<AppState>,
    session_id: web::Path<String>,
) -> impl Responder {
    let uuid = match try_parsing_uuid(&session_id) {
        Ok(uuid) => uuid,
        Err(response) => return response,
    };

    let session_record = match SessionRecord::find_by_secret(&state.pool, &uuid).await {
        Ok(Some(session_record)) => session_record,
        _ => return HttpResponse::NotFound().body(""),
    };

    match session_record.delete(&state.pool).await {
        Ok(()) => HttpResponse::Ok().body("Record deleted"),
        Err(_) => HttpResponse::InternalServerError().body("An unexpected error occurred."),
    }
}
