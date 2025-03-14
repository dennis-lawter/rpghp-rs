use actix_web::HttpResponse;
use uuid::Uuid;

pub(crate) mod session;

fn try_parsing_uuid(id: &str) -> Result<Uuid, HttpResponse> {
    let parse = Uuid::parse_str(id);
    parse.map_err(|_| HttpResponse::NotFound().body("")) // Invalid format reports as 404
    // parse.map_err(|_| HttpResponse::BadRequest().body("invalid id format"))
}
