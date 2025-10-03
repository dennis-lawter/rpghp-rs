#[allow(unused_imports)]
use crate::prelude::*;

use crate::domain::records::Record;

use poem_openapi::SecurityScheme;
use poem_openapi::auth::Bearer;

pub mod creature;
pub mod init_group;
pub mod session;

pub trait View<T: Record>: Sized {
    fn from_record(record: &T) -> Self;
}

#[derive(SecurityScheme)]
#[oai(ty = "bearer", key_name = "Bearer", key_in = "header")]
pub struct ApiV1AuthScheme(Bearer);

#[derive(SecurityScheme)]
#[allow(dead_code)]
pub enum ApiV1AuthSchemeOptional {
    Bearer(ApiV1AuthScheme),
    #[oai(fallback)]
    NoAuth,
}
