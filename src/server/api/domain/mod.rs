use crate::prelude::*;

use sqlx::PgPool;
use sqlx::types::Uuid;

pub mod creature_record;
pub mod session_record;

#[allow(dead_code)]
pub trait Record: Sized {
    const TABLE: &'static str;

    async fn find_by_id(
        conn: &PgPool,
        id: &Uuid,
    ) -> CrateResult<Option<Self>>;
    async fn save(
        &self,
        conn: &PgPool,
    ) -> CrateResult<()>;
    async fn delete(
        self,
        conn: &PgPool,
    ) -> CrateResult<()>;
}
