use actix_web::HttpResponse;

use crate::records::Record;

pub(crate) mod session;

pub(crate) trait Dto<T: Record>: Sized {
    fn from_record(record: &T) -> Self;
    fn to_response(&self) -> HttpResponse;
}
