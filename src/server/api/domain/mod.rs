pub mod creature;
pub mod session;

use crate::prelude::*;

#[allow(dead_code)]
pub trait Record: Sized {
    const TABLE: &'static str;

    async fn find_by_id(
        conn: &sqlx::PgPool,
        id: &uuid::Uuid,
    ) -> CrateResult<Option<Self>>;

    async fn save(
        &self,
        conn: &sqlx::PgPool,
    ) -> CrateResult<()>;

    async fn delete(
        self,
        conn: &sqlx::PgPool,
    ) -> CrateResult<()>;
}
