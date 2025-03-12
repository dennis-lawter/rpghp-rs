use crate::prelude::*;

use sqlx::PgConnection;
use sqlx::types::Uuid;

pub(crate) mod session;

#[allow(dead_code)]
pub trait Record: Sized {
    const TABLE: &'static str;

    async fn find_by_id(conn: &mut PgConnection, id: &Uuid) -> CrateResult<Option<Self>>;
}
