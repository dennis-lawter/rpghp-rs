use poem_openapi::SecurityScheme;
use poem_openapi::auth::Bearer;

use super::domain::Record;

pub mod creature;
pub mod session;

pub trait View<T: Record>: Sized {
    fn from_record(record: &T) -> Self;
}

#[derive(SecurityScheme)]
#[oai(ty = "bearer", key_name = "Bearer", key_in = "header")]
#[allow(dead_code)]
pub struct ApiV1AuthScheme(Bearer);
