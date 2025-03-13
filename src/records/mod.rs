use crate::prelude::*;

use sqlx::PgPool;
use sqlx::types::Uuid;

pub(crate) mod session;

#[allow(dead_code)]
pub trait Record: Sized {
    const TABLE: &'static str;

    async fn find_by_id(conn: &PgPool, id: &Uuid) -> CrateResult<Option<Self>>;
    async fn save(&self, conn: &PgPool) -> CrateResult<()>;
}
